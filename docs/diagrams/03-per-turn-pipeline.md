# 03 — Per-Turn Pipeline

## What this is

Every time the BA sends a message, the system runs a five-step pipeline before responding. Each step has a single job. The output of one step feeds the next. The pipeline always ends with exactly one action for the BA: a question, a confirmation prompt, or a gate resolution request.

This diagram shows what happens inside the system on each turn — in plain terms.

---

## The Five Steps

```mermaid
flowchart TD
    IN([BA sends a message]) --> S1

    subgraph S1["Step 1 — Intent Classification\n⚡ Fast model · &lt; 200ms"]
        IC[What is the BA trying to do?\nAnswer · Confirm · Skip · Revisit\nUpload signal · Correction · Question]
    end

    S1 --> S2

    subgraph S2["Step 2 — Entity Extraction\n🔍 Standard model · state-aware"]
        EE[Pull out the structured facts\nfrom what the BA just said.\nActors · Requirements · Constraints\nDecisions · Names · Numbers]
    end

    S2 --> S3

    subgraph S3["Step 3 — Context Retrieval\n📚 Search uploaded documents"]
        RAG[Find the most relevant chunks\nfrom documents uploaded this session.\nUsed to ground extracted facts\nand find supporting evidence.]
    end

    S3 --> S4

    subgraph S4["Step 4 — Gap Analysis\n🔎 Standard model · AC evaluator"]
        GA[Check: which acceptance criteria\nare still unmet for the current phase?\nPick the single highest-priority gap\nand form the next question.]
    end

    S4 --> S5

    subgraph S5["Step 5 — Guidance Generation\n✍️ Premium model · streamed"]
        GG[Write the response:\n1. Acknowledge what was said\n2. Show what was captured\n3. Ask exactly one next question\nor offer to advance the phase]
    end

    S5 --> OUT([Response streams to BA])
```

---

## What Each Step Produces

```mermaid
flowchart LR
    A[BA message\n'The admin needs to\nexport CSV reports'] -->|raw text| B

    B[Intent Classifier] -->|ANSWER| C

    C[Entity Extractor] -->|Actor: Admin\nRequirement: CSV export\nType: Functional| D

    D[RAG Retrieval] -->|Chunk: Payment_Gateway_V2.pdf p4\nRelevance: 0.91\nContent: reconciliation report spec| E

    E[Gap Analyzer] -->|AC met: FR extracted ✓\nAC unmet: no acceptance criteria yet\nNext question: what triggers the export?| F

    F[Guidance Generator] -->|Got it — the Admin needs to export\nreconciliation reports as CSV.\n\n• Requirement added: FR-007\n• Source: Payment Gateway doc, p.4\n\nWhat should trigger the export —\na scheduled job or a manual action?| G

    G([Response to BA])
```

---

## How Intent Changes the Route

Not every message goes through all five steps the same way. The intent determines which path is taken.

```mermaid
flowchart TD
    MSG([BA message]) --> IC[Intent Classifier]

    IC -->|ANSWER| EE[Entity Extractor\nthen full pipeline]
    IC -->|CONFIRM| AC[AC Validator\ncheck if all AC met\nthen offer transition]
    IC -->|SKIP| SK[Log open question\napply confidence impact\nthen next gap]
    IC -->|REVISIT| RV[Load prior phase summary\nre-enter that phase\nskip to last unmet AC]
    IC -->|UPLOAD SIGNAL| UP[Issue upload prompt\nor confirm upload checkpoint]
    IC -->|CORRECTION| CR[Find the entity being corrected\noverwrite it\nmark prior version as overridden]
    IC -->|QUESTION| QA[Answer the BA's question\nthen re-ask the current gap]

    EE --> RAG[RAG Retrieval]
    RAG --> GA[Gap Analyzer]
    GA --> GG[Guidance Generator]
    AC --> GG
    SK --> GA
    RV --> GG
    UP --> GG
    CR --> GA
    QA --> GG

    GG --> OUT([Response to BA])
```

---

## The Response Contract

No matter what path the pipeline takes, every response follows this structure. The Guidance Generator is constrained to produce exactly this — no walls of text, no open-ended rambling.

```
Acknowledge   →  One sentence. What you heard, paraphrased.
Captured      →  0 to 3 bullets. What was extracted and added to the session.
               (omitted if nothing new was captured)
Next action   →  One sentence only. A question, a transition offer,
               or a gate resolution prompt.
```

The BA always knows exactly what to do next. That is the guarantee this pipeline exists to deliver.
