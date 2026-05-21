# Key Decisions — Sprint 0

**Purpose:** Consolidated record of all foundational architectural and technology decisions. Replaces the 15 individual ADR files. Each decision tracks the question, meaningful options with tradeoffs, decision criteria, and current status.

**Status legend:** `OPEN` = unevaluated, `GUIDED` = BA/PM directional input received, `DECIDED` = engineering sign-off complete, `DEFERRED` = explicitly postponed.

---

## 1. Core Runtime

### D-001 — Application Language & Runtime

**Question:** What language and runtime underpins all backend services?

| Option | Tradeoff |
|---|---|
| Python 3.11+ | Best-in-class agentic/ML ecosystem (LangGraph, LangChain, HuggingFace). Slower raw throughput but rarely the bottleneck in LLM-bound workloads. |
| TypeScript (Node) | Strong async I/O; weaker ML ecosystem; interop with Python services adds complexity. |
| Go | Best performance; minimal ML library support; agentic patterns require significant custom work. |

**Decision criteria:** Ecosystem maturity for LLM orchestration, HITL graph state management, and RAG pipeline tooling.

**Status:** `OPEN`

---

## 2. AI/ML Layer

### D-002 — LLM Model Selection & Tier Assignment

**Question:** Which LLM models are assigned to which reasoning tiers, and from which providers?

**Tier model:**

| Tier | Role | Candidate Models |
|---|---|---|
| Premium | Complex synthesis, BRD generation | Claude Opus, GPT-4o, Gemini Ultra |
| Standard | Requirement extraction, classification | Claude Sonnet, GPT-4o-mini, Gemini Pro |
| Fast | Routing, tagging, short extractions | Claude Haiku, GPT-4o-mini, Gemini Flash |
| Fallback | Cross-vendor failover | Must be different vendor from Primary |

**Decision criteria:** Structured output reliability, reasoning depth for requirements synthesis, context window (long documents), cost per 1M tokens, zero data-retention enterprise tier availability.

**Status:** `OPEN`

---

### D-003 — Embedding Model

**Question:** Which embedding model and vector dimension is used for all semantic search?

**Critical constraint:** This decision cannot be changed post-launch without a full re-embedding of all tenant data. Choose deliberately.

| Option | Dimension | Tradeoff |
|---|---|---|
| text-embedding-3-large (OpenAI) | 1536 or 3072 | Strong baseline; OpenAI vendor dependency |
| voyage-large-2 (Voyage AI) | 1536 | Top retrieval benchmarks; smaller vendor |
| Cohere embed-v3 | 1024 | Good multilingual; cost-competitive |
| AWS Titan Embeddings | 1536 | AWS-native; simpler if deploying on AWS |

**Decision criteria:** Retrieval quality on domain-specific (business requirements) text, vendor lock-in risk, dimension compatibility with chosen vector store.

**Status:** `OPEN`

---

### D-004 — Retrieval Strategy

**Question:** How are relevant chunks found for each query — dense, sparse, or hybrid?

| Option | Tradeoff |
|---|---|
| Dense only (vector similarity) | Simple; misses exact keyword matches; good for semantic intent |
| Sparse only (BM25 / keyword) | Good for exact terms, version numbers, named entities; misses paraphrase |
| Hybrid + re-ranking | Best recall; higher latency; requires a cross-encoder re-ranker model |

**Decision criteria:** Recall on business requirement documents (which mix semantic and exact-match content), acceptable latency budget, infrastructure complexity.

**Status:** `OPEN`

---

## 3. Data Layer

### D-005 — Database Architecture

**Question:** Single PostgreSQL + pgvector instance or split relational and vector databases?

| Option | Tradeoff |
|---|---|
| PostgreSQL + pgvector | Unified ACID + vector search; single operational surface; RLS enforced in one place; simpler multi-tenancy |
| PostgreSQL + Pinecone/Weaviate | Best-in-class vector performance; two systems to operate, two RLS boundaries to maintain |
| PostgreSQL + Qdrant (self-hosted) | Open source vector DB; operational overhead; strong filtering support |

**Decision criteria:** Multi-tenancy enforcement at vector layer, operational complexity budget, query latency at scale (target: <500ms p95 semantic search).

**Status:** `OPEN`

---

### D-006 — Caching Layer

**Question:** What caching infrastructure handles session state, idempotency keys, and rate-limit counters?

| Option | Tradeoff |
|---|---|
| Redis (managed — Upstash, ElastiCache) | Industry standard; rich data structures; pub/sub for real-time events |
| DragonflyDB | Redis-compatible; better throughput; smaller ecosystem |
| In-process (no external cache) | Zero infra cost; not horizontally scalable |

**Decision criteria:** Horizontal scalability of API tier, session persistence across restarts, real-time notification support.

**Status:** `OPEN`

---

### D-007 — Object Storage

**Question:** Where are raw uploaded documents stored?

