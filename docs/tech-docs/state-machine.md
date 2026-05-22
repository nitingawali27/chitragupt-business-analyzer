# State Machine Kernel — Technical Reference

**Service:** `services/state-machine` (Rust)
**gRPC port:** `:50051`
**Role:** Single source of truth for session state — what phase is the BA in, are they ready to advance, and what gates are blocking them.

---

## 1. What Problem This Solves

Chitragupt's BA conversation spans up to seven phases and dozens of discrete acceptance criteria. Every user message must answer two questions before the AI can respond:

1. **Where are we?** Which phase is this session in, and has anything changed?
2. **Are we ready to advance?** Have all the measurable criteria been satisfied?

These questions are pure logic — no LLM needed. They run synchronously, in microseconds, on every turn. They must be deterministic: two evaluations of the same session state must always produce the same result.

The state machine kernel is a standalone Rust service that owns this logic entirely. It is the gatekeeper. The Python AI service and the Go API gateway never make state decisions — they ask the kernel.

---

## 2. Architecture Position

```
Client (browser)
      │
      ▼ WebSocket / REST
Go API Gateway  (:8080)
      │
      ▼ gRPC ProcessTurn
Rust State Machine  (:50051)   ◄── owns all session state
      │
      ▼ gRPC RunPipeline
Python AI Orchestration  (:50052)   ◄── owns all LLM calls
```

On every user message:
1. Go receives the WebSocket message and calls `StateEngine.ProcessTurn` on Rust.
2. Rust loads session state (Redis cache → PostgreSQL on miss).
3. Rust evaluates upload gates synchronously. If a hard gate is open, it returns a checkpoint prompt immediately — Python is not called.
4. If gates are clear, Rust calls `AIOrchestration.RunPipeline` on Python with the full session state.
5. Python streams guidance tokens back through Rust through Go to the browser.
6. After Python completes, Rust applies entity updates, re-evaluates AC, writes updated state to PostgreSQL and Redis.

---

## 3. SessionPhase — The Compile-Time Guarantee

The seven phases of the BA journey are modeled as a Rust enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionPhase {
    ProblemIntake,
    StakeholderDiscovery,
    RequirementElicitation,
    ConstraintCapture,
    ArchitectureAlignment,
    ReviewAndSignOff,
    SignedOff,
}
```

**Why an enum, not a string or integer?**

Every `match` on `SessionPhase` in the codebase must handle all seven variants. The compiler rejects any match that is missing a variant. This means:

- A new phase added to the enum will produce a compile error in every place that needs to handle it. You cannot forget to update AC evaluation, transition logic, or display names.
- There is no runtime path where an unknown phase reaches the AC evaluator and returns a default result. The default does not exist.
- Phase identity is checked at compile time. `SessionPhase::StakeholderDiscovery == "stakeholder_discovery"` is a type error, not a silent bug.

This is the primary reason the state machine is in Rust. The Python service's equivalent would be a `Literal` type — checked by mypy but not the interpreter. Rust enforces it in the compiled binary.

### Valid Transitions

```rust
impl SessionPhase {
    pub fn valid_transitions(&self) -> &'static [SessionPhase] {
        match self {
            SessionPhase::ProblemIntake         => &[SessionPhase::StakeholderDiscovery],
            SessionPhase::StakeholderDiscovery  => &[SessionPhase::RequirementElicitation],
            SessionPhase::RequirementElicitation => &[SessionPhase::ConstraintCapture],
            SessionPhase::ConstraintCapture     => &[SessionPhase::ArchitectureAlignment],
            SessionPhase::ArchitectureAlignment => &[SessionPhase::ReviewAndSignOff],
            SessionPhase::ReviewAndSignOff      => &[SessionPhase::SignedOff],
            SessionPhase::SignedOff             => &[],
        }
    }
}
```

`ReviewAndSignOff` can also loop back to `RequirementElicitation` when the BA requests revisions — this is handled by the transition engine checking for BA `REVISIT` intent before the normal forward-only gate.

---

## 4. SessionState — What the Kernel Knows

`SessionState` is the complete picture of a session at a point in time. It is the only object passed between the Rust kernel and other services.

```rust
pub struct SessionState {
    // Identity
    pub session_id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Uuid,

    // Position
    pub current_phase: SessionPhase,
    pub ba_confirmed_transition: bool,

