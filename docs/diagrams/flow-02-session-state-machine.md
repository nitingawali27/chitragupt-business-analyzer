# Flow 02 — BA Session State Machine

## What this covers

The session advances through seven phases in order. The BA can never be moved forward automatically — every transition requires the BA to explicitly confirm. The BA can also revisit any prior phase at any time without losing what was already captured.

**Key rule:** Every state has Acceptance Criteria (AC). All AC must pass before the system even offers a transition. If the BA says "let's move on" before AC are met, the system surfaces the unmet criteria explicitly.

## Important Components

| Component | Role |
|---|---|
| `SessionState` | Typed object that holds all captured data for the session |
| `TransitionHandler` | Proposes transitions and awaits BA confirmation |
| Acceptance Criteria (AC) | Discrete, programmatically checkable conditions per state |
| REVISIT handler | Re-enters a prior state and resumes from the last unmet criterion |
| `GapAnalyzerAgent` | Evaluates which AC are unmet after every turn |

## Input → Process → Output

- **Input:** BA natural language messages and confirmation responses
- **Process:** After every turn, all AC for the current state are evaluated; transition is offered only when all pass and BA confirms
- **Output:** Session advances to the next state; all captured data is persisted to PostgreSQL

## Diagram

```mermaid
flowchart TD
    %% Seven phases — transitions are gated by AC and require explicit BA confirmation

    START([New Session Created]) --> S1[PROBLEM_INTAKE\nDescribe the business problem]

    S1 -->|All AC-S1 met\nBA confirms| S2[STAKEHOLDER_DISCOVERY\nIdentify actors and roles]

    S2 -->|All AC-S2 met\nBA confirms| S3[REQUIREMENT_ELICITATION\nMap what each actor needs]

    S3 -->|All AC-S3 met\nBA confirms| S4[CONSTRAINT_CAPTURE\nBudget · timeline · compliance]

    S4 -->|All AC-S4 met\nBA confirms| S5[ARCHITECTURE_ALIGNMENT\nGuide technology decisions]

    S5 -->|All AC-S5 met\nBA confirms| S6[REVIEW_AND_SIGN_OFF\nReview draft BRD and HLD]

    %% BA can request revision and return to requirement elicitation
    S6 -->|BA requests revision| S3

    S6 -->|All AC-S6 met\nBA approves| S7[SIGNED_OFF\nLocked and exported]

    S7 --> END([Session Complete])
```

## Output Artifacts per Phase

| Phase | What gets produced |
|---|---|
| PROBLEM_INTAKE | Problem Statement card |
| STAKEHOLDER_DISCOVERY | Actor Registry |
| REQUIREMENT_ELICITATION | Draft Requirements List with confidence tags |
| CONSTRAINT_CAPTURE | Constraint and Assumption Register |
| ARCHITECTURE_ALIGNMENT | Decision Direction Summary |
| REVIEW_AND_SIGN_OFF | Draft BRD + HLD |
| SIGNED_OFF | Locked BRD export (DOCX/PDF/Markdown) + Audit record |

---

> Chitragupt · Flow 02 of 11 · May 2026
