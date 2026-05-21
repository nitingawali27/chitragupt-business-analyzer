# Flow 03 — Per-Turn LLM Pipeline

## What this covers

Every message the BA sends triggers a six-step pipeline before a response is generated. No step can be skipped. The final output always ends with exactly one clear action for the BA — a question, a confirmation prompt, or a transition offer.

**Design principle:** The BA should never wonder "what do I do next?" — that is a system failure.

## Important Components

| Agent | Model Tier | Role |
|---|---|---|
| `IntentClassifierAgent` | Fast LLM | Classifies what the BA meant (target: < 200ms) |
| `EntityExtractorAgent` | Standard LLM | Pulls structured data from the BA's words |
| `RAGRetrievalAgent` | — (retrieval) | Hybrid search over all session documents |
| `GapAnalyzerAgent` | Standard LLM | Checks which acceptance criteria are still unmet |
| `GuidanceGeneratorAgent` | Premium LLM | Composes the final response (streamed) |

## Intent Types

| Intent | When it fires |
|---|---|
| `ANSWER` | BA is providing new information |
| `CONFIRM` | BA accepts a system summary or proposal |
| `DENY / CORRECTION` | BA rejects or corrects something |
| `UPLOAD_SIGNAL` | BA mentions a file or document |
| `REVISIT` | BA wants to go back to a prior phase |
| `SKIP` | BA wants to move on without answering |
| `CLARIFICATION_REQUEST` | BA is asking the system a question |
| `UPLOAD_COMPLETE` | System event after successful document ingestion |
| `OUT_OF_SCOPE` | BA message is off-topic |

## Input → Process → Output

- **Input:** Any BA message
- **Process:** Classify intent → extract or handle → retrieve → analyze gaps → generate response
- **Output:** Streamed system message — acknowledge + captured bullets + one next action

## Diagram

```mermaid
flowchart TD
    %% Six-step pipeline — runs on every BA message without exception

    MSG([BA Message]) --> INTENT[Step 1: Classify Intent\nFast LLM · target under 200ms]

    INTENT --> BRANCH{Intent type?}

    %% Each intent type routes to its own handler in Step 2
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

    %% Rollback bypasses retrieval — goes straight to guidance with summary
    ROLLBACK --> GUIDE[Step 5: Generate Guidance\nPremium LLM · streamed response]

    %% Retrieval and validation both feed gap analysis
    RAG --> GAP[Step 4: Gap Analysis\nWhich AC is unmet?\nSelect highest-priority gap]
    VALIDATE --> GAP

    GAP --> GUIDE

    GUIDE --> READY{All AC met\nfor this state?}
    READY -->|No| QUESTION([Emit: one next question])
    READY -->|Yes| OFFER([Emit: transition offer with summary])
```

## Response Format (Every Turn)

```
[Acknowledgment — 1 sentence]
What the system heard from the BA, paraphrased.

[Captured — 0 to 3 bullets]
• What was extracted and added to the session.

[Next action — 1 sentence only]
Exactly one clear question or transition offer.
```

---

> Chitragupt · Flow 03 of 11 · May 2026
