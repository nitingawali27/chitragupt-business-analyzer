# Ontology: Entities and Relationships

**Phase:** Product Discovery
**Purpose:** To define the structural grammar, data models, and relationships that Chitragupt will use to parse, understand, and store business requirements.

---

## 1. Core Entities

The system must map unstructured text into the following structured entities.

### 1.1 Source Entities

- **Document:** The root file, URL, or chat session ingested into the system.
- **Chunk:** A semantically contiguous segment of a Document, typically 300–600 tokens, stored in the vector database.
- **Author/Stakeholder:** The human or system that generated the Document.

### 1.2 Knowledge Entities

- **Requirement:** The primary unit of value. A documented need or condition.
  - *Sub-types:* Functional, Non-Functional, Business Rule, Data Requirement.
- **Constraint:** A limiting factor that affects the execution of a requirement (e.g., "Must use AWS," "Budget is $50k").
- **Assumption:** A condition assumed to be true but not explicitly verified.
- **Entity/Actor:** A person, persona, or external system that interacts with the product (e.g., "Admin," "Payment Gateway").

### 1.3 State & Workflow Entities

- **Conflict:** A logical contradiction between two or more Chunks.
- **Gap / Open Question:** A missing piece of knowledge required to complete a Specification.
- **Specification:** The final output document, an aggregated collection of validated Requirements, Constraints, and Assumptions.

## 2. Relationships (The Knowledge Graph)

How the entities interact within the system:

- **Document** `HAS_MANY` **Chunks**
- **Chunk** `SUPPORTS` **Requirement** (Traceability link)
- **Requirement** `AFFECTS` **Actor**
- **Requirement** `CONFLICTS_WITH` **Requirement**
- **Gap** `BLOCKS` **Specification**
- **Stakeholder** `RESOLVES` **Conflict / Gap**

## 3. The "Requirement" Schema

A synthesized Requirement object must eventually conform to this structural standard:

- **ID:** Unique identifier (e.g., `REQ-001`)
- **Type:** (Functional, Non-Functional, etc.)
- **Description:** The synthesized text.
- **Source_Chunks:** Array of vector IDs.
- **Confidence_Score:** Float (0.0 to 1.0).
- **Status:** (Draft, Inferred, Human-Approved, Rejected).
- **Acceptance_Criteria:** Array of boolean conditions.

---

> End of Document • Chitragupt Ontology