| Option | Tradeoff |
|---|---|
| AWS S3 | Mature; WORM support; strong egress cost at scale |
| Cloudflare R2 | S3-compatible; zero egress cost; newer |
| GCP Cloud Storage | Good EU residency options; GCP-native |
| Azure Blob Storage | Strong EU residency; better for enterprises already on Azure |

**Decision criteria:** Data residency compliance options (EU), egress cost, path-based tenant isolation, WORM compliance for regulated workspaces.

**Status:** `OPEN`

---

## 4. Infrastructure

### D-008 — Deployment Platform

**Question:** Where do API servers and async ingestion workers run?

| Option | Tradeoff |
|---|---|
| AWS ECS Fargate | Managed containers; no cluster ops; good cold-start for API (<2s) |
| AWS Lambda | Cheapest for sporadic load; cold-start problematic for streaming responses |
| AWS EKS | Full control; significant ops overhead; right for >10 services |
| Google Cloud Run | Simpler than EKS; good EU regions; GCP lock-in |

**Decision criteria:** Cold-start latency for streaming API responses, worker isolation for long-running ingestion tasks, operational overhead budget for early stage.

**Status:** `OPEN`

---

### D-009 — Observability Stack

**Question:** How are LLM costs, traces, and quality metrics tracked?

| Option | Tradeoff |
|---|---|
| Langfuse (self-hosted or cloud) | Open source; LLM-native cost attribution; GDPR-friendly self-host option |
| LangSmith | LangChain-native; good tracing; vendor lock-in |
| Helicone | Simple proxy; lightweight; less agentic tracing depth |
| Custom (OpenTelemetry + Grafana) | Full control; significant build effort |

**Decision criteria:** LLM-native cost attribution per project/agent, budget alerting integration, data residency options.

**Status:** `OPEN`

---

### D-010 — CI/CD Pipeline

**Question:** What pipeline runs tests, builds containers, and deploys?

| Option | Tradeoff |
|---|---|
| GitHub Actions | Native GitHub integration; generous free tier; sufficient for most workloads |
| GitLab CI | Better built-in container registry; requires self-hosted or GitLab SaaS |
| CircleCI | Mature; good parallelism; additional vendor |

**Decision criteria:** Container build support, parallel test execution, cost within startup budget.

**Status:** `OPEN`

---

## 5. Identity & Integrations

### D-011 — Authentication Provider

**Question:** How are users authenticated and how are API keys managed?

| Option | Tradeoff |
|---|---|
| Auth0 | Feature-rich; SAML + OIDC + MFA; enterprise pricing at scale |
| AWS Cognito | Cost-effective; tighter AWS integration; complex configuration |
| Clerk | Modern DX; good social + enterprise auth; US-only data by default |
| Self-built (FastAPI + JWT) | Full control; significant security implementation burden |

**Decision criteria:** SAML 2.0 support (enterprise clients), custom JWT claims (tenant_id, plan), API key management, EU data residency option.

**Status:** `OPEN`

---

### D-012 — Connector Integrations (MVP Set)

**Question:** Which inbound and outbound connector platforms are in scope for MVP?

**Inbound candidates:** Jira, Confluence, Notion, Google Docs/Drive, GitHub, Linear, SharePoint, Slack

**Outbound candidates:** Jira (story creation), Confluence (spec publish), Notion, GitHub Issues, Linear, file export (DOCX/PDF/Markdown)

**Decision criteria:** Client demand, OAuth complexity, rate limit risk, BA team's existing tooling.

**Status:** `OPEN` — PM decision required.

---

### D-013 — Email Delivery

**Question:** What service sends budget alerts, notifications, and sign-off requests?

| Option | Tradeoff |
|---|---|
| AWS SES | Cheapest at volume; requires domain setup; deliverability work needed |
| SendGrid | Good deliverability; generous free tier; straightforward API |
| Postmark | Excellent deliverability; transactional-only focus |

**Decision criteria:** Budget cap alert delivery reliability, EU data residency, developer setup simplicity.

**Status:** `OPEN`

---

### D-014 — Speech-to-Text (Deferred)

**Question:** Which STT service handles stakeholder voice recordings uploaded at elicitation checkpoints?

**Status:** `DEFERRED` — Not required for MVP. Re-evaluate at Sprint 2.

Candidates for future evaluation: AssemblyAI (best diarisation), Deepgram (fast + cost), AWS Transcribe (AWS-native).

---

## Decision Governance

- Decisions move from `OPEN` → `GUIDED` when the BA/PM session (Phase 5 of BA HITL Flow) yields a directional answer.
- Decisions move from `GUIDED` → `DECIDED` when engineering validates the choice against performance and cost criteria.
- Once `DECIDED`, a decision is locked. Changes require creating a new decision entry (D-XXX supersedes D-YYY) rather than editing this record.
- All decisions must reach `DECIDED` status before Sprint 0 closes.

---

> Chitragupt Key Decisions • Sprint 0 • May 2026