    // Extracted knowledge
    pub problem_statement: Option<String>,
    pub business_domain: Option<String>,
    pub actors: Vec<Actor>,
    pub requirements: Vec<DraftRequirement>,
    pub constraints: Vec<DraftConstraint>,
    pub decision_directions: HashMap<String, String>,

    // AC tracking
    pub ac_met: Vec<String>,
    pub ac_unmet: Vec<String>,
    pub open_questions: Vec<OpenQuestion>,

    // Upload / gate tracking
    pub documents_indexed: Vec<Uuid>,
    pub checkpoint_a_prompted: bool,
    pub checkpoint_b_prompted: bool,
    pub checkpoint_c_prompted: bool,
    pub checkpoint_d_prompted: bool,
    pub regulatory_context: Option<String>,
    pub memory_only_waiver: bool,
    pub compliance_doc_waiver: bool,

    // Artifact tracking
    pub brd_artifact_id: Option<Uuid>,
    pub hld_artifact_id: Option<Uuid>,
    pub client_signature_confirmed: bool,

    // Cost control
    pub session_cost_usd: f64,
    pub turn_count: u32,
}
```

`SessionState` is serialized to PostgreSQL as JSONB. On every successful turn, an atomic write updates the record. Redis caches the state for 5 minutes (refreshed on write) to avoid a database hit on every turn in active sessions.

---

## 5. Acceptance Criteria (AC) System

### What AC Is

Each phase transition has a list of named, discrete criteria that must be `Met` before the transition offer is extended to the BA. These are not fuzzy — each criterion maps to a specific, programmatic check on `SessionState` fields.

### AcResult

```rust
pub struct AcResult {
    pub criteria: Vec<AcCriterion>,  // full list with statuses
    pub gaps: Vec<AcGap>,            // unmet criteria with suggested questions
    pub transition_ready: bool,      // true only if all required criteria are Met
}

pub struct AcGap {
    pub criterion_id: String,
    pub description: String,
    pub suggested_question: String,  // the exact question to ask the BA next
}
```

`transition_ready` is `false` if any required (non-optional) criterion is `Unmet`. Optional criteria (upload-related, prefixed `U`) can be `Waived` — they do not block transition but affect confidence scoring on related entities.

### Per-Phase Evaluators

Each phase has a dedicated evaluator function:

```rust
impl SessionPhase {
    pub fn evaluate_ac(&self, state: &SessionState) -> AcResult {
        match self {
            SessionPhase::ProblemIntake          => ac::s1::evaluate(state),
            SessionPhase::StakeholderDiscovery   => ac::s2::evaluate(state),
            SessionPhase::RequirementElicitation => ac::s3::evaluate(state),
            SessionPhase::ConstraintCapture      => ac::s4::evaluate(state),
            SessionPhase::ArchitectureAlignment  => ac::s5::evaluate(state),
            SessionPhase::ReviewAndSignOff       => ac::s6::evaluate(state),
            SessionPhase::SignedOff              => AcResult::terminal(),
        }
    }
}
```

Every evaluator lives in `src/ac/s{N}.rs`. Adding a new phase requires adding a new file and adding it to this match — the compiler enforces the match is exhaustive.

### Example: Phase 1 AC (PROBLEM_INTAKE)

```rust
// src/ac/s1.rs
pub fn evaluate(state: &SessionState) -> AcResult {
    let mut criteria = vec![];

    criteria.push(check(
        "AC-S1-01",
        "Problem statement captured (min 50 words)",
        state.problem_statement.as_deref()
            .map(|s| s.split_whitespace().count() >= 50)
            .unwrap_or(false),
        "Can you describe the business problem you're trying to solve in a few sentences?",
    ));

    criteria.push(check(
        "AC-S1-02",
        "Business domain identified",
        state.business_domain.is_some(),
        "What industry or business domain does this project operate in?",
    ));

    criteria.push(check(
        "AC-S1-03",
        "Primary goal articulated",
        state.definition_of_success.is_some(),
        "What is the single most important outcome this project needs to achieve?",
    ));

    criteria.push(check(
        "AC-S1-04",
        "At least one pain point captured",
        !state.pain_points.is_empty(),
        "What is the biggest pain point the current process has that this system will fix?",
    ));

    criteria.push(check(
        "AC-S1-05",
        "Scope boundary stated",
        state.scope_boundary_stated,
        "Are there things you already know are explicitly out of scope for this project?",
    ));

    // U1 is optional — waived if BA declined upload; only triggered if brief was referenced
    criteria.push(check_optional(
        "AC-S1-U1",
        "Optional: existing system documentation or brief uploaded",
        state.documents_indexed.len() > 0 || state.memory_only_waiver,
        "Do you have any existing documentation — a brief, a deck, or previous specs — you'd like to share?",
    ));

    AcResult::from_criteria(criteria)
}
```

The `gaps` vector in `AcResult` is ordered by priority — the first gap is what the `GuidanceGenerator` should ask next. The Python service reads the first gap from the result and uses it as the next question.

---

## 6. The Gate System

Gates are the upload enforcement layer. They sit between the AC evaluator and the transition engine and can block a transition even when all conversational AC criteria are met.

### Gate Types

```rust
pub enum GateType {
    Hard,           // Transition unreachable. No waiver. No exceptions.
    RequiredPrompt, // System must ASK. BA may decline. Not asking is a pipeline error.
    Triggered,      // System detected a reference. Asks. BA may decline.
    Recommended,    // System suggests once. BA may ignore entirely.
}
```

### GateManager

```rust
pub struct GateManager;

