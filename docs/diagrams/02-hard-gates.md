# 02 — Hard Gates

## What this is

A gate is a condition that must be resolved before the session can move forward. Without gates, the system could produce a signed BRD with no compliance documentation for a healthcare project, or lock a specification without a client signature. Gates prevent those outcomes.

Not all gates are equal. Some stop the world. Some just ask a question. This document explains the four gate types and why each exists.

---

## The Four Gate Types

```mermaid
quadrantChart
    title Gate Types — How hard the block is vs. how often it fires
    x-axis Fires Rarely --> Fires Often
    y-axis Soft Block --> Hard Block

    quadrant-1 Highest risk — handle first
    quadrant-2 Critical but uncommon
    quadrant-3 Low friction guardrails
    quadrant-4 Common workflow nudges

    HARD GATE: [0.15, 0.95]
    REQUIRED PROMPT: [0.65, 0.55]
    TRIGGERED: [0.45, 0.25]
    RECOMMENDED: [0.80, 0.10]
```

| Gate Type | What it does | Can the BA skip it? |
|---|---|---|
| **HARD GATE** | Transition is unreachable. Full stop. | No |
| **REQUIRED PROMPT** | System must ask the question before offering to advance. BA may then say no. | Yes — but the question must be asked |
| **TRIGGERED** | System detects a reference and asks. BA can decline. Decline is recorded. | Yes — confidence impact noted |
| **RECOMMENDED** | System suggests once. BA may ignore entirely. | Yes — no impact |

---

## Why Hard Gates Exist

```mermaid
flowchart TD
    A([BA says: let's move on]) --> B{Any HARD GATE open?}

    B -->|No| C{Any REQUIRED PROMPT\nnot yet issued?}
    C -->|No| D{Conversational AC\nall met?}
    D -->|No| E[Surface next gap\nAsk the missing question]
    D -->|Yes| F[Present summary\nOffer transition]
    F --> G{BA confirms?}
    G -->|Yes| H([Advance to next phase])
    G -->|No| E

    C -->|Yes| I[Issue the checkpoint prompt\nbefore anything else]
    I --> A

    B -->|Yes| J{What kind of hard gate?}

    J -->|BRD not generated| K[Generate BRD now\nThen re-evaluate]
    J -->|Client signature pending| L[Show status:\nAwaiting signature from client email\nCannot proceed until signed]
    J -->|Regulated domain —\nno source document| M[Surface:\nThis is a healthcare project.\nWe need at least one reference doc\nor an explicit waiver.]
    J -->|Compliance flags —\nno compliance doc| N[Surface:\nGDPR is flagged but no compliance\ndoc has been attached.\nUpload or confirm none exists.]

    K --> B
    M --> O{BA uploads or waives?}
    N --> O
    O -->|Uploads| P[Ingest document\nRe-evaluate gates]
    O -->|Explicit waiver| Q[Record waiver\nTag affected items\nRe-evaluate gates]
    P --> B
    Q --> B
```

---

## The Hard Gates in This System

| Gate | Transition | Why it cannot be skipped |
|---|---|---|
| Client signature | Review → Signed Off | A BRD without a client signature is not a deliverable — it is a draft |
| BRD artifact exists | Review → Signed Off | Cannot approve a document that has not been generated and stored |
| HLD artifact exists | Review → Signed Off | The architecture diagram is a required deliverable, not optional |
| Source document (regulated domain) | Elicitation → Constraints | Healthcare, fintech, and government requirements with zero grounding have unacceptable liability risk |
| Existing architecture doc (if system exists) | Constraints → Alignment | Architecture decisions made without knowing the existing system will likely contradict it |

---

## What Happens When a Gate Fires

The system does not say "error." It surfaces exactly one resolution action — the simplest possible thing the BA can do to unblock progress.

```mermaid
flowchart LR
    A[HARD GATE fires] --> B[System identifies\nthe one action needed]
    B --> C{Action type}
    C -->|Upload needed| D[Show upload prompt\nwith specific doc type]
    C -->|Generate artifact| E[Trigger generation\nautomatically]
    C -->|Awaiting external| F[Show waiting state\nwith who holds the ball]
    C -->|Waiver available| G[Offer explicit waiver\nwith consequence explained]
    D --> H[Gate re-evaluated\nafter action]
    E --> H
    G --> H
    F --> I[Notify BA\nwhen unblocked]
```

The BA never sees a wall of errors. They see one message: what the gate is, and the single next step to resolve it.
