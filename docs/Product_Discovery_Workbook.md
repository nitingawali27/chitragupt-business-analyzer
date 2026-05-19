# PRODUCT DISCOVERY WORKBOOK: Business Requirement Analyzer (BRA)

**Subtitle:** First-Principles Sprint Planning & Stakeholder Questionnaire

## Document Info

| Details | Value |
| :--- | :--- |
| **Phase** | Product Discovery & Sprint Definition |
| **Audience** | Product Managers, Stakeholders, Solutions Architects |
| **Purpose** | To define the ontological foundations, system invariants, and functional queries required to confidently plan the first set of engineering sprints. |

---

## PURPOSE

This document serves as the primary artifact for the **Discovery Phase** of the Business Requirement Analyzer (Chitragupt). Before defining user-facing features, the product team must align with stakeholders on the *first principles* of the system: its ontology (what concepts exist), its invariants (what must always be true), and its boundaries (what the system will explicitly not do).

This approach ensures that initial sprints build a resilient, structurally sound data model and knowledge graph before layering on the agentic orchestration.

---

## 1. First Principles Definition (Sprints 0–1 Focus)

The earliest sprints must focus on establishing the "ground truth" architecture. The following areas need immediate stakeholder consensus.

### 1.1 Ontology (Entities & Relationships)

We must define the structural grammar the LLM will use to understand business requirements.

| Entity Type | Definition Needed from Stakeholders |
| :--- | :--- |
| **Requirement Objects** | How do we categorize inputs? (e.g., *Functional*, *Non-Functional*, *Constraint*, *Assumption*). Are there sub-types? |
| **Actors & Roles** | Who are the entities interacting with the requirements? (e.g., *Admin*, *Client*, *System*, *Third-Party API*). |
| **Traceability Nodes** | How do we define a "source"? Is a Slack thread treated the same as a PDF document? |
| **Output Artifacts** | What constitutes a "Specification"? Is it a monolithic document, or a collection of Epics and Stories? |

### 1.2 System Invariants (The "Unbreakables")

Invariants are rules that must hold true at all times regardless of the state of the system or the behavior of the agentic RAG pipeline.

- **Traceability Invariant:** Every generated requirement **must** map to at least one ingested source chunk. (Zero-shot ungrounded generation is prohibited).
- **Conflict Invariant:** If two sources contradict each other, the system **must not** auto-resolve the conflict. It must raise a "Conflict Flag" for human resolution.
- **Tenant Isolation Invariant:** Vector embeddings and chunks from Client A **cannot** be retrieved during a session for Client B under any circumstances.
- **Override Invariant:** A human reviewer's explicit override or edit to a requirement acts as the absolute ground truth for future generations.

### 1.3 Boundary Constraints

What the system is *not* responsible for in this phase:

- It will **not** automatically execute code or deploy infrastructure.
- It will **not** generate UI/UX wireframes.
- It will **not** handle real-time synchronous voice transcriptions (yet).

---

## 2. Stakeholder Discovery Questionnaire

Product teams should use these specific queries during stakeholder interviews to extract the necessary requirements for Sprint Planning.

### Section A: Ontological & Data Questions

> *Goal: Understand the shape of the data before designing the vector schema.*

1. **Taxonomy:** When your team writes a specification today, what are the absolute mandatory fields? (e.g., ID, Priority, Description, Acceptance Criteria).
2. **Definition of Done:** What makes a requirement "complete"? (e.g., "It must have at least one positive and one negative test case").
3. **Data Hierarchy:** Do requirements map one-to-one to Epics? Or can one requirement spawn multiple User Stories?

### Section B: Ingestion & Boundary Questions

> *Goal: Understand what the Ingestion Agent must support on Day 1.*

1. **Format Prioritization:** If you could only support 3 input formats for the MVP, what are they? (e.g., PDFs, Jira links, free-text chat).
2. **Volume:** What is the average size of a requirements dump? (e.g., 5 pages, 50 pages, 500 pages?)
3. **Staleness:** If a Confluence page changes *after* it was ingested, should the system automatically re-ingest it, or wait for manual human triggering?

### Section C: Agentic Behavior & Human-in-the-Loop

> *Goal: Define the guardrails for the LLM.*

1. **Confidence Thresholds:** If the AI is only 70% confident in a synthesized requirement, should it include it with an `[INFERRED]` tag, or omit it entirely and ask the human a clarifying question in chat?
2. **Elicitation Aggressiveness:** How proactive should the chat agent be? Should it ask 10 clarifying questions at once, or drip them one by one as the human answers?
3. **Output Formats:** How do you want the final document delivered? (e.g., Markdown file, exported Word doc, or pushed directly via API to Jira?)

---

## 3. Recommended Sprint Phasing

Based on the first-principles approach, the product backlog should follow this trajectory:

### Sprint 0: Foundation & Ontology

- **Objective:** Finalize data models, vector schema, and system invariants.
- **Key Deliverables:**
  - Database schema for documents, chunks, and generated requirements.
  - Definition of the embedding strategy (chunk size, overlap).
  - Setup of tenant-isolated vector store environments.

### Sprint 1: Core Ingestion & Retrieval (The RAG Baseline)

- **Objective:** Build the pipeline that turns files into searchable knowledge.
- **Key Deliverables:**
  - Document Parser (PDF, TXT, MD).
  - Chunker and Embedding generation.
  - Semantic Search API (ensure traceability links remain intact).

### Sprint 2: Synthesis & Elicitation (The Agentic Layer)

- **Objective:** Introduce the LLM to reason over the retrieved chunks.
- **Key Deliverables:**
  - Synthesis Agent (Drafts requirements from retrieved chunks).
  - Elicitation Agent (Identifies missing info and prompts user in chat).
  - Implementation of Confidence tags and the Conflict Resolution flag.

### Sprint 3: Output & Review Loop

- **Objective:** Format the output and allow human overrides.
- **Key Deliverables:**
  - Spec Writer Agent (Assembles the final document).
  - Human Review Interface (Approve, Reject, or Edit).
  - Re-generation loop based on human feedback.

---

> End of Discovery Workbook • Chitragupt • Discovery Phase