impl GateManager {
    pub fn open_hard_gates(&self, state: &SessionState) -> Vec<UploadGate> {
        self.all_gates_for_session(state)
            .into_iter()
            .filter(|g| g.gate_type == GateType::Hard && g.is_open)
            .collect()
    }

    pub fn open_gates(&self, state: &SessionState) -> Vec<UploadGate> {
        self.all_gates_for_session(state)
            .into_iter()
            .filter(|g| g.is_open)
            .collect()
    }
}
```

### Current Hard Gates

| Gate ID | Condition | Resolved When |
|---|---|---|
| `GATE-BRD-EXISTS` | `brd_artifact_id` is `None` | BRD generated and persisted to S3 |
| `GATE-HLD-EXISTS` | `hld_artifact_id` is `None` | HLD generated and persisted |
| `GATE-CLIENT-SIGNATURE` | `client_signature_confirmed = false` | Client burns sign-off token |
| `GATE-REGULATED-SOURCE-DOC` | `regulatory_context` set AND `documents_indexed` empty | ≥ 1 document indexed OR explicit waiver recorded |

### Gate Evaluation Order on Every Turn

```
1. Evaluate all Hard gates for current phase
   → If any open: return UploadGatePrompt to BA immediately. Do NOT call Python.

2. Evaluate all RequiredPrompt gates for current phase
   → If the prompt has not been issued this session: insert prompt into guidance output before next AC question.

3. Evaluate all Triggered gates for current phase
   → If trigger condition fired but not resolved: insert prompt into guidance output.

4. Evaluate all conversational AC criteria
   → Surface the first unmet criterion as the next question.
```

Hard gates short-circuit everything. A session in `ReviewAndSignOff` with no BRD artifact will never reach the Python pipeline — the kernel returns the BRD generation prompt directly and Python is not invoked.

---

## 7. Transition Engine

### The Three-Condition Rule

A forward transition from phase A to phase B is approved only when all three are true:

1. **Valid:** B is in `A.valid_transitions()`.
2. **Gates clear:** No hard gates are open for the current phase.
3. **AC met:** `A.evaluate_ac(state).transition_ready == true`.

```rust
pub struct TransitionEngine;

