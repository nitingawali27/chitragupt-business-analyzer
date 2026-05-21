# System Invariants

**Phase:** Product Discovery
**Document Version:** 2.0
**Purpose:** To establish the absolute, unbreakable rules of the Chitragupt system. These invariants are architectural constraints that must not be violated by any feature, sprint, agent behavior, or LLM output. They are non-negotiable and take precedence over product requirements, performance optimization, and convenience.

An invariant is different from a guideline. A guideline describes preferred behavior. An invariant describes behavior that, if violated, renders the system's outputs untrustworthy or its operation unsafe. Engineering teams must treat invariant violations as P0 bugs regardless of context.

---

## 1. Data Security and Tenant Isolation

### 1.1 Tenant Isolation Invariant
Under no circumstances can vector embeddings, document chunks, metadata, structured requirement objects, or any generated artifact from Tenant A be queried, retrieved, synthesized, or exposed during a session belonging to Tenant B.

- **Enforcement:** Tenant namespace isolation is enforced at the vector store filter level, the PostgreSQL row-level security (RLS) policy level, and the blob storage container level — independently and redundantly. Isolation is not a single check; it is layered defense-in-depth.
- **Failure Mode:** Any cross-tenant data bleed is a critical security incident. The system must immediately terminate the affected session, invalidate the generated output, and trigger an automated incident alert.

### 1.2 Ephemeral PII Invariant
Chat logs, conversation turns, or any session messages that contain PII (names, emails, phone numbers, national IDs), financial data, credentials, or secrets must be identified and scrubbed before they are embedded into the persistent vector store.

- **Enforcement:** A PII detection pass (using Azure AI Content Safety or equivalent NER-based detection) must run on all text before the embedding step in the ingestion pipeline. Detection failures block ingestion; they never silently allow PII to pass through.

### 1.3 Prompt Injection Containment Invariant
No content from an ingested document, user message, or retrieved chunk may alter, override, or escape the agent's system prompt or tool call boundaries.

- **Enforcement:** All retrieved chunk content and user input must be inserted into the LLM context as data content within delimited blocks, never as raw prompt extensions. The system prompt is defined statically at agent initialization and cannot be modified at runtime by any external input source.
- **Monitoring:** The observability layer (LangSmith) must flag any output that includes system prompt content verbatim, which indicates a potential prompt injection or prompt leakage event.

### 1.4 Secret and Credential Isolation Invariant
LLM API keys, database credentials, blob storage access keys, and all secrets must be loaded exclusively from Azure Key Vault at runtime. No secret may be hardcoded, committed to source control, written to logs, or included in any LLM prompt context.

- **Enforcement:** CI/CD pipeline secret scanning must block any commit that contains credential patterns. Key rotation must be supported without requiring application redeployment.

---

## 2. Epistemological and Traceability Invariants

### 2.1 Traceability Invariant
Every generated requirement, constraint, assumption, or specification section **must** contain a verifiable reference to at least one ingested source chunk stored in the vector store under the current tenant and project namespace.

- **No Exceptions:** Ungrounded, zero-shot generation of business requirements is strictly prohibited under all circumstances, including when the retrieval pipeline returns empty results.
- **Empty Retrieval Response:** If no relevant chunks are retrieved for a given synthesis task, the agent must report a knowledge gap — it must not generate requirements from its parametric (training-time) knowledge to fill the void.

### 2.2 Conflict Non-Resolution Invariant
If the retrieval stage surfaces two or more chunks that make directly contradicting factual claims about the same subject, the Agentic layer **must not** unilaterally decide which is correct.

- **Required Action:** Halt synthesis for that specific requirement topic, create a Conflict Flag knowledge object, place the dependent specification section in BLOCKED state, and surface the Conflict Flag in the Gap Analysis Report.
- **No Confidence Bypass:** A high confidence score on one of the conflicting claims does not grant the agent authority to resolve the conflict autonomously. Confidence measures grounding quality, not truth supremacy.

### 2.3 Confidence Tagging Invariant
Any synthesized output with a composite confidence score below the `CONFIDENCE_TAG_THRESHOLD` (default: 0.85) **must** be visually tagged in the output document.

- **Required Tags:** `[VERIFY]` for scores 0.60–0.84; `[INFERRED - REQUIRES VALIDATION]` for scores 0.40–0.59; `[SPECULATIVE - DO NOT USE]` for scores below 0.40.
- **Tag Removal:** Only a T1 human approval action removes a confidence tag. The LLM, upon re-generation, may not remove a tag it previously applied to its own output.

### 2.4 Orphan Prohibition Invariant
Any claim in a generated output that cannot be traced to at least one stored vector chunk in the current project namespace is classified as an Orphan. Orphaned claims must never appear in a finalized specification without a visible `[UNVERIFIED - SOURCE NOT FOUND]` tag and explicit human approval.

