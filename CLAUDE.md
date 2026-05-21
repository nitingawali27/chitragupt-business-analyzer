# CLAUDE.md — Chitragupt Project Instructions

## Prompt Registry

**Every user prompt in this project must be logged to `docs/logs/prompt_trail.md`.**

Before responding to any user request, append a new entry to the prompt trail using the established format:

```
**P-NNN**
> [exact user prompt verbatim]
```

Where NNN is the next sequential number after the last logged entry. Do not paraphrase or clean up the prompt — log it exactly as the user typed it. If a prompt was clearly inferred from context (no verbatim text), mark it `[INFERRED]` and summarize it in one sentence.

Add new entries under the appropriate phase section heading. If a prompt doesn't fit an existing phase, create a new `## Phase Name` heading.

This rule applies to all sessions in this project, without exception.

---

## Project Overview

**Chitragupt** is an agentic Business Requirement Analyzer. It ingests multi-modal documents and stakeholder conversations, extracts structured requirements, and produces client-ready BRDs and High-Level Architecture Diagrams.

**Primary user:** Business Analysts (BAs) who are guided through a structured HITL conversation flow — not document editors.

---

## Key Documents

| Document | Purpose |
|---|---|
| `sprint0/BA_HITL_FLOW.md` | BA onboarding protocol (7-phase HITL state machine) |
| `sprint0/DECISIONS.md` | All open architectural decisions |
| `sprint0/ARCHITECTURE.md` | Trust hierarchy, invariants, engineering conventions |
| `docs/architecture/ontology.md` | Complete data model and entity schemas |
| `docs/logs/prompt_trail.md` | Prompt registry (append-only) |

---

## Engineering Conventions (Summary)

- Python 3.11+, `black` formatter, `isort`, `mypy --strict`
- Commit format: `<type>(<scope>): <description>` (Conventional Commits)
- Branch naming: `feat/sprint-N-name`, `fix/id-name`, `chore/name`
- Every LLM call must use pinned model versions — no floating aliases
- RLS enforced on every database table with tenant-specific data
- No code unless a Sprint 0 decision is DECIDED for the relevant technology

---

## What NOT to do

- Do not create new documentation files unless the user explicitly asks for one.
- Do not implement code for any component where the corresponding decision in `sprint0/DECISIONS.md` is still OPEN.
- Do not add orchestration middleware or framework-specific stubs — that decision is OPEN.
- Do not introduce new ADR files — use `sprint0/DECISIONS.md` as the single decisions document.
