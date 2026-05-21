# Flow 07 — Trust Hierarchy and Confidence Scoring

## What this covers

Every piece of extracted knowledge is tagged with two values: a **trust tier** (how reliable the source is) and a **confidence score** (how certain the extraction is). When two sources conflict, the higher trust tier always wins automatically. Claims below a confidence threshold appear with visible warning tags in the final BRD — or are dropped entirely as Open Questions.

**Key rule:** Rank 5 (Agentic Inference) may never supersede or silently replace Rank 3+ claims. Silent merging across tiers is prohibited.

## Important Components

| Component | Role |
|---|---|
| Trust tiers 1–5 | Assigned per source type at ingestion time |
| Confidence score (0.0–1.0) | Computed by the extraction agent per claim |
| Confidence tags | `[SYNTHESIZED]`, `[INFERRED — VERIFY]` — visible in BRD output |
| Orphan Knowledge rule | Any LLM claim with no matching chunk is raised as an Open Question |
| Visual extraction cap | All image/diagram extractions are hard-capped at 0.80 confidence |

## Trust Tier Reference

| Rank | Source | Examples |
|---|---|---|
| 1 | Human Override | BA inline edits, approvals, rejections |
| 2 | Structured External Systems | Jira fields, typed API contracts, database schemas |
| 3 | Primary Source Documents | Client PDFs, signed Confluence pages, official emails |
| 4 | Secondary Documents | Meeting notes, Slack threads, internal memos |
| 5 | Agentic Inference | LLM synthesis, industry-standard assumptions |

## Input → Process → Output

- **Input:** A claim produced by any agent, along with its source document
- **Process:** Assign trust tier from source → score the extraction → tag or drop
- **Output:** Tagged requirement in session state, or Open Question raised for the BA

## Diagram

```mermaid
flowchart TD
    %% Every claim is tiered by source quality before scoring begins

    CLAIM[Incoming Claim] --> TIER{Assign Trust Tier\nbased on source type}

    TIER -->|Rank 1| T1[Human Override\nBA edits · approvals · rejections]
    TIER -->|Rank 2| T2[Structured External Systems\nJira fields · typed API contracts]
    TIER -->|Rank 3| T3[Primary Source Documents\nClient PDFs · signed pages]
    TIER -->|Rank 4| T4[Secondary Documents\nMeeting notes · Slack threads]
    TIER -->|Rank 5| T5[Agentic Inference\nLLM synthesis · industry assumptions]

    T1 & T2 & T3 & T4 & T5 --> SCORE[Assign Confidence Score\n0.0 to 1.0]

    SCORE --> RANGE{Score range?}

    RANGE -->|0.85 to 1.00| EXPLICIT[No tag\nExplicit extraction · include as-is]
    RANGE -->|0.65 to 0.84| SYNTH[Tag: SYNTHESIZED\nInclude with visible tag]
    RANGE -->|0.40 to 0.64| INFER[Tag: INFERRED — VERIFY\nInclude with visible tag]
    RANGE -->|Below 0.40| DROP[Raise as Open Question\nDo not include in spec]
```

## Calibration Targets

| Metric | Target |
|---|---|
| Brier Score | < 0.15 vs. human expert review |
| ECE (Expected Calibration Error) | < 0.10 |
| Human rejection rate for claims scored > 0.85 | Must stay below 15% — recalibrate if exceeded |
| Visual extraction confidence cap | Hard cap at 0.80 regardless of LLM-reported score |

---

> Chitragupt · Flow 07 of 11 · May 2026
