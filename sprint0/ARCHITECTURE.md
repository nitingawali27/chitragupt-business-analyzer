# Architecture Principles — Chitragupt

**Purpose:** Canonical reference for how Chitragupt acquires, validates, and emits knowledge — and the non-negotiable rules that govern system behavior. Engineering decisions that conflict with anything in this document are bugs, not product choices.

This document consolidates what was previously spread across `epistemology.md`, `invariants.md`, and `conventions_and_protocols.md`.

---

## 1. Trust Hierarchy

Every claim in a generated specification must be traceable to a source with an assigned trust tier. Higher tiers always win over lower tiers. Silent merging across tiers is prohibited.

| Rank | Source | Examples |
|---|---|---|
| 1 | Human Override | BA inline edits, explicit approvals, rejections |
| 2 | Structured External Systems | Jira fields, typed API contracts, database schemas |
| 3 | Primary Source Documents | Client PDFs, signed Confluence pages, official emails |
| 4 | Secondary Documents | Meeting notes, Slack threads, internal memos |
| 5 | Agentic Inference | LLM synthesis; industry-standard assumptions |

Rank 5 claims may never supersede or silently replace Rank 3+ claims. When tiers conflict, the higher rank wins and the lower-ranked claim is discarded or flagged — never merged.

---

## 2. Confidence Tiers

Every synthesized output carries a confidence score (0.0–1.0) and a visible tag in both the database record and the rendered output.

| Score Range | Label | Tag in Output |
|---|---|---|
| 0.85–1.00 | Explicit Extraction | *(no tag — high confidence)* |
| 0.65–0.84 | Deductive Synthesis | `[SYNTHESIZED]` |
| 0.40–0.64 | Inductive Inference | `[INFERRED — VERIFY]` |
| < 0.40 | Speculative | Do not include — raise as Open Question instead |

Visual extractions (diagrams, screenshots) have a hard confidence cap of 0.80, regardless of LLM-reported score, and always carry `[VISUAL EXTRACTION — VERIFY]`.

**Calibration target:** Brier Score < 0.15, ECE < 0.10 vs. human expert review. Recalibrate if human rejection rate for claims scored > 0.85 exceeds 15%.

---

## 3. Traceability

Every generated requirement, constraint, and assumption must maintain:

- Source document ID and chunk ID
- Retrieval score at query time
- Agent ID and model ID that produced the output
- Timestamp of synthesis
- Confidence score

Any LLM-generated claim with no matching chunk in the vector store is **Orphan Knowledge** — treated as a hallucination. Orphan Knowledge must not appear in the final specification as a confirmed requirement. It must be raised as an Open Question or tagged `[SPECULATIVE — REVIEW]`.

---

## 4. Conflict Protocol

When two sources of equal trust tier contradict each other, the system halts synthesis on that topic. It does not guess which source is correct.

**Required actions:**
1. Create a Conflict object with full citations from both sources.
2. Mark affected section `[CONFLICT — PENDING RESOLUTION]`.
3. Present both sources side-by-side in the review UI.
4. Await explicit human resolution: Accept A, Accept B, or Provide Clarification.
5. Resume synthesis only after resolution; the human's choice becomes Rank 1 truth for that topic.

The exception: when the trust hierarchy is unambiguous (e.g., Rank 1 Human Override vs. Rank 3 document), the higher rank wins automatically — no conflict flag needed.

---

## 5. Non-Negotiable Invariants

These hold under all conditions including edge cases, failures, and high load. Violations are system bugs.

### Security

**INV-SEC-01 — Tenant Isolation:** No data from Tenant A may be queried, retrieved, or displayed in a Tenant B session. Enforced via separate vector namespaces, PostgreSQL RLS, JWT-bound API middleware, and mandatory `tenant_id` filter on every vector query. Any vector query issued without a `tenant_id` filter is a critical defect.

**INV-SEC-02 — PII Scrubbing:** Chat session and elicitation content must be scanned and PII-redacted before any embedding call. Scrubbing executes before embedding, never after.

**INV-SEC-03 — File Isolation:** Uploaded documents are stored at paths scoped to `{tenant_id}/{project_id}/`. No user may receive a presigned URL pointing to another tenant's file.

**INV-SEC-04 — Audit Log Immutability:** The audit log is append-only. No entry may be modified or deleted by any user, agent, or process.

