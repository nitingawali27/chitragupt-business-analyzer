# 04 — Knowledge & Trust

## What this is

Chitragupt reads dozens of documents, hours of conversation, and sometimes conflicting information from multiple sources. It needs a principled way to decide: what do I believe, how confident am I, and what do I do when sources disagree?

The answer is a trust hierarchy combined with a confidence scoring system. Together they determine whether a requirement appears in the BRD as a firm commitment, a suggestion, or an open question — and what happens when two sources say different things.

---

## The Trust Hierarchy

Not all information is equally reliable. The system assigns every piece of knowledge a rank based on where it came from. Higher rank always wins.

```mermaid
graph TB
    R1["🏛  Rank 1 — Human Override\nThe BA's own edits, approvals, and rejections.\nAbsolute. Cannot be overwritten by the system."]
    R2["📊  Rank 2 — Structured External Systems\nJira fields, typed API contracts, database schemas.\nHigh trust — the source system enforced structure."]
    R3["📄  Rank 3 — Primary Source Documents\nClient PDFs, signed Confluence pages, official emails.\nBaseline trust — authored by the client or signed off."]
    R4["📝  Rank 4 — Secondary Documents\nMeeting notes, Slack threads, internal memos.\nModerate trust — informal, may not reflect final decisions."]
    R5["🤖  Rank 5 — Agentic Inference\nLLM synthesis and industry-standard assumptions.\nConditional trust — always tagged, always verifiable."]

    R1 --- R2
    R2 --- R3
    R3 --- R4
    R4 --- R5

    style R1 fill:#1565C0,color:#fff,stroke:none
    style R2 fill:#1976D2,color:#fff,stroke:none
    style R3 fill:#42A5F5,color:#212121,stroke:none
    style R4 fill:#90CAF9,color:#212121,stroke:none
    style R5 fill:#E3F2FD,color:#212121,stroke:none
```

**The key rule:** A Rank 5 inference can never silently replace a Rank 3 document. If the LLM thinks it knows better than what the client wrote — it is wrong.

---

## How Confidence Scores Work

Every requirement the system produces carries a score from 0 to 1. The score is computed from five signals and determines what tag — if any — appears next to the requirement.

```mermaid
flowchart LR
    subgraph Signals["Five Scoring Signals"]
        A[Retrieval match\nhow closely the chunk\nmatched the query\n40% weight]
        B[Re-ranking score\ncross-encoder precision\ncheck after retrieval\n30% weight]
        C[Source trust tier\nhigher rank = boost\n15% weight]
        D[LLM self-confidence\ndiscounted — LLMs\nover-report certainty\n10% weight]
        E[Historical accuracy\nwere similar claims\napproved by humans before?\n5% weight]
    end

    Signals --> SCORE[Calibrated\nConfidence Score\n0.0 – 1.0]

    SCORE --> T1{Score}

    T1 -->|≥ 0.85| C1[✅  No tag\nHigh confidence\nInclude as-is]
    T1 -->|0.65 – 0.84| C2[🔵  SYNTHESIZED\nLogically derived\nReview recommended]
    T1 -->|0.40 – 0.64| C3[🟡  INFERRED — VERIFY\nPattern-based\nMust be checked]
    T1 -->|below 0.40| C4[❌  Excluded\nRaised as Open Question\nNever in the BRD]
```

**Visual extractions** — anything pulled from a diagram or screenshot — have a hard cap of 0.80 regardless of score, and always carry the `[VISUAL EXTRACTION — VERIFY]` tag. A diagram is an illustration, not a contract.

---

## What Happens When Sources Conflict

The system never resolves a conflict on its own. It stops, surfaces both sides, and waits for the BA.

```mermaid
flowchart TD
    A[Two sources address\nthe same topic] --> B{Same trust rank?}

    B -->|No — different ranks| C[Higher rank wins\nLower-rank claim discarded\nNo human action needed]

    B -->|Yes — same rank| D[CONFLICT DETECTED\nSynthesis halted for this topic]

    D --> E[Show both sources\nside by side with full citations]

    E --> F[BA chooses]

    F -->|Accept Source A| G[Source A locked as\nRank 1 ground truth]
    F -->|Accept Source B| H[Source B locked as\nRank 1 ground truth]
    F -->|Neither — I'll clarify| I[BA writes the\ncorrect statement\nBecomes Rank 1]

    G --> J[Synthesis resumes\nusing the resolved truth]
    H --> J
    I --> J

    style D fill:#B71C1C,color:#fff,stroke:none
    style J fill:#1B5E20,color:#fff,stroke:none
```

**Why this matters:** Chitragupt is used to commit engineering work. A requirement produced by silently picking one conflicting source over another is a liability. The BA — who knows the client — must make that call, not the model.

---

## Orphan Knowledge — The Hard Line

Any claim the LLM produces that cannot be traced back to at least one source chunk is called **Orphan Knowledge**. It is treated as a hallucination.

```mermaid
flowchart LR
    A[LLM generates a claim] --> B{Can it be traced\nto a source chunk?}

    B -->|Yes — source found| C[Requirement created\nwith source citation\nand confidence score]

    B -->|No — no source| D[ORPHAN KNOWLEDGE]

    D --> E{Does the BA\nwant to keep it?}

    E -->|Yes, I know the source| F[BA points to source\nor uploads document\nClaim becomes grounded]
    E -->|Yes, it's inference| G[Tagged SPECULATIVE\nFlagged for expert review\nNot a firm requirement]
    E -->|Remove it| H[Discarded entirely]

    C --> OUT([In the BRD])
    F --> OUT
    G --> REVIEW([In BRD — flagged section])
    H --> GONE([Not in the BRD])
```

The BRD is only as trustworthy as its sources. A requirement with no source is a guess dressed as a commitment.
