# 01 — The BA Journey

## What this is

A Business Analyst starts a session with nothing but a problem in their head. Chitragupt leads them — one question at a time — through seven phases until a signed-off BRD and architecture diagram lands in the client's inbox. The BA never fills out a form. The system always knows what to ask next.

This diagram shows the full journey, what gets captured at each phase, and where documents can be uploaded to improve confidence in the output.

---

## The Journey

```mermaid
stateDiagram-v2
    direction TB

    [*] --> ProblemIntake : Session starts

    state ProblemIntake {
        [*] --> [*]
        note right of [*]
            System asks: What problem are you solving?
            Captures: problem statement, domain, affected users,
            definition of success
        end note
    }

    ProblemIntake --> StakeholderDiscovery : Problem confirmed ✓

    state StakeholderDiscovery {
        [*] --> [*]
        note right of [*]
            System asks: Who is involved?
            Captures: actors, roles, decision authority,
            external systems
            📎 Upload: org chart, RACI
        end note
    }

    StakeholderDiscovery --> RequirementElicitation : Actors confirmed ✓

    state RequirementElicitation {
        [*] --> [*]
        note right of [*]
            System asks: What does each actor need?
            Captures: functional requirements, NFRs,
            acceptance criteria, open questions
            📎 Upload: wireframes, process docs
        end note
    }

    RequirementElicitation --> ConstraintCapture : Requirements confirmed ✓
    RequirementElicitation --> RequirementElicitation : BA requests revision

    state ConstraintCapture {
        [*] --> [*]
        note right of [*]
            System asks: What are the boundaries?
            Captures: budget, timeline, compliance flags,
            data residency, integration mandates
            📎 Upload: compliance docs, infra specs
        end note
    }

    ConstraintCapture --> ArchitectureAlignment : Constraints confirmed ✓

    state ArchitectureAlignment {
        [*] --> [*]
        note right of [*]
            System asks: Business-language tech questions
            Captures: guided directions for key
            architectural decisions
            📎 Upload: existing architecture plans
        end note
    }

    ArchitectureAlignment --> ReviewAndSignOff : Decisions guided ✓

    state ReviewAndSignOff {
        [*] --> [*]
        note right of [*]
            System generates: Draft BRD + HLD diagram
            BA reviews, requests changes, approves
            📎 Upload: client review comments
        end note
    }

    ReviewAndSignOff --> RequirementElicitation : Revision requested
    ReviewAndSignOff --> SignedOff : BA approves ✓

    state SignedOff {
        [*] --> [*]
        note right of [*]
            BRD and HLD locked
            Client signs off
            Export triggered
        end note
    }

    SignedOff --> [*]
```

---

## Key Rules

**The system always leads.** Every phase starts with the system asking a focused question, not the BA filling in a field.

**Transitions require confirmation.** The system presents a summary of what it captured and waits for the BA to say "yes, that's right" before advancing. The BA is never surprised by where the session ends up.

**Uploads improve confidence, not gate progress.** Most uploads are optional — the BA can proceed without them. The system notes absences and lowers confidence scores on affected requirements. A few uploads are hard-blocked (e.g., the client signature to reach Signed Off).

**Revision is always safe.** The BA can return to any prior phase at any time. Captured data is preserved; the session re-enters that phase and picks up from the last unanswered gap.
