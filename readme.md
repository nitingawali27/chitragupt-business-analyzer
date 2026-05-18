# Chitragupt — Business Requirement Analyzer (BRA)

## Overview
**Chitragupt** is an enterprise-grade, RAG-based agentic Business Requirement Analyzer (BRA). Named after the divine record-keeper of Hindu mythology — the keeper of complete, accurate accounts — Chitragupt captures, understands, and transforms fragmented client requirements into structured, traceable specification documents.

## The Problem
Enterprise business analysis is manual, fragmented, and expensive. Business Analysts spend **60–80% of their time** organizing and transcribing requirements rather than analyzing them. Requirements arrive through incompatible channels, change constantly, and are documented inconsistently — leading to rework, delays, and missed scope.

## The Solution
The platform ingests requirements from multiple sources (documents, chat, APIs), processes them through an intelligent NLP and embedding pipeline, stores knowledge in a vector-based enterprise memory, and orchestrates a suite of specialized AI agents to generate:
- Business Requirement Documents (BRDs)
- Functional Requirement Documents (FRDs)
- User Stories
- Gap Analysis Reports
- Requirements Traceability Matrices (RTMs)

All within an interactive, human-in-the-loop conversational interface.

## System Architecture
Chitragupt is built using a modern, AI-native stack:
- **Presentation Layer**: Chat Interface, Document Manager, Spec Reviewer, Admin Console
- **API Gateway**: FastAPI with Azure AD B2C Auth
- **Agent Orchestration**: LangGraph with specialized agents (Planner, Understanding, Clarification, Retrieval, Authoring, Validation, Traceability)
- **RAG Engine**: Azure Document Intelligence, `text-embedding-3-large`, Azure AI Search (Hybrid Dense + BM25 Retriever)
- **Data Layer**: PostgreSQL (Structured), Azure Blob Storage (Documents)
- **AI Services**: Azure OpenAI (GPT-4o)

## Value Proposition
| Dimension | Value Delivered |
|---|---|
| **Speed** | BRD generation time reduced from 3–5 days to < 4 hours |
| **Quality** | Consistent, structured specs with source citations and validation |
| **Coverage** | Institutional knowledge from past projects surfaced automatically |
| **Traceability** | Full chain from business goal → requirement → story → test |
| **Scalability** | Multiple concurrent projects without proportional headcount growth |
| **Governance** | Human-in-the-loop approval gates with full audit trail |

## Documentation
For more detailed information regarding the research and architecture, please refer to the documentation:
- [Chitragupt Research & Architecture](docs/chitragupt-research.md)