---

## 3. Human-in-the-Loop (HITL) Authority Invariants

### 3.1 Human Override Invariant
A human reviewer's explicit edit, approval, or rejection of an LLM-generated output becomes the absolute ground truth (Tier 1) for that knowledge object.

- **Re-generation Lock:** The system **cannot** overwrite a human edit during any subsequent re-generation cycle. The human-edited content must be injected as a locked constraint in the re-generation system prompt.
- **Preservation:** The original LLM-generated content is never deleted. It is archived with status `SUPERSEDED_BY_HUMAN` and remains accessible in the audit trail for the full retention period.

### 3.2 Manual Trigger Invariant
The system **will not** automatically push, export, or synchronize any generated specification, requirement object, user story, or artifact to any downstream external system (Jira, GitHub, Azure DevOps, Confluence, email, or any other integration) without an explicit and unambiguous human approval action in the Chitragupt review interface.

- **Phase 2 Integrations:** This invariant applies even after Phase 2 integration features are shipped. The human approval gate is structural, not a UI nicety that can be toggled off.

### 3.3 Clarification Gate Invariant
The Clarification Agent must present questions to the human and must not proceed to specification authoring for requirements with confidence below `CLARIFICATION_THRESHOLD` (default: 0.70) without first receiving a human response.

- **Timeout Behavior:** If the human does not respond to clarification questions within the configured session timeout, the affected requirements must be marked with status `PENDING_CLARIFICATION` and excluded from the current specification draft. They must not be synthesized with assumed answers.

### 3.4 Approval Before Export Invariant
No output document (BRD, FRD, user story set, RTM, gap report) may be exported in finalized format until the project contains at least one human-approved requirement object. A specification consisting entirely of Draft (T4) items cannot be exported as a final deliverable — only as a working draft clearly labeled `[DRAFT - PENDING HUMAN REVIEW]`.

---

## 4. RAG Pipeline Invariants

### 4.1 Deterministic Retrieval Invariant
Given the exact same vector database state, the exact same tenant and project scope filters, and the exact same query string, the retrieval phase **must** return the exact same ranked chunk set in the exact same order.

- **Enforcement:** The vector store must be queried with deterministic parameters (no random tie-breaking, no non-deterministic approximate nearest neighbor settings in production mode). Re-ranking models must be run with temperature = 0 or equivalent deterministic mode.
- **Purpose:** Determinism is required for auditability. If a specification can be regenerated and produces different citations each time, the audit trail is invalid.

### 4.2 Hybrid Search Completeness Invariant
Both the dense (vector similarity) leg and the sparse (BM25 keyword) leg of the hybrid search pipeline must be executed for every retrieval query. Neither leg may be disabled, short-circuited, or skipped in production.

- **Reason:** Each retrieval method recovers documents the other misses. Disabling either degrades knowledge completeness and risks missing critical, literally-stated requirements that embed poorly.

### 4.3 Re-ranking Invariant
The Cross-Encoder Re-ranker must be applied to the fused candidate set before any chunk is passed as context to a generation LLM. Raw retrieval score ordering is insufficient for evidence selection in a business-critical synthesis task.

- **Minimum Candidate Set:** The re-ranker must receive a minimum of 20 candidate chunks (or the full result set if fewer are available) before selecting the final top-K context window.

### 4.4 Metadata Filter Enforcement Invariant
Every vector store query must include all of the following mandatory filters: `tenant_id`, `project_id`. Optional filters (e.g., `document_type`, `date_range`) are additive but cannot replace the mandatory set.

- **Failure Behavior:** A query that reaches the vector store without `tenant_id` and `project_id` filters must be rejected by the vector store access layer, not by application logic. The rejection must be logged as a security event.

### 4.5 Embedding Model Consistency Invariant
All chunks within a single vector index namespace must be embedded using the same model version. Mixed-model embeddings within a namespace produce geometrically invalid similarity calculations.

- **Enforcement:** The ingestion pipeline must verify the `embedding_model_version` in namespace metadata before writing new embeddings. If a version mismatch is detected, the ingestion job must fail and alert — it must not write mixed-model embeddings silently.
- **Model Upgrade Path:** Upgrading the embedding model requires full re-ingestion of all documents in the affected namespace. There is no partial migration path.

### 4.6 Chunk Integrity Invariant
Once a chunk is written to the vector store, its text content, embedding vector, and source metadata must be immutable. Chunks may be logically deleted (marked inactive) but never physically overwritten.

- **Reason:** Mutable chunks break the audit trail. A cited chunk must always be retrievable in the exact form it was at the time it was cited.

---

## 5. Agent Behavior Invariants

