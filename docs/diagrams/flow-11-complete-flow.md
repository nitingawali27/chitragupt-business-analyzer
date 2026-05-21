# Flow 11 — Complete High-Level Flow

## What this covers

This is the full end-to-end picture — all modules working together, from a BA opening a new session to a locked, client-signed specification ready for delivery.

Each numbered flow document covers one module in depth. This diagram shows how they connect.

## Module Map

| Flow | Module |
|---|---|
| [Flow 01](flow-01-system-overview.md) | System Overview |
| [Flow 02](flow-02-session-state-machine.md) | BA Session State Machine |
| [Flow 03](flow-03-per-turn-pipeline.md) | Per-Turn LLM Pipeline |
| [Flow 04](flow-04-document-ingestion.md) | Document Ingestion Pipeline |
| [Flow 05](flow-05-rag-retrieval.md) | RAG Retrieval Flow |
| [Flow 06](flow-06-upload-gates.md) | Upload Gate System |
| [Flow 07](flow-07-trust-hierarchy.md) | Trust Hierarchy and Confidence Scoring |
| [Flow 08](flow-08-conflict-resolution.md) | Conflict Detection and Resolution |
| [Flow 09](flow-09-brd-hld-generation.md) | BRD and HLD Generation |
| [Flow 10](flow-10-client-signoff.md) | Client Sign-Off and Spec Lock |

## Input → Process → Output

- **Input:** A BA with a business problem and (optionally) a set of documents
- **Process:** Seven guided phases with AI extraction, retrieval, gap analysis, and conflict resolution
- **Output:** Locked, client-signed BRD and HLD exported in DOCX, PDF, and Markdown formats

## Diagram

```mermaid
flowchart TD
    %% Complete end-to-end — from session start to locked specification

    BA3([BA starts session]) --> S1C[Phase 1: Problem Intake\nDescribe the problem in plain language]

    S1C --> LOOP[Per-Turn LLM Pipeline\nIntent · Extract · Retrieve · Gap · Guide]

    %% Document uploads feed the ingestion pipeline at any checkpoint
    DOC2([Document Upload\nat Checkpoint A B C or D]) --> INGEST3[Ingestion Pipeline\nScrub · Chunk · Embed · Index]
    INGEST3 -->|UPLOAD_COMPLETE event| LOOP

    %% Pipeline loops until all AC are met for each phase
    LOOP -->|Gap unmet · ask next question| LOOP

    LOOP -->|AC-S1 met + BA confirms| S2C[Phase 2: Stakeholder Discovery\nActors · roles · decision authority]
    S2C --> LOOP

    LOOP -->|AC-S2 met + BA confirms| S3C[Phase 3: Requirement Elicitation\nFunctional and non-functional requirements]
    S3C --> LOOP

    LOOP -->|AC-S3 met + BA confirms| S4C[Phase 4: Constraint Capture\nBudget · timeline · compliance · residency]
    S4C --> LOOP

    LOOP -->|AC-S4 met + BA confirms| S5C[Phase 5: Architecture Alignment\nTranslate BA answers into technology direction]
    S5C --> LOOP

    LOOP -->|AC-S5 met + BA confirms| S6C[Phase 6: Review and Sign-Off]

    %% Artifacts generated — both must succeed before BA can approve
    S6C --> ARTGEN2[Generate BRD and HLD\nPremium LLM · domain template applied]
    ARTGEN2 --> BAREV2{BA reviews artifacts}

    BAREV2 -->|Revision requested| S6C
    BAREV2 -->|BA approves| SIGNFLOW2[Client Sign-Off Flow\nHMAC token · Email · Client signature]

    SIGNFLOW2 --> LOCKED2([Specification Locked\nExported · Audited · Downstream unblocked])
```

## Key System Rules (Summary)

| Rule | Description |
|---|---|
| INV-SEC-01 | Every vector query must include `tenant_id` filter — no exceptions |
| INV-SEC-02 | PII is scrubbed before embedding, never after |
| INV-EPI-01 | Every requirement must have at least one source chunk — no orphan knowledge in the spec |
| INV-EPI-02 | Agent may not resolve equal-tier conflicts — must halt and raise a Conflict object |
| INV-HITL-01 | Human edits are absolute ground truth — system may not overwrite them |
| INV-HITL-02 | No generated artifact is pushed downstream without explicit human approval |
| INV-HITL-05 | A locked specification is immutable — changes require a new version |
| INV-COST-01 | When project budget cap is reached, all agent tasks stop until cap is raised |

---

> Chitragupt · Flow 11 of 11 · May 2026
