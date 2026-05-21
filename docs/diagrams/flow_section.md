# Chitragupt — Project Flow Documentation

> **Who this is for:** Anyone new to Chitragupt who wants to understand how the system works without reading all the technical specs.
> Each section covers one module — what it does, a simple diagram, and what goes in and comes out.

---

## Table of Contents

1. [System Overview](#1-system-overview)
2. [BA Session State Machine](#2-ba-session-state-machine)
3. [Per-Turn LLM Pipeline](#3-per-turn-llm-pipeline)
4. [Document Ingestion Pipeline](#4-document-ingestion-pipeline)
5. [RAG Retrieval Flow](#5-rag-retrieval-flow)
6. [Upload Gate System](#6-upload-gate-system)
7. [Trust Hierarchy and Confidence Scoring](#7-trust-hierarchy-and-confidence-scoring)
8. [Conflict Detection and Resolution](#8-conflict-detection-and-resolution)
9. [BRD and HLD Generation](#9-brd-and-hld-generation)
10. [Client Sign-Off and Spec Lock](#10-client-sign-off-and-spec-lock)
11. [Complete High-Level Flow](#11-complete-high-level-flow)

---

## 1. System Overview

Chitragupt is an AI-powered Business Requirement Analyzer. A Business Analyst (BA) opens a chat session and is guided through seven structured phases — from describing a business problem to producing a signed-off Business Requirements Document (BRD) and High-Level Architecture Diagram (HLD).

**Important components involved:**
- BA Chat Interface — the only way the BA interacts with the system
- LangGraph State Machine — tracks which phase the session is in
- LLM Agent Pipeline — the reasoning engine behind every response
- RAG Pipeline — retrieves relevant content from uploaded documents
- PostgreSQL + pgvector — stores all session data, entities, and embeddings
- Export Engine — generates the final BRD and HLD artifacts

**Input → Process → Output:**
- **Input:** BA messages and document uploads
- **Process:** Structured, phase-by-phase guided conversation with AI extraction
- **Output:** Signed-off BRD (DOCX/PDF) and HLD diagram (Mermaid/PNG)

```mermaid
flowchart TD
    %% Top-level system view — what comes in, what happens, what comes out

    BA([Business Analyst]) --> CHAT[Chat Interface]
    CHAT --> SM[Session State Machine\n7-phase HITL graph]
    SM --> PIPELINE[LLM Agent Pipeline\nIntent · Extract · Retrieve · Guide]
    PIPELINE --> RAG[RAG Pipeline\nHybrid semantic and keyword search]
    RAG --> DB[(PostgreSQL + pgvector\nSession state · Entities · Chunks)]
    PIPELINE --> EXPORT[Export Engine\nBRD and HLD generation]
    EXPORT --> BRD[BRD Document\nDOCX and PDF]
    EXPORT --> HLD[HLD Diagram\nMermaid and PNG]
    BRD --> SIGNOFF[Client Sign-Off\nToken-based approval]
    HLD --> SIGNOFF
    SIGNOFF --> LOCKED([Locked Specification\nAudit record created])
```

---

## 2. BA Session State Machine

The session advances through seven phases in order. The BA can never be moved forward automatically — every transition requires the BA to explicitly confirm. The BA can also revisit any prior phase at any time without losing what was already captured.

**Important components involved:**
- `SessionState` — the typed object that holds all captured data
- `TransitionHandler` — proposes transitions and waits for BA confirmation
- Acceptance Criteria (AC) — discrete, checkable conditions that must all pass before a transition is offered
- REVISIT handler — re-enters a prior state and resumes from the last unmet criterion

**Input → Process → Output:**
- **Input:** BA natural language messages and confirmation responses
- **Process:** AC evaluation after every turn; transition offer only when all AC pass
- **Output:** Session advances to the next state; all captured data is persisted

```mermaid
flowchart TD
    %% Seven phases of the BA journey — transitions require explicit BA confirmation

    START([New Session Created]) --> S1[PROBLEM_INTAKE\nDescribe the business problem]
    S1 -->|All AC-S1 met\nBA confirms| S2[STAKEHOLDER_DISCOVERY\nIdentify actors and roles]
    S2 -->|All AC-S2 met\nBA confirms| S3[REQUIREMENT_ELICITATION\nMap what each actor needs]
    S3 -->|All AC-S3 met\nBA confirms| S4[CONSTRAINT_CAPTURE\nBudget · timeline · compliance]
    S4 -->|All AC-S4 met\nBA confirms| S5[ARCHITECTURE_ALIGNMENT\nGuide technology decisions]
    S5 -->|All AC-S5 met\nBA confirms| S6[REVIEW_AND_SIGN_OFF\nReview draft BRD and HLD]

    %% BA can request revision and return to requirement phase
    S6 -->|BA requests revision| S3

    S6 -->|All AC-S6 met\nBA approves| S7[SIGNED_OFF\nLocked and exported]
    S7 --> END([Session Complete])
```

---

## 3. Per-Turn LLM Pipeline

Every message the BA sends triggers a six-step pipeline before a response is generated. No step can be skipped. The final output always ends with exactly one clear action for the BA — a question, a confirmation prompt, or a transition offer. The BA should never wonder "what do I do next?"

**Important components involved:**
- `IntentClassifierAgent` — Fast LLM, classifies what the BA meant (target: < 200ms)
- `EntityExtractorAgent` — Standard LLM, pulls structured data from the BA's words
- `RAGRetrievalAgent` — Hybrid search over all uploaded documents in the session
- `GapAnalyzerAgent` — Standard LLM, checks which acceptance criteria are still unmet
- `GuidanceGeneratorAgent` — Premium LLM, composes the final response (streamed)

**Input → Process → Output:**
- **Input:** Any BA message (answer, confirmation, correction, upload signal, revisit request, skip)
- **Process:** Intent classification → entity handling → retrieval → gap check → response generation
- **Output:** Streamed system message: acknowledge + captured bullets + one next action

```mermaid
flowchart TD
    %% Six-step pipeline — runs on every BA message without exception

    MSG([BA Message]) --> INTENT[Step 1: Classify Intent\nFast LLM · target under 200ms]

    INTENT --> BRANCH{Intent type?}

    %% Each intent type routes to its own handler
    BRANCH -->|ANSWER| EXTRACT[Step 2a: Extract Entities\nStandard LLM · state-specific schema]
    BRANCH -->|DENY or CORRECTION| CORRECT[Step 2b: Apply Correction\nLocate entity · update session state]
    BRANCH -->|UPLOAD_SIGNAL| INGEST[Step 2c: Trigger Ingestion\nChunk · Embed · Index document]
    BRANCH -->|SKIP| SKIPLOG[Step 2d: Log Open Question\nMark priority level]
    BRANCH -->|REVISIT| ROLLBACK[Step 2e: Re-enter Prior State\nPresent captured summary]
    BRANCH -->|CONFIRM| VALIDATE[Step 2f: Check Transition Readiness\nEvaluate all AC for current state]

    %% Main branches feed into retrieval
    EXTRACT --> RAG[Step 3: Retrieve Context\nHybrid search · up to 15 chunks · score floor 0.65]
    CORRECT --> RAG
    INGEST --> RAG
    SKIPLOG --> RAG

    %% Rollback bypasses retrieval and goes straight to guidance
    ROLLBACK --> GUIDE[Step 5: Generate Guidance\nPremium LLM · streamed response]

    %% Retrieval and validation both feed gap analysis
    RAG --> GAP[Step 4: Gap Analysis\nWhich AC is still unmet?\nSelect highest-priority gap]
    VALIDATE --> GAP

    GAP --> GUIDE

    GUIDE --> READY{All AC met\nfor this state?}
    READY -->|No| QUESTION([Emit: one next question])
    READY -->|Yes| OFFER([Emit: transition offer\nwith summary])
```

---

## 4. Document Ingestion Pipeline

When a BA uploads a document at any checkpoint, it passes through a processing pipeline before its content becomes searchable. PII scrubbing always happens before embedding — never after. Every chunk is tagged with the tenant and project IDs to enforce data isolation.

**Important components involved:**
- Object Storage — raw files stored at `s3://{tenant_id}/{project_id}/`
- PII Scrubber — redacts personal identifiers before any embedding call
- Chunker — splits documents into 512-token overlapping windows
- Embedding Model — converts text to dense 1536-dimension vectors
- pgvector — stores chunks with both dense and sparse (BM25) vectors
- `UPLOAD_COMPLETE` event — fires after successful indexing; triggers AC re-evaluation

**Input → Process → Output:**
- **Input:** Raw uploaded file (PDF, DOCX, image, spreadsheet, audio, etc.)
- **Process:** Store → Scrub PII → Chunk → Embed → Index → Notify
- **Output:** Searchable chunks available for retrieval; upload acceptance criteria re-evaluated

```mermaid
flowchart TD
    %% Ingestion pipeline — PII scrub always runs before embedding

    UPLOAD([Document Uploaded\nat Checkpoint A B C or D]) --> STORE[Store Raw File\nObject Storage: s3 tenant and project path]
    STORE --> META[Record Document Metadata\nPostgreSQL · status = processing]
    META --> PII[PII Scrub\nRedact names · emails · IDs]
    PII --> CHUNK[Split into Chunks\n512-token windows with overlap]
    CHUNK --> EMBED[Generate Dense Embeddings\nEmbedding model · 1536 dimensions]
    EMBED --> SPARSE[Generate Sparse Vectors\nBM25 for keyword search]
    SPARSE --> INDEX[Index to pgvector\nWith tenant · project · trust tier metadata]
    INDEX --> STATUS[Update Document Status\nstatus = indexed]
    STATUS --> EVENT([Fire UPLOAD_COMPLETE Event])
    EVENT --> REEVAL[GapAnalyzer Re-evaluates\nAll upload AC for current state]
```

---

## 5. RAG Retrieval Flow

From the STAKEHOLDER_DISCOVERY phase onward, every turn includes a retrieval step. The system builds a query from the current session state and extracted entities, runs it against both dense and sparse indexes, fuses the results, filters by mandatory criteria, and re-ranks before returning chunks to the pipeline.

**Important components involved:**
- State-specific query templates — different query patterns per session phase
- Dense search — vector cosine similarity against `pgvector`
- Sparse search — BM25 keyword matching
- RRF fusion — Reciprocal Rank Fusion combines both result sets
- Cross-encoder re-ranker — orders final results by relevance
- Mandatory filters — `tenant_id`, `project_id`, `is_active`, `valid_until`

**Input → Process → Output:**
- **Input:** Current session state + entities extracted this turn
- **Process:** Build query → dual-path search → fuse → filter → floor → re-rank
- **Output:** Up to 15 ranked chunks with source metadata, passed to GapAnalyzer and GuidanceGenerator

```mermaid
flowchart TD
    %% Hybrid retrieval — both dense and sparse paths run in parallel then merge

    STATE[Session State + Extracted Entities] --> QUERY[Build State-Specific Query\nTemplate varies per phase]

    %% Two search paths run simultaneously
    QUERY --> DENSE[Dense Search\nVector similarity · cosine distance]
    QUERY --> SPARSE2[Sparse Search\nBM25 keyword matching]

    DENSE --> FUSE[Fuse Results\nReciprocal Rank Fusion]
    SPARSE2 --> FUSE

    FUSE --> FILTER[Apply Mandatory Filters\ntenant · project · active · not expired]
    FILTER --> FLOOR[Score Floor: 0.65\nDiscard low-confidence chunks]
    FLOOR --> RERANK[Cross-Encoder Re-ranking\nTop 15 chunks selected]
    RERANK --> CHUNKS([Return Ranked Chunks\nwith trust tier and confidence metadata])
```

**State-specific query patterns:**

| Phase | Query template |
|---|---|
| STAKEHOLDER_DISCOVERY | `{problem_statement} stakeholders actors roles responsibilities` |
| REQUIREMENT_ELICITATION | `{actor_name} needs requirements workflows` — one query per actor |
| CONSTRAINT_CAPTURE | `budget timeline compliance regulatory constraints {domain}` |
| ARCHITECTURE_ALIGNMENT | `{decision_id} {constraint_summary} {compliance_flags}` |
| REVIEW_AND_SIGN_OFF | `{requirement_description} {source_chunk_ids}` — re-retrieval for BRD grounding |

---

## 6. Upload Gate System

Each state transition has upload checkpoints enforced by the `GapAnalyzerAgent`. Gates are classified into four types. The gate evaluation runs before conversational AC — no transition offer can be made while a gate is open.

**Important components involved:**
- Upload AC Registry — 16 named upload rules across all six transitions
- Checkpoint prompts A, B, C, D — pre-written templates issued at designated transitions
- REQUIRED PROMPT tracker — records whether each checkpoint prompt has been issued
- TRIGGERED detector — scans BA messages for references to existing documents
- HARD GATE evaluator — blocks the transition offer when a hard condition is unmet
- Waiver recorder — stores explicit BA skips (`memory_only_waiver`, `compliance_doc_waiver`)

**Input → Process → Output:**
- **Input:** End of every pipeline turn
- **Process:** Check hard gates → check required prompts → check triggered conditions → then conversational AC
- **Output:** Correct upload prompt inserted into guidance, or transition offer unblocked

```mermaid
flowchart TD
    %% Upload gate evaluation runs before conversational AC every turn

    TURN([End of Pipeline Turn]) --> HARD{Is a Hard Gate\nopen and unresolvable?}
    HARD -->|Yes| BLOCK[Block Transition Offer\nSurface specific resolution action]
    HARD -->|No| REQPROMPT{Has a Required Prompt\nnot been issued yet?}

    REQPROMPT -->|Yes| ISSUE[Issue Checkpoint Prompt\nA · B · C or D]
    REQPROMPT -->|No| TRIG{Has a Triggered condition\nfired but not resolved?}

    TRIG -->|Yes| TRIGMSG[Insert Upload Prompt\ninto Guidance response]
    TRIG -->|No| CONVAC[Evaluate Conversational AC\nGapAnalyzer proceeds normally]

    BLOCK --> WAIT([Await BA action])
    ISSUE --> WAIT
    TRIGMSG --> CONVAC
    CONVAC --> GUIDE([Generate Guidance Response])
```

**Gate type reference:**

| Type | Behavior | Can BA skip? |
|---|---|---|
| HARD GATE | Blocks transition — unresolvable without action | No |
| REQUIRED PROMPT | Must be issued before transition offer | Yes — system records skip |
| TRIGGERED | Fires when BA references an existing document | Yes — confidence metadata adjusted |
| RECOMMENDED | Offered once | Yes — no impact |

---

## 7. Trust Hierarchy and Confidence Scoring

Every piece of extracted knowledge is tagged with two values: a **trust tier** (how reliable the source is) and a **confidence score** (how certain the extraction is). When two sources conflict, the higher trust tier always wins. Claims below a confidence threshold appear with visible tags in the final BRD.

**Important components involved:**
- Trust tiers 1–5 — assigned per source type at ingestion time
- Confidence score (0.0–1.0) — computed by the extraction agent
- Confidence tags — `[SYNTHESIZED]`, `[INFERRED — VERIFY]` — visible in the BRD output
- Orphan Knowledge rule — any LLM claim with no matching chunk is raised as an Open Question, never included as a confirmed requirement

**Input → Process → Output:**
- **Input:** A claim produced by any agent, with its source document
- **Process:** Assign tier → score → tag → include or raise as open question
- **Output:** Tagged requirement or open question, stored in session state

```mermaid
flowchart TD
    %% Every claim gets a trust tier from its source before scoring

    CLAIM[Incoming Claim] --> TIER{Assign Trust Tier\nbased on source type}

    TIER -->|Rank 1| T1[Human Override\nBA edits · approvals · rejections]
    TIER -->|Rank 2| T2[Structured External Systems\nJira fields · typed API contracts]
    TIER -->|Rank 3| T3[Primary Source Documents\nClient PDFs · signed pages]
    TIER -->|Rank 4| T4[Secondary Documents\nMeeting notes · Slack threads]
    TIER -->|Rank 5| T5[Agentic Inference\nLLM synthesis · industry assumptions]

    T1 & T2 & T3 & T4 & T5 --> SCORE[Assign Confidence Score\n0.0 to 1.0]

    SCORE --> RANGE{Score range?}
    RANGE -->|0.85 to 1.00| EXPLICIT[No tag — Explicit extraction\nInclude in spec as-is]
    RANGE -->|0.65 to 0.84| SYNTH[Tag: SYNTHESIZED\nInclude with tag visible]
    RANGE -->|0.40 to 0.64| INFER[Tag: INFERRED — VERIFY\nInclude with tag visible]
    RANGE -->|Below 0.40| DROP[Raise as Open Question\nDo not include in spec]
```

---

## 8. Conflict Detection and Resolution

When two retrieved chunks from sources of equal trust tier contradict each other on the same topic, the system halts synthesis and raises a Conflict object. It never guesses which source is correct. The BA must resolve it before the affected requirements can be finalized.

**Important components involved:**
- `Conflict` entity — stores both source citations and the affected requirements
- Conflict status — `open`, `resolved_a`, `resolved_b`, `resolved_manual`, `escalated`
- HITL authority — the BA's resolution becomes Rank 1 (human override) truth for that topic
- Conflict flag in BRD — `[CONFLICT — PENDING RESOLUTION]` tag visible until resolved

**Input → Process → Output:**
- **Input:** Two retrieved chunks from same-tier sources that contradict on a topic
- **Process:** Create Conflict object → flag → present to BA → apply resolution
- **Output:** BA resolution recorded as Rank 1 truth; synthesis resumes

```mermaid
flowchart TD
    %% Conflicts halt synthesis until a human resolves them — never auto-resolved

    CHUNKS2([Retrieved Chunks]) --> COMPARE[Compare Claims\nSame topic detected]
    COMPARE --> FOUND{Contradiction\nfound?}

    FOUND -->|No| PROCEED[Continue synthesis\nnormally]
    FOUND -->|Yes| TIERS{Same trust\ntier?}

    %% Different tiers: higher rank wins automatically
    TIERS -->|No — different tiers| WIN[Higher tier wins automatically\nLower claim discarded]

    %% Equal tiers: halt and raise conflict
    TIERS -->|Yes — equal tiers| HALT[Halt synthesis on this topic]
    HALT --> OBJECT[Create Conflict Object\nCite both sources]
    OBJECT --> FLAG[Flag affected section\nCONFLICT — PENDING RESOLUTION]
    FLAG --> PRESENT[Present to BA\nSource A vs Source B side by side]

    PRESENT --> RESOLVE{BA decision}
    RESOLVE -->|Accept A| RA[Record Source A as truth\nRank 1 override · resume synthesis]
    RESOLVE -->|Accept B| RB[Record Source B as truth\nRank 1 override · resume synthesis]
    RESOLVE -->|Provide clarification| RM[Record manual resolution\nresume synthesis]

    WIN --> PROCEED
    RA & RB & RM --> PROCEED
```

---

## 9. BRD and HLD Generation

When the session reaches REVIEW_AND_SIGN_OFF, two artifacts are generated in parallel: a structured Business Requirements Document assembled from all confirmed requirements, and a High-Level Architecture Diagram derived from the identified actors, integrations, and system components. The BA reviews both in the chat interface and can request targeted revisions — approved sections are never regenerated.

**Important components involved:**
- `BRDWriterAgent` — Premium LLM, applies the domain BRD template to confirmed requirements
- `HLDGeneratorAgent` — derives component diagram from actors, integrations, and decisions
- `ExportArtifact` records — `brd_docx`, `brd_pdf`, `hld_mermaid`, `hld_png` with `file_hash` and `storage_uri`
- HITL authority — BA section approvals are immutable; only flagged sections are revised

**Input → Process → Output:**
- **Input:** All confirmed requirements, constraints, actors, and decision directions from the session
- **Process:** Generate BRD + HLD → persist → present → BA reviews → approve or revise
- **Output:** Approved BRD and HLD artifacts persisted to object storage; `ClientSignOff` record created

```mermaid
flowchart TD
    %% Artifacts generated once — revisions target only flagged sections

    ENTER([REVIEW_AND_SIGN_OFF entered]) --> BRDGEN[Generate Draft BRD\nPremium LLM · BRD template applied]
    ENTER --> HLDGEN[Generate HLD Diagram\nMermaid from actors · integrations · decisions]

    BRDGEN --> BRDSTORE[Persist BRD to Object Storage\nbrd_docx and brd_pdf with file hash]
    HLDGEN --> HLDSTORE[Persist HLD to Object Storage\nhld_mermaid and hld_png with file hash]

    BRDSTORE --> PRESENT2[Present BRD and HLD to BA\nvia chat interface]
    HLDSTORE --> PRESENT2

    PRESENT2 --> REVIEW2{BA review decision}

    REVIEW2 -->|Request revision\non a specific section| REVISE[Re-generate flagged section only\nApproved sections unchanged]
    REVIEW2 -->|Approve all sections| APPROVE[Mark BRD as approved\nCreate ClientSignOff record with status = pending]

    REVISE --> PRESENT2
    APPROVE --> TOKEN3[Dispatch Sign-Off Token\nvia email to client stakeholder]
```

---

## 10. Client Sign-Off and Spec Lock

After the BA approves the BRD, a single-use HMAC token is sent to the client stakeholder. When the client clicks the link and signs, the token is burned and the Specification is permanently locked — no further automated changes are possible. Any change after locking requires creating a new version.

**Important components involved:**
- `ClientSignOff` entity — tamper-evident record with `signature_token`, `signed_at`, and IP address
- HMAC token — single-use, 7-day expiry; burned on first click
- Spec lock — sets `Specification.status = locked` and `locked_at`; immutable thereafter
- Audit log — append-only record of the sign-off event; cannot be modified or deleted
- Downstream unblock — Jira, Confluence, and webhook exports are gated until `status = signed`

**Input → Process → Output:**
- **Input:** BA BRD approval
- **Process:** Generate token → send email → await client action → burn token → lock spec
- **Output:** Locked, exported specification with immutable audit record; downstream exports unblocked

```mermaid
flowchart TD
    %% Sign-off flow — spec becomes immutable only after client signature token is burned

    BAAPPROVE([BA approves BRD]) --> GENTOKEN[Generate HMAC Sign-Off Token\nSingle-use · 7-day expiry]
    GENTOKEN --> EMAIL2[Send Sign-Off Email\nto client stakeholder]

    EMAIL2 --> WAIT3{Client action}

    WAIT3 -->|Client clicks link and signs| BURN[Burn Token\nRecord signature · timestamp · IP address]
    WAIT3 -->|Client declines| DECLINE[Record declined reason\nReturn to REVIEW_AND_SIGN_OFF]
    WAIT3 -->|Token expires after 7 days| EXPIRE[Mark token expired\nResend required]

    BURN --> LOCK2[Lock Specification\nstatus = locked · locked_at set · immutable]
    LOCK2 --> EXPORT2[Generate Final Export Package\nDOCX · PDF · Markdown]
    LOCK2 --> AUDIT2[Write Immutable Audit Record\nTimestamp · BA identity · client email]
    LOCK2 --> DOWNSTREAM[Unblock Downstream Exports\nJira · Confluence · webhooks]

    EXPIRE --> EMAIL2
```

---

## 11. Complete High-Level Flow

All modules working together — from a BA opening a new session to a locked, client-signed specification ready for delivery.

```mermaid
flowchart TD
    %% End-to-end flow combining all modules

    BA3([BA starts session]) --> S1C[Phase 1: Problem Intake\nDescribe the problem in plain language]

    S1C --> LOOP[Per-Turn LLM Pipeline\nIntent · Extract · Retrieve · Gap · Guide]

    %% Document uploads can arrive at any checkpoint
    DOC2([Document Upload\nat any checkpoint]) --> INGEST3[Ingestion Pipeline\nScrub · Chunk · Embed · Index]
    INGEST3 -->|UPLOAD_COMPLETE event| LOOP

    %% Pipeline drives phase progression
    LOOP -->|Gap unmet — ask next question| LOOP
    LOOP -->|AC-S1 met + BA confirms| S2C[Phase 2: Stakeholder Discovery\nActors · roles · decision authority]

    S2C --> LOOP
    LOOP -->|AC-S2 met + BA confirms| S3C[Phase 3: Requirement Elicitation\nFunctional and non-functional requirements]

    S3C --> LOOP
    LOOP -->|AC-S3 met + BA confirms| S4C[Phase 4: Constraint Capture\nBudget · timeline · compliance · residency]

    S4C --> LOOP
    LOOP -->|AC-S4 met + BA confirms| S5C[Phase 5: Architecture Alignment\nTranslate BA answers into technology direction]

    S5C --> LOOP
    LOOP -->|AC-S5 met + BA confirms| S6C[Phase 6: Review and Sign-Off]

    S6C --> ARTGEN2[Generate BRD and HLD\nPremium LLM · domain template applied]
    ARTGEN2 --> BAREV2{BA reviews artifacts}

    BAREV2 -->|Revision requested| S6C
    BAREV2 -->|BA approves| SIGNFLOW2[Client Sign-Off Flow\nToken · Email · Client signature]

    SIGNFLOW2 --> LOCKED2([Specification Locked\nExported · Audited · Downstream unblocked])
```

---

> Chitragupt Flow Documentation · Sprint 0 and Sprint 1 · May 2026
