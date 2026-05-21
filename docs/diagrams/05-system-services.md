# 05 — System Services

## What this is

Chitragupt is built from three services, each written in a different language. This is not complexity for its own sake — each language was chosen because it is genuinely the best tool for that job. The three services never share code. They talk to each other over a well-defined protocol (gRPC).

This document explains what each service is responsible for and why that language was chosen — in plain terms.

---

## The Three Services

```mermaid
graph TB
    subgraph USER["What the BA sees"]
        BA[Browser\nChat interface]
    end

    subgraph GO["Go — API Gateway\nThe front door"]
        G1[Accepts all incoming connections]
        G2[Checks identity — JWT tokens]
        G3[Streams responses back to browser]
        G4[Routes to the right internal service]
    end

    subgraph RUST["Rust — State Machine\nThe brain"]
        R1[Knows which phase the session is in]
        R2[Evaluates acceptance criteria]
        R3[Checks upload gates]
        R4[Decides when to advance to next phase]
        R5[Calls Python when AI work is needed]
    end

    subgraph PYTHON["Python — AI Orchestration\nThe intelligence"]
        P1[Classifies what the BA meant]
        P2[Extracts structured facts from conversation]
        P3[Searches uploaded documents]
        P4[Finds the next question to ask]
        P5[Generates the actual response — streamed]
        P6[Processes uploaded documents]
    end

    subgraph DATA["Shared Data"]
        DB[(PostgreSQL\nAll persistent data)]
        CACHE[(Redis\nSession cache + events)]
        FILES[(S3\nUploaded files)]
    end

    BA -->|WebSocket| GO
    GO -->|gRPC call| RUST
    RUST -->|gRPC call| PYTHON
    PYTHON -->|tokens stream back| RUST
    RUST -->|tokens stream back| GO
    GO -->|tokens stream back| BA

    RUST --- DB
    RUST --- CACHE
    PYTHON --- DB
    PYTHON --- FILES
    GO --- CACHE
```

---

## Why Each Language

```mermaid
graph LR
    subgraph RUST_WHY["Why Rust for the State Machine"]
        RW1["The compiler checks every\npossible state transition.\nUnhandled states are build errors,\nnot runtime surprises."]
        RW2["No garbage collector means\nno pauses during AC evaluation.\nEvery session is deterministic."]
        RW3["Memory safety rules prevent\ntwo sessions accidentally\nreading each other's data."]
    end

    subgraph PY_WHY["Why Python for AI"]
        PW1["Every LLM SDK, embedding model,\ndocument parser, and ML library\nis Python-first or Python-only."]
        PW2["LLM calls take 1–5 seconds.\nPython overhead is 10ms.\nIt is irrelevant."]
        PW3["LangGraph — the pipeline\norchestration framework —\nis Python-only."]
    end

    subgraph GO_WHY["Why Go for the API"]
        GW1["Each WebSocket connection\ngets its own goroutine.\n10,000 sessions costs ~20MB.\nPython equivalent: ~500MB."]
        GW2["Go compiles to one binary.\nNo runtime, no packages,\nno virtual environments.\nJust copy and run."]
        GW3["Streaming tokens from Rust\nthrough to the browser\nwithout buffering.\nNative channel model."]
    end
```

---

## How a Message Travels Through the System

```mermaid
sequenceDiagram
    participant BA as BA (Browser)
    participant Go as Go Gateway
    participant Rust as Rust State Machine
    participant Python as Python AI

    BA->>Go: "The admin needs a CSV export"
    Note over Go: Validates JWT<br/>Looks up session

    Go->>Rust: ProcessTurn(session_id, message)
    Note over Rust: Loads session state<br/>Checks upload gates<br/>No gates open

    Rust->>Python: RunPipeline(session_state, message)
    Note over Python: IntentClassifier → EntityExtractor<br/>→ RAG Retrieval → Gap Analyzer<br/>→ Guidance Generator

    loop Tokens stream back
        Python-->>Rust: token
        Rust-->>Go: token
        Go-->>BA: token
    end

    Python-->>Rust: Done(entities, ac_updates)
    Note over Rust: Updates session state<br/>Evaluates AC<br/>Saves to database

    Rust-->>Go: TurnComplete(transition_ready=false)
    Note over BA: Sees the full response<br/>streamed word by word
```

---

## What Each Service Knows — and Doesn't

A key design principle: each service has a single area of authority. It does not reach into another service's domain.

| | Go | Rust | Python |
|---|---|---|---|
| **Knows about** | HTTP, WebSocket, auth, routing | Session state, AC, phase transitions, gates | LLMs, RAG, extraction, document processing |
| **Does not know about** | AC criteria, LLMs, session state | LLM APIs, document parsing, embeddings | State transitions, phase logic, gate rules |
| **Owns in the database** | Nothing (reads session for auth only) | `session` table | `chunk`, `requirement`, `llm_call_log`, `document` tables |
| **What happens if it goes down** | No new connections accepted | Sessions freeze mid-turn | AI responses stop; state machine waits |

This separation means each service can be scaled, upgraded, or replaced independently. A faster embedding model in Python does not require touching Go or Rust. A new gate rule in Rust does not require touching the LLM prompts.

---

## The Communication Protocol

The three services talk to each other using **gRPC** — a strongly typed, high-performance protocol used across the industry for exactly this kind of multi-language system.

```mermaid
flowchart LR
    PROTO[proto/ files\nShared contract\nLanguage-agnostic]

    PROTO -->|generates| GS[Go client code\nauto-generated]
    PROTO -->|generates| RS[Rust server + client\nauto-generated]
    PROTO -->|generates| PS[Python server\nauto-generated]

    GS -->|calls| RS
    RS -->|calls| PS

    NOTE["If the contract changes,\nall three services are\nupdated from one file.\nNo drift. No surprises."]
```

The `.proto` files are the single source of truth for how the services communicate. Every field, every message, every API call is defined there — in one place, version-controlled with the rest of the code.