impl TransitionEngine {
    pub fn attempt(
        state: &SessionState,
        target: SessionPhase,
        gate_manager: &GateManager,
    ) -> Result<()> {
        if state.current_phase.is_terminal() {
            return Err(StateError::SessionTerminated);
        }

        // 1. Valid target
        if !state.current_phase.valid_transitions().contains(&target) {
            return Err(StateError::InvalidTransition {
                from: state.current_phase,
                to: target,
            });
        }

        // 2. Hard gates clear
        let open_hard_gates = gate_manager.open_hard_gates(state);
        if !open_hard_gates.is_empty() {
            return Err(StateError::HardGateBlocking {
                gate_count: open_hard_gates.len(),
            });
        }

        // 3. AC met
        let ac = state.current_phase.evaluate_ac(state);
        if !ac.transition_ready {
            return Err(StateError::AcNotMet {
                unmet_count: ac.unmet_count(),
            });
        }

        Ok(())
    }
}
```

`attempt` does not mutate state — it returns `Ok(())` or an error. The caller applies the phase change to `SessionState` on success and writes it to the database.

### BA Confirmation Requirement

Even when `attempt` returns `Ok`, the transition is not executed until the BA sends a `CONFIRM` intent in response to the transition offer. The kernel sets `ba_confirmed_transition = false`, presents the summary, and waits. On the next turn classified as `CONFIRM`, the transition is committed.

If the BA sends `DENY` or any non-confirm intent in response to a transition offer, the kernel clears the `transition_pending` flag and resumes AC gap probing.

---

## 8. gRPC Interface

The kernel exposes four RPCs to other services:

```protobuf
service StateEngine {
  // Primary entry point. Streaming — tokens flow back as they're generated.
  rpc ProcessTurn(TurnRequest) returns (stream TurnEvent);

  // Session lifecycle
  rpc CreateSession(CreateSessionRequest) returns (SessionStateProto);
  rpc GetSessionState(SessionQuery)       returns (SessionStateProto);

  // Called by Go after document ingestion completes.
  // Re-evaluates upload AC; may resolve a hard gate.
  rpc NotifyUploadComplete(UploadCompleteEvent) returns (AcEvalResult);
}
```

### TurnEvent Stream

A single `ProcessTurn` call produces a stream of events. Go forwards each event to the browser as it arrives:

```protobuf
message TurnEvent {
  oneof payload {
    StreamToken      token      = 1;  // LLM guidance token — relay immediately
    UploadGatePrompt gate       = 2;  // Hard gate prompt — display to BA
    TransitionOffer  transition = 3;  // AC met — offer to advance phase
    TurnComplete     complete   = 4;  // Turn finished; includes AC state summary
  }
}
```

The stream emits in order:
1. Zero or one `UploadGatePrompt` (if a hard gate fired — Python not called in this case)
2. Multiple `StreamToken` events (LLM tokens streaming from Python through Rust)
3. Zero or one `TransitionOffer` (if all AC met and BA has not yet confirmed)
4. One `TurnComplete`

### NotifyUploadComplete

When Go finishes uploading a document to S3 and Python finishes ingestion and indexing, Go calls `NotifyUploadComplete`. The kernel re-evaluates all upload AC — this may resolve a previously blocking hard gate and allow the session to advance on the BA's next message.

```protobuf
message UploadCompleteEvent {
  string session_id   = 1;
  string document_id  = 2;
  string checkpoint   = 3;  // "A", "B", "C", or "D"
  string tenant_id    = 4;
}

message AcEvalResult {
  bool   gates_resolved   = 1;  // true if any hard gates changed state
  int32  open_gate_count  = 2;
  int32  met_ac_count     = 3;
  int32  unmet_ac_count   = 4;
}
```

---

## 9. Module Structure

```
services/state-machine/
├── Cargo.toml
├── proto/
│   └── state_engine.proto          gRPC service definition (Rust server + Go client)
└── src/
    ├── main.rs                     Entry point; tokio runtime; gRPC server (Sprint 1 EPIC-1)
    ├── error.rs                    Typed errors: StateError enum
    │
    ├── state/
    │   ├── mod.rs                  Re-exports phase, session, transition
    │   ├── phase.rs                SessionPhase enum — transitions, AC dispatch, display
    │   ├── session.rs              SessionState struct — all session fields
    │   └── transition.rs           TransitionEngine — 3-condition validation
    │
    ├── ac/
    │   ├── mod.rs                  Re-exports all evaluators
    │   ├── result.rs               AcResult, AcCriterion, AcGap, AcStatus types
    │   ├── s1.rs                   AC evaluator: PROBLEM_INTAKE
    │   ├── s2.rs                   AC evaluator: STAKEHOLDER_DISCOVERY
    │   ├── s3.rs                   AC evaluator: REQUIREMENT_ELICITATION
    │   ├── s4.rs                   AC evaluator: CONSTRAINT_CAPTURE
    │   ├── s5.rs                   AC evaluator: ARCHITECTURE_ALIGNMENT
    │   └── s6.rs                   AC evaluator: REVIEW_AND_SIGN_OFF
    │
    └── gates/
        ├── mod.rs                  GateType enum, UploadGate struct
        └── manager.rs              GateManager — evaluates and returns open gates
