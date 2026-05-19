# System Invariants

**Phase:** Product Discovery
**Purpose:** To establish the absolute, unbreakable rules of the Chitragupt system. These invariants serve as architectural constraints that must not be violated by any feature, sprint, or LLM output.

---

## 1. Data Security & Isolation

- **Tenant Isolation Invariant:** Under no circumstances can vector embeddings, chunks, or metadata from Tenant A be queried, retrieved, or synthesized during a session belonging to Tenant B.
- **Ephemeral Chat Invariant:** Chat logs containing PII or sensitive secrets must be scrubbed before being embedded into the vector store.

## 2. Epistemological & Agentic Rules

- **Traceability Invariant:** Every generated requirement, constraint, or assumption **must** contain a verifiable pointer to at least one ingested source chunk. Ungrounded, zero-shot generation of business requirements is strictly prohibited.
- **Conflict Non-Resolution Invariant:** If the retrieval stage surfaces two directly contradicting facts, the Agentic layer **must not** unilaterally decide which is correct. It must halt synthesis for that specific topic and raise a Conflict Flag for human review.
- **Confidence Tagging Invariant:** Any synthesized output with an LLM confidence score below the agreed threshold (e.g., 85%) **must** be visually tagged in the output (e.g., `[INFERRED - VERIFY]`).

## 3. Human-in-the-Loop (HITL) Authority

- **Human Override Invariant:** A human reviewer's explicit edit, approval, or rejection of an LLM-generated output becomes the absolute ground truth. The system **cannot** overwrite a human edit during subsequent re-generation cycles.
- **Manual Trigger Invariant:** The system **will not** automatically push generated specifications to downstream execution tools (like Jira or GitHub) without an explicit human approval action.

## 4. Performance & Execution

- **Deterministic Pipeline Invariant:** Given the exact same vector database state and the exact same user prompt, the retrieval phase **must** return the exact same chunks in the exact same order.
- **Stateless Agent Invariant:** The core reasoning agents must remain stateless between invocations; all memory and context must be explicitly loaded from the vector store or the active session state machine.

---

> End of Document • Chitragupt Invariants