### Epistemology

**INV-EPI-01 — Traceability:** Every requirement in the final specification must contain a populated `source_chunks` array pointing to at least one active chunk. Requirements with empty source arrays may not be written to the database except as explicit human overrides.

**INV-EPI-02 — Conflict Non-Resolution:** The agent may not unilaterally resolve conflicts between equal-tier sources. It must halt and raise a Conflict object.

**INV-EPI-03 — Confidence Tagging:** Any output with confidence < 0.85 must carry the appropriate confidence tag in both the database record and the rendered output. Stripping tags is a product defect.

**INV-EPI-04 — Embedding Consistency:** All chunks in a vector namespace must use the same embedding model and dimension. Mixing models within a single index is strictly prohibited. Model upgrades require full re-embedding of all active chunks before queries are issued.

### HITL Authority

**INV-HITL-01 — Human Override:** A human edit, approval, or rejection is absolute ground truth. The system may not overwrite a human edit during re-generation cycles. The original AI text is preserved in `human_override_text`; the human edit is in `description`.

**INV-HITL-02 — No Auto-Push:** No generated artifact may be pushed to downstream systems (Jira, Confluence, CI/CD) without explicit human approval of that specific export. Specifications in `draft` or `in_review` status must never trigger external writes.

**INV-HITL-05 — Spec Lock:** A locked specification is immutable. No further automated changes may be made to its requirements, constraints, or assumptions. Changes after locking require creating a new specification version.

### Cost Control

**INV-COST-01 — Project Budget Cap:** When a project's cost reaches its configured cap, all agent tasks for that project stop. The state is preserved. A human must explicitly raise the cap to resume.

**INV-COST-03 — Cost Attribution:** Every LLM call must be attributed to a specific project, agent, and model tier before it is issued. Unattributed calls are prohibited.

**INV-MODEL-03 — Pinned Model Versions:** All LLM calls must use pinned model versions (e.g., `claude-sonnet-4-6`, not `claude-sonnet-latest`). Model upgrades are treated as deployment events with quality validation.

---

## 6. Engineering Conventions

### Python Standards

- Formatter: `black` (line length 88)
- Imports: `isort` with black-compatible profile
- Type hints: mandatory on all function signatures; `mypy --strict` must pass
- No multi-paragraph docstrings — one short line max, only when the why is non-obvious

### Git Protocol

Branch naming: `feat/sprint-N-name`, `fix/id-name`, `chore/name`, `perf/name`

Commit format (Conventional Commits):
```
<type>(<scope>): <description>
```
Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`

Breaking changes: append `!` after type/scope or add `BREAKING CHANGE:` footer.

### Database Protocol

- RLS enabled on every table containing tenant-specific data
- Session variable `app.current_tenant_id` set at transaction start
- No raw DB sessions — use the service wrapper that attaches tenant context automatically
- Vector similarity queries: always include tenant filter, similarity threshold ≥ 0.65, limit ≤ 15 chunks
- HNSW index (cosine ops) required once a vector table exceeds 50k active rows

### LLM Client Protocol

- Wrap every LLM call in retry with exponential backoff + jitter (max 3 retries)
- Fallback chain: primary → secondary (different vendor) → dead-letter queue
- Zero data-retention enterprise endpoints only
- System prompt placed in protected position preceding all retrieved content (prompt injection containment)
- Validate structured output against expected schema before writing to database; retry up to 3 times on schema failure

---

## 7. Multi-Modal Input Handling

| Modality | Trust Level | Special Handling |
|---|---|---|
| Plain text (PDF/DOCX) | Baseline | Tables require structured parsing |
| Spreadsheet (XLSX) | High (structured) | Row = potential requirement; column headers = semantic schema |
| Diagram / Image | Moderate | Vision LLM extraction; confidence capped at 0.80; `[VISUAL EXTRACTION — VERIFY]` tag mandatory |
| Audio recording | Moderate | STT transcription → text pipeline; speaker attribution affects trust |
| Chat session | High (recency) | Authoritative for the question asked; may conflict with older docs |
| URL / Web page | Baseline | Mark `source_type: web`; authoritativeness not verifiable |

---

> Chitragupt Architecture Principles • Sprint 0 • May 2026