```

---

## 10. Key Libraries

| Crate | Role |
|---|---|
| `tokio` | Async runtime — all I/O is async |
| `tonic` | gRPC server and client (HTTP/2 + protobuf) |
| `prost` | Protobuf code generation from `.proto` files |
| `sqlx` | Async PostgreSQL with compile-time query checking |
| `serde` / `serde_json` | `SessionState` serialization to/from PostgreSQL JSONB |
| `deadpool-redis` | Redis connection pool for session cache |
| `thiserror` | Typed error hierarchy — `StateError` variants |
| `tracing` + `tracing-subscriber` | Structured, async-aware logging and trace spans |
| `uuid` | `Uuid` type for all IDs — prevents mixing session/tenant/project IDs |
| `anyhow` | Top-level error propagation in `main.rs` |

---

## 11. Why These Design Decisions

### Why Rust for the state machine?

| Decision | Reasoning |
|---|---|
| **Exhaustive match** | Adding a new `SessionPhase` variant breaks the build everywhere the match is incomplete. Impossible to forget to handle a new state. |
| **Zero-GC** | AC evaluation runs synchronously on every turn. No garbage collection pause introduces latency spikes into a pipeline where every millisecond before LLM streaming counts. |
| **Borrow checker** | Prevents data races when Go sends concurrent gRPC requests for the same session. The borrow checker enforces that `SessionState` has a single mutable owner at any time. |
| **Performance** | Evaluating 30+ AC criteria per turn is pure computation. Rust executes this in microseconds; LLM calls take 1–5 seconds. The state machine adds zero perceptible latency. |
| **Type safety** | `Uuid` for all IDs prevents accidentally passing a `session_id` where a `tenant_id` is required. Newtypes catch these bugs at compile time. |

### Why a separate process, not a library?

The state machine could be a Python library called from LangGraph. The reasons it's a separate gRPC service:

- **Language boundary:** The AC evaluation logic must be in Rust for compile-time correctness. A Python library wrapper would lose the exhaustive match guarantee.
- **Independent scaling:** The state machine is CPU-bound (logic) while the AI service is I/O-bound (LLM calls). They scale on different axes.
- **Single authority:** Every path through the system — WebSocket turns, upload completion events, sign-off token burns — goes through one service. There is no race between two callers advancing the same session to different states.
- **Clean testing:** The kernel can be tested with unit tests and integration tests against a real PostgreSQL instance, completely independent of any LLM provider.

### Why gRPC over REST between services?

- **Typed contracts** — `.proto` files are version-controlled. A change to `TurnRequest` that breaks Go or Python is caught when their generated stubs are regenerated.
- **Bidirectional streaming** — `ProcessTurn` streams tokens from Python through Rust to Go without any internal buffering. REST would require polling or chunked encoding; gRPC streams natively.
- **HTTP/2 multiplexing** — multiple concurrent turn requests share a single TCP connection between Go and Rust, and between Rust and Python.

---

## 12. Local Development

### Build the kernel

```bash
# From repo root
cargo build -p state-machine

# Run with trace logging
RUST_LOG=debug cargo run -p state-machine
```

### Protobuf generation

Proto stubs are committed — do not generate at build time. To regenerate after changing `proto/state_engine.proto`:

```bash
# Rust stubs (via build.rs + prost)
cargo build   # prost re-runs automatically when .proto changes

# Go stubs
protoc --go_out=. --go-grpc_out=. proto/state_engine.proto

# Python stubs
python -m grpc_tools.protoc -I. --python_out=. --grpc_python_out=. proto/state_engine.proto
```

Commit all three generated files together. Never leave stubs out of sync across services.

### Adding a new AC criterion

1. Open `src/ac/s{N}.rs` for the relevant phase.
2. Add a `check(...)` or `check_optional(...)` call with a new criterion ID, description, condition, and suggested question.
3. Run `cargo test` — the AC unit tests will catch regressions.
4. Update the AC table in `docs/sprints/sprint1/README.md` for the relevant transition.

### Adding a new hard gate

1. Open `src/gates/manager.rs`.
2. Add a new `UploadGate` to `all_gates_for_session`.
3. Add the gate ID and description to the gate table in this document (Section 6).
4. Add an integration test that verifies the gate fires and blocks the transition.

---

> Chitragupt State Machine Kernel · Technical Reference · Sprint 1 · May 2026
