# Flow 08 — Conflict Detection and Resolution

## What this covers

When two retrieved chunks from sources of equal trust tier contradict each other on the same topic, the system halts synthesis on that topic and raises a Conflict object. It never guesses which source is correct. The BA must resolve it explicitly before the affected requirements can be finalized.

**Key rule (INV-EPI-02):** The agent may not unilaterally resolve conflicts between equal-tier sources. It must halt and raise a Conflict object. The human's choice becomes Rank 1 (human override) truth for that topic.

## Important Components

| Component | Role |
|---|---|
| `Conflict` entity | Stores both source citations, severity, and affected requirements |
| Conflict status | `open`, `resolved_a`, `resolved_b`, `resolved_manual`, `escalated` |
| HITL authority | BA resolution becomes Rank 1 truth — immutable thereafter |
| `[CONFLICT — PENDING RESOLUTION]` tag | Visible flag in BRD until resolved |
| AC-S6-3 | All Conflict objects must have `status = resolved_*` before sign-off |

## Conflict Severity Levels

| Severity | Meaning |
|---|---|
| `low` | Affects a non-critical section; does not block transition |
| `medium` | Should be resolved before sign-off |
| `high` | Blocks the affected requirement from being finalized |
| `blocking` | Blocks the entire BRD approval until resolved |

## Input → Process → Output

- **Input:** Two retrieved chunks from same-tier sources that contradict on a topic
- **Process:** Detect → create Conflict object → flag → present to BA → apply resolution
- **Output:** BA resolution recorded as Rank 1 truth; synthesis resumes on the affected topic

## Diagram

```mermaid
flowchart TD
    %% Conflicts halt synthesis — never auto-resolved by the agent

    CHUNKS2([Retrieved Chunks]) --> COMPARE[Compare Claims\nSame topic detected across sources]

    COMPARE --> FOUND{Contradiction\nfound?}

    FOUND -->|No| PROCEED[Continue synthesis normally]

    FOUND -->|Yes| TIERS{Same trust\ntier?}

    %% Different tiers: higher rank wins — no conflict raised
    TIERS -->|No — different tiers| WIN[Higher tier wins automatically\nLower claim discarded]

    %% Equal tiers: halt and raise a Conflict object
    TIERS -->|Yes — equal tiers| HALT[Halt synthesis on this topic]

    HALT --> OBJECT[Create Conflict Object\nCite both sources · record severity]

    OBJECT --> FLAG[Flag affected section\nCONFLICT — PENDING RESOLUTION]

    FLAG --> PRESENT[Present to BA\nSource A vs Source B side by side]

    PRESENT --> RESOLVE{BA decision}

    RESOLVE -->|Accept A| RA[Record Source A as truth\nRank 1 override · resume synthesis]
    RESOLVE -->|Accept B| RB[Record Source B as truth\nRank 1 override · resume synthesis]
    RESOLVE -->|Provide clarification| RM[Record manual resolution\nResume synthesis]

    WIN --> PROCEED
    RA & RB & RM --> PROCEED
```

## Conflict Object Fields

```
conflict_id         · project_id          · conflict_type
source_a_chunk_id   · source_b_chunk_id   · affected_requirements
status              · resolution          · resolved_by
severity            · resolved_at
```

`conflict_type` values: `direct_contradiction` · `scope_overlap` · `version_conflict` · `implicit_contradiction`

---

> Chitragupt · Flow 08 of 11 · May 2026
