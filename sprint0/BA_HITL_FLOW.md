# BA HITL Flow — Human-in-the-Loop BA Onboarding Protocol

**Phase:** Sprint 0 — Foundation
**Purpose:** Defines the conversation-first, state-aware protocol through which a Business Analyst (BA) is guided from initial problem statement to signed-off BRD and High-Level Architecture. The BA never fills out forms. The system drives the session via active chat; documents are uploaded only at designated checkpoints.

---

## Design Principle

The system acts as a structured interview partner, not a document editor. Every phase begins with the system asking a focused question. The BA responds in natural language. The system extracts structured knowledge from the conversation and presents it back for confirmation before moving forward.

Uploads and structured inputs are accepted only at checkpoints — they supplement the conversation, they do not replace it.

---

## Session State Machine

```
PROBLEM_INTAKE
     │
     ▼
STAKEHOLDER_DISCOVERY
     │
     ▼
REQUIREMENT_ELICITATION   ◄── [Checkpoint: process docs, wireframes]
     │
     ▼
CONSTRAINT_CAPTURE         ◄── [Checkpoint: compliance docs, infra specs]
     │
     ▼
ARCHITECTURE_ALIGNMENT
     │
     ▼
REVIEW_AND_SIGN_OFF        ◄── [Checkpoint: BA annotations, client comments]
     │
     ▼
SIGNED_OFF (terminal)
```

State transitions are explicit — the system proposes the transition, the BA confirms. A BA can request to revisit a prior state at any time; the system re-enters that state and presents a summary of what was already captured.

---

## Phase Definitions

### Phase 1 — PROBLEM_INTAKE

**System behavior:** Opens with an open-ended prompt asking the BA to describe the business problem in plain language. Follows up with clarifying questions until the problem statement is crisp: domain, affected users, urgency, and the gap between current state and desired state.

**BA inputs:** Free-form natural language chat.

**Uploads accepted:** None. The BA should not be asked to upload anything at this stage — the goal is to hear the problem in the BA's own words first.

**System extracts:**
- Problem statement (1–3 sentences)
- Business domain
- Affected user types
- Definition of success / acceptance signal

**Transition trigger:** System presents a synthesized problem summary and asks the BA to confirm or refine. On confirmation → STAKEHOLDER_DISCOVERY.

---

### Phase 2 — STAKEHOLDER_DISCOVERY

**System behavior:** Asks the BA to identify the key actors — who will use the system, who decides, who is impacted but not a direct user, and who holds sign-off authority. Probes for external systems or third parties involved.

**BA inputs:** Names, roles, descriptions in chat.

**Uploads accepted (Checkpoint A):**
- Org charts
- RACI matrices
- Existing stakeholder maps
- Any document that lists actors, teams, or responsibilities

**System extracts:**
- Actor list with roles
- Decision authority mapping
- External system dependencies
- Key communication touchpoints

**Transition trigger:** BA confirms the actor list is complete → REQUIREMENT_ELICITATION.

---

### Phase 3 — REQUIREMENT_ELICITATION

**System behavior:** Drives a structured Q&A to surface functional requirements. Covers: core user workflows, critical features, edge cases, integration needs, and what "done" looks like for each major capability. Uses the actor list from Phase 2 to frame questions per user type.

**BA inputs:** Chat responses, scenario descriptions, corrections to what the system has captured.

**Uploads accepted (Checkpoint B):**
- Wireframes or mockups
- Process flow diagrams
- Existing feature lists or backlog items
- Meeting notes or workshop outputs
- Legacy system documentation

**System extracts:**
- Functional requirements per actor
- User stories (Who / What / Why)
- Acceptance criteria candidates
- Open questions flagged for stakeholder resolution

**Transition trigger:** System presents extracted requirements list. BA confirms coverage or adds missing items → CONSTRAINT_CAPTURE.

---

### Phase 4 — CONSTRAINT_CAPTURE

**System behavior:** Asks about non-functional requirements and project-level constraints: budget envelope, delivery timeline, compliance obligations, geographic restrictions, performance expectations, and integration mandates.

**BA inputs:** Chat. Budget and timeline can be approximate ranges — precision is not required here.

**Uploads accepted (Checkpoint C):**
- Compliance requirement documents (GDPR, HIPAA, SOC2 checklists)
- Existing infrastructure specifications
- Vendor contracts or technical mandates
- Security policy documents

