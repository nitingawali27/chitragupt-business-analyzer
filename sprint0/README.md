# Sprint 0 — Foundation

**Goal:** Establish the core documents that define how Chitragupt works and what decisions must be made before implementation begins.

Sprint 0 does not produce code. It produces: a clear BA onboarding protocol, consolidated architectural decisions, and the principles document that all subsequent sprints build on.

---

## Documents in This Directory

| File | Purpose |
|---|---|
| [BA_HITL_FLOW.md](BA_HITL_FLOW.md) | The BA conversation protocol — 7-phase HITL state machine from problem intake to signed-off BRD |
| [DECISIONS.md](DECISIONS.md) | All 14 open architectural decisions (language, LLM tiers, database, auth, connectors, etc.) |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Core principles: trust hierarchy, confidence tiers, traceability rules, invariants, engineering conventions |

---

## Status

| Document | Status |
|---|---|
| BA_HITL_FLOW.md | Draft — ready for BA team review |
| DECISIONS.md | All decisions OPEN — Architecture Alignment phase (Phase 5 of BA flow) will guide initial directions |
| ARCHITECTURE.md | Draft — binding on all subsequent implementation |

---

## Sprint 0 Exit Criteria

- [ ] All decisions in DECISIONS.md reach GUIDED or DECIDED status
- [ ] BA HITL flow reviewed and approved by BA team lead
- [ ] ARCHITECTURE.md ratified by engineering lead
- [ ] Data model (docs/architecture/ontology.md) validated against BA flow outputs

Sprint 0 closes when all exit criteria are met. Sprint 1 (Ingestion & RAG) begins immediately after.

---

> Chitragupt Sprint 0 • May 2026