### 5.1 Stateless Agent Invariant
Core reasoning agents (Planner, Requirement Understanding, Clarification, Retrieval, Authoring, Validation, Traceability) must be stateless between invocations. No agent may store reasoning state, partial results, or session variables in local memory that persists across invocations.

- **State Loading:** All context required for an agent invocation — conversation history, prior requirements, retrieved chunks, human edits — must be explicitly loaded from the session state machine or the persistent store at the start of each invocation.
- **Reason:** Statelessness is required for horizontal scaling, reproducibility, and fault tolerance. An agent that fails mid-run must be restartable from the same inputs without side effects.

### 5.2 Agent Input/Output Contract Invariant
Each agent has a defined input schema and output schema (see Research doc §8.4). An agent must never produce output that violates its output schema, and the orchestrator must validate output structure before passing it to the next agent.

- **Schema Violations:** If an agent produces structurally invalid output, the orchestrator must retry the agent call once with an explicit schema correction instruction before failing the workflow and escalating to human intervention.

### 5.3 Planner Authority Invariant
The Planner Agent is the sole entry point for task decomposition. No individual specialist agent may autonomously invoke another specialist agent directly. All agent-to-agent communication must be mediated by the Planner or the LangGraph state machine.

- **Reason:** Unmediated agent-to-agent calls bypass quality gates, break the audit trail, and make the system's behavior unpredictable and untraceable.

### 5.4 No Autonomous External Tool Invocation Invariant
No agent may invoke an external tool (search engine, external API, code interpreter, filesystem writer) unless that tool is explicitly registered in the agent's tool manifest and its use is logged.

- **Unregistered Tool Calls:** Any attempt by an agent to use an unregistered tool must be blocked by the agent framework and logged as a containment event.

### 5.5 Reflection Loop Bound Invariant
Self-reflection loops (where an agent generates output, evaluates it, and re-generates) must have a hard maximum iteration count (default: 3 cycles) to prevent unbounded token consumption or infinite refinement loops.

- **Behavior at Limit:** If the output has not cleared the quality threshold after the maximum number of reflection cycles, the agent must return its best current output with a `[BELOW THRESHOLD - MANUAL REVIEW REQUIRED]` flag rather than continuing to loop.

---

## 6. AI Model Selection and Usage Invariants

### 6.1 Quality-Critical Path Model Invariant
The Specification Authoring Agent, Validation Agent, and Gap Analysis Agent must never use a model of lower reasoning capability than the designated primary model (GPT-4o or equivalent) without explicit human authorization.

- **Cost Optimization Boundary:** Model tiering for cost optimization is permitted only for low-stakes tasks (query expansion, embedding, short classification). It is never permitted for tasks that directly produce output that a human will review or sign off on.

### 6.2 Model Usage Logging Invariant
Every LLM API call made by any agent must be logged with the following data: model ID, prompt token count, completion token count, temperature setting, timestamp, session ID, agent ID, and the task type. This log is non-negotiable for cost monitoring, auditability, and anomaly detection.

### 6.3 Cost Circuit Breaker Invariant
A per-session and per-project token budget must be configured. If an analysis session exceeds the token budget threshold (default: 80% of budget), the orchestrator must alert the session owner and pause non-critical agent calls. If the session reaches 100% of budget, all LLM calls must halt and the user must be notified.

- **Reason:** Without a circuit breaker, a malformed document or adversarial input could cause runaway LLM API consumption.

### 6.4 No Training Data Submission Invariant
Client requirement documents, conversation turns, generated specifications, and any data ingested into the system must never be submitted to an LLM provider in a way that allows the provider to use that data for model training.

- **Enforcement:** All LLM API usage must be configured with data processing agreements (DPAs) that prohibit training data use. Azure OpenAI with customer-managed keys satisfies this requirement by default; any alternative provider must be verified before use.

---

## 7. Output and Specification Invariants

### 7.1 Citation Completeness Invariant
Every discrete factual claim in every generated output artifact — BRD, FRD, user story, gap report, RTM — must be accompanied by a citation referencing the source chunk(s) that ground the claim.

- **Uncited Claims:** Any output section that contains factual claims without citations must not be included in the final document. It must be placed in the Gap Report as an item requiring additional source documents.

### 7.2 Specification Completeness Gate Invariant
The Authoring Agent must not generate a document that is structurally incomplete (e.g., a BRD missing its Assumptions section, a user story missing Acceptance Criteria). Structural completeness is defined per template.

- **Incomplete Sections:** If insufficient grounding context exists to populate a mandatory section, the section must contain an explicit gap placeholder: `[GAP: Insufficient source material. Provide additional input for this section.]`

### 7.3 INVEST Compliance Invariant (User Stories)
All generated user stories must pass an automated INVEST (Independent, Negotiable, Valuable, Estimable, Small, Testable) compliance check before being included in the output.