**System extracts:**
- Budget range (if available)
- Delivery timeline / milestone dates
- Compliance flags (data residency, PHI, audit requirements)
- Performance thresholds (latency, availability)
- Hard integration mandates (specific third-party systems required)
- Named assumptions

**Transition trigger:** BA confirms constraints are captured → ARCHITECTURE_ALIGNMENT.

---

### Phase 5 — ARCHITECTURE_ALIGNMENT

**System behavior:** Presents the key open decisions from `DECISIONS.md` in plain language — each one framed as a question the BA can answer without technical expertise (e.g., "Is this system used by a single company or multiple clients?" → drives multi-tenancy decision). The system explains the tradeoffs behind each choice at a business level.

**BA inputs:** Selections and preferences in chat. The BA does not need to know architecture — the system translates business answers into technical directions.

**Uploads accepted:** None required. BA may share reference architecture docs from prior systems if available.

**System produces:**
- Technology direction summary (which ADR options are now preferred based on BA answers)
- Key decisions marked as GUIDED (awaiting engineering validation) vs OPEN

**Transition trigger:** All key decisions in DECISIONS.md have at least a guided direction → REVIEW_AND_SIGN_OFF.

---

### Phase 6 — REVIEW_AND_SIGN_OFF

**System behavior:** Generates two artifacts from all captured context:
1. **Draft BRD** — structured requirements document with source citations, confidence tags, and open questions
2. **High-Level Architecture Diagram** — a system context diagram showing major components, integrations, and data flows

Presents both to the BA for review in the chat interface. The BA can request changes, flag errors, or approve sections individually.

**BA inputs:** Review comments in chat, section-by-section approvals, or wholesale revision requests.

**Uploads accepted (Checkpoint D):**
- Prior BRD versions for comparison
- Client review comments (if the BA gathered feedback externally)
- Reference architecture diagrams from client

**System behavior on revision requests:** Re-enters only the specific section flagged; does not re-generate approved sections (see HITL Authority in ARCHITECTURE.md).

**Transition trigger:** BA explicitly approves the BRD and HLA → SIGNED_OFF.

---

### Phase 7 — SIGNED_OFF (Terminal)

**System behavior:**
- Locks the BRD specification (no further automated modifications)
- Locks the HLA artifact
- Generates the export package (DOCX, PDF, Markdown)
- Records the sign-off event in the audit log with BA identity and timestamp
- Optionally triggers downstream export to connected systems (Jira, Confluence, etc.) pending workspace configuration

**BA inputs:** Export destination selection (optional).

**State:** Immutable. Any subsequent change requires creating a new project version from this locked baseline.

---

## Conversation Protocol — System Rules

1. **One question at a time.** The system never presents more than one question per turn. Follow-up questions are asked after the BA has responded.

2. **Summarize and confirm before moving on.** Before every state transition, the system presents a structured summary of what it captured in that phase and asks the BA to confirm or correct.

3. **Never assume.** If the BA's answer is ambiguous, the system asks a clarifying question rather than inferring. Inferred content is always tagged `[INFERRED — VERIFY]` when it appears in the draft BRD.

4. **Surface open questions explicitly.** Any topic the BA could not answer is logged as an Open Question with a suggested action (stakeholder to consult, document to retrieve). Open Questions appear in the draft BRD as a named section.

5. **Respect HITL authority.** Any edit the BA makes to a system-generated summary becomes ground truth for that item. The system does not re-derive or overwrite BA edits in subsequent phases.

6. **Checkpoint prompts are non-blocking.** Document upload checkpoints are offers, not requirements. The BA can skip any upload and proceed; the system will note the absence and flag higher uncertainty for related requirements.

---

## Output Artifacts by Phase

| Phase | Output |
|---|---|
| PROBLEM_INTAKE | Problem Statement card (persisted to session) |
| STAKEHOLDER_DISCOVERY | Actor Registry (roles, responsibilities, authority) |
| REQUIREMENT_ELICITATION | Draft Requirements List with confidence tags |
| CONSTRAINT_CAPTURE | Constraint & Assumption Register |
| ARCHITECTURE_ALIGNMENT | Decision Direction Summary |
| REVIEW_AND_SIGN_OFF | Draft BRD + High-Level Architecture Diagram |
| SIGNED_OFF | Locked BRD export (DOCX/PDF/Markdown), Audit record |

---

> Chitragupt BA HITL Flow • Sprint 0 • May 2026