- **Non-Compliant Stories:** Stories that fail INVEST are routed to the Validation Agent's reflection loop for revision. Stories that fail INVEST after the maximum reflection cycles are flagged `[NON-INVEST COMPLIANT - REVIEW REQUIRED]` rather than silently included.

### 7.4 Version Immutability Invariant
Once a specification version has been exported or marked as Approved, its content is immutable. No in-place edits may be made to an exported or approved version.

- **Change Process:** All changes after approval must produce a new version with an incremented version number, an audit record of what changed, who changed it, and why.

---

## 8. Performance and Execution Invariants

### 8.1 Async Ingestion Invariant
Document ingestion (parsing, chunking, embedding) must run asynchronously via a message queue (Azure Service Bus) and must never block the API request-response cycle.

- **User Feedback:** The API must immediately return an acknowledgment with a job ID. Clients must poll or receive a webhook notification for completion status.

### 8.2 Latency SLA Invariant
The following latency targets are architectural commitments, not best-effort targets:

| Operation | p95 SLA | Hard Abort Timeout |
|:---|:---|:---|
| Chat response (streaming first token) | < 5 seconds | 30 seconds |
| Vector retrieval | < 500 ms | 5 seconds |
| Full specification generation (20 requirements) | < 90 seconds | 5 minutes |
| Document ingestion (50-page PDF) | < 3 minutes | 10 minutes |

- **Timeout Behavior:** Any operation that exceeds its hard abort timeout must terminate, return an error to the user, and log the event for engineering review. Silent hangs are prohibited.

### 8.3 Idempotent Ingestion Invariant
Re-ingesting the same document (identified by content hash) must not create duplicate chunks in the vector store. The ingestion pipeline must check for existing chunks with the same content hash and update metadata rather than creating duplicates.

- **Change Detection:** If a document is re-ingested and its content has changed (different hash), the old chunks must be marked inactive and new chunks created, preserving the full ingestion history.

---

## 9. Observability and Audit Invariants

### 9.1 Immutable Audit Log Invariant
Every state transition for every knowledge object, every agent invocation, every human action, and every document access must be recorded in an append-only, tamper-evident audit log.

- **Minimum Fields Per Event:** `event_id`, `event_type`, `actor_id` (user or agent), `resource_id`, `resource_type`, `old_state`, `new_state`, `timestamp`, `session_id`, `ip_address` (for human actions).
- **Retention:** Audit logs must be retained for a minimum of 7 years to support SOC 2 and regulatory audit requirements.

### 9.2 Distributed Tracing Invariant
Every cross-service request — from API Gateway to Agent Orchestrator to RAG Engine to LLM API to Data Store — must carry a `trace_id` that allows the full request graph to be reconstructed in the observability platform (OpenTelemetry + LangSmith).

- **LLM Trace Coverage:** LLM calls must be traced end-to-end including prompt content, completion content, token counts, latency, and model parameters. This data is used for hallucination rate calculation and cost attribution.

### 9.3 Retrieval Quality Monitoring Invariant
The system must continuously measure and report retrieval quality metrics: Mean Reciprocal Rank (MRR@10), Normalized Discounted Cumulative Gain (NDCG), and citation precision. These metrics must be available in the engineering dashboard with a per-tenant breakdown.

- **Alert Threshold:** An MRR@10 drop below 0.65 must trigger an automated alert to the AI/ML team. This indicates retrieval degradation that will propagate into lower-quality specifications.

---

## 10. Knowledge Graph Invariants (Phase 3)

These invariants apply when the Neo4j knowledge graph layer is introduced in Phase 3.

### 10.1 Graph Consistency Invariant
Every node in the knowledge graph must correspond to an existing, active knowledge object in either the vector store or PostgreSQL. Orphaned graph nodes (references to deleted or non-existent objects) are prohibited.

- **Enforcement:** Graph writes must be transactional with the corresponding structured store writes. A graph node must not be created unless the backing knowledge object was successfully persisted.

### 10.2 Relationship Traceability Invariant
Every edge in the knowledge graph (e.g., `Requirement CONFLICTS_WITH Requirement`, `Chunk SUPPORTS Requirement`) must carry its own provenance metadata: created by which agent, at what time, based on what evidence.

- **Reason:** Graph edges represent asserted knowledge relationships. They carry the same epistemological burden as any other knowledge claim and must be traceable.

### 10.3 Impact Analysis Soundness Invariant
When the graph is queried for impact analysis (e.g., "What requirements are affected if this constraint changes?"), the graph traversal must only follow edges that have a confidence score above `INCLUSION_THRESHOLD` (0.40). Low-confidence edges must not participate in impact propagation.

---

> End of Document • Chitragupt System Invariants v2.0 • Aligned Automation
