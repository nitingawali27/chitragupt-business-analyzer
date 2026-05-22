// Tests for gates::manager — GateManager open/closed state under various session configurations.

use chitragupt_state_machine::gates::manager::GateManager;
use chitragupt_state_machine::state::session::SessionState;
use uuid::Uuid;

fn fresh() -> SessionState {
    SessionState::new(Uuid::new_v4(), Uuid::new_v4())
}

// ─────────────────────────────────────────────────────────────────────────────
// HARD GATES OPEN ON FRESH SESSION — POSITIVE (blocking is correct behaviour)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn fresh_session_brd_gate_is_open() {
    let state = fresh();
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        gates.iter().any(|g| g.id == "GATE-BRD-EXISTS"),
        "BRD gate should be open when brd_artifact_id is None"
    );
}

#[test]
fn fresh_session_hld_gate_is_open() {
    let state = fresh();
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        gates.iter().any(|g| g.id == "GATE-HLD-EXISTS"),
        "HLD gate should be open when hld_artifact_id is None"
    );
}

#[test]
fn fresh_session_client_signature_gate_is_open() {
    let state = fresh();
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        gates.iter().any(|g| g.id == "GATE-CLIENT-SIGNATURE"),
        "client signature gate should be open when client_signature_confirmed = false"
    );
}

#[test]
fn fresh_session_has_exactly_three_open_hard_gates() {
    // BRD, HLD, client signature — regulated gate not triggered (no regulatory context)
    let state = fresh();
    assert_eq!(GateManager::new().open_hard_gates(&state).len(), 3);
}

#[test]
fn fresh_session_open_gates_matches_open_hard_gates() {
    // All defined gates are Hard type, so open_gates() and open_hard_gates() return the same set
    let state = fresh();
    let gm = GateManager::new();
    assert_eq!(
        gm.open_gates(&state).len(),
        gm.open_hard_gates(&state).len()
    );
}

#[test]
fn regulated_session_without_documents_opens_regulated_gate() {
    let mut state = fresh();
    state.regulatory_context = Some("GDPR, HIPAA".to_string());
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        gates.iter().any(|g| g.id == "GATE-REGULATED-SOURCE-DOC"),
        "regulated gate should be open for non-empty regulatory context with no documents"
    );
}

#[test]
fn regulated_session_without_documents_has_four_open_hard_gates() {
    let mut state = fresh();
    state.regulatory_context = Some("SOC2".to_string());
    assert_eq!(GateManager::new().open_hard_gates(&state).len(), 4);
}

// ─────────────────────────────────────────────────────────────────────────────
// GATE RESOLUTION — POSITIVE (gate correctly closes when condition is met)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn setting_brd_artifact_resolves_brd_gate() {
    let mut state = fresh();
    state.brd_artifact_id = Some(Uuid::new_v4());
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        !gates.iter().any(|g| g.id == "GATE-BRD-EXISTS"),
        "BRD gate should be closed when brd_artifact_id is Some"
    );
}

#[test]
fn setting_hld_artifact_resolves_hld_gate() {
    let mut state = fresh();
    state.hld_artifact_id = Some(Uuid::new_v4());
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        !gates.iter().any(|g| g.id == "GATE-HLD-EXISTS"),
        "HLD gate should be closed when hld_artifact_id is Some"
    );
}

#[test]
fn confirming_client_signature_resolves_signature_gate() {
    let mut state = fresh();
    state.client_signature_confirmed = true;
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        !gates.iter().any(|g| g.id == "GATE-CLIENT-SIGNATURE"),
        "client signature gate should be closed when client_signature_confirmed = true"
    );
}

#[test]
fn regulated_session_with_one_document_resolves_regulated_gate() {
    let mut state = fresh();
    state.regulatory_context = Some("GDPR".to_string());
    state.documents_indexed.push(Uuid::new_v4());
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        !gates.iter().any(|g| g.id == "GATE-REGULATED-SOURCE-DOC"),
        "regulated gate should close once at least one document is indexed"
    );
}

#[test]
fn non_regulated_session_regulated_gate_is_never_open() {
    // regulatory_context = None ⟹ is_regulated = false ⟹ gate is closed
    let state = fresh();
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(
        !gates.iter().any(|g| g.id == "GATE-REGULATED-SOURCE-DOC"),
        "regulated gate should not appear when there is no regulatory context"
    );
}

#[test]
fn fully_resolved_session_has_no_open_hard_gates() {
    let mut state = fresh();
    state.brd_artifact_id = Some(Uuid::new_v4());
    state.hld_artifact_id = Some(Uuid::new_v4());
    state.client_signature_confirmed = true;
    // regulatory_context = None — regulated gate not triggered
    assert!(GateManager::new().open_hard_gates(&state).is_empty());
}

#[test]
fn fully_resolved_session_has_no_open_gates() {
    let mut state = fresh();
    state.brd_artifact_id = Some(Uuid::new_v4());
    state.hld_artifact_id = Some(Uuid::new_v4());
    state.client_signature_confirmed = true;
    assert!(GateManager::new().open_gates(&state).is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// PARTIAL RESOLUTION — NEGATIVE (other gates stay open)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn resolving_brd_gate_does_not_resolve_hld_gate() {
    let mut state = fresh();
    state.brd_artifact_id = Some(Uuid::new_v4());
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(gates.iter().any(|g| g.id == "GATE-HLD-EXISTS"));
}

#[test]
fn resolving_hld_gate_does_not_resolve_signature_gate() {
    let mut state = fresh();
    state.hld_artifact_id = Some(Uuid::new_v4());
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(gates.iter().any(|g| g.id == "GATE-CLIENT-SIGNATURE"));
}

#[test]
fn resolving_all_but_signature_leaves_one_open_hard_gate() {
    let mut state = fresh();
    state.brd_artifact_id = Some(Uuid::new_v4());
    state.hld_artifact_id = Some(Uuid::new_v4());
    // client_signature_confirmed stays false
    assert_eq!(GateManager::new().open_hard_gates(&state).len(), 1);
}

// ─────────────────────────────────────────────────────────────────────────────
// GATE PROPERTIES
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn open_hard_gates_all_have_non_empty_ids() {
    let state = fresh();
    for gate in GateManager::new().open_hard_gates(&state) {
        assert!(!gate.id.is_empty());
    }
}

#[test]
fn open_hard_gates_all_have_non_empty_descriptions() {
    let state = fresh();
    for gate in GateManager::new().open_hard_gates(&state) {
        assert!(
            !gate.description.is_empty(),
            "gate {} has empty description",
            gate.id
        );
    }
}

#[test]
fn open_hard_gates_all_have_non_empty_resolution_prompts() {
    let state = fresh();
    for gate in GateManager::new().open_hard_gates(&state) {
        assert!(
            !gate.resolution_prompt.is_empty(),
            "gate {} has empty resolution_prompt",
            gate.id
        );
    }
}

#[test]
fn all_open_hard_gates_report_is_open_true() {
    let state = fresh();
    for gate in GateManager::new().open_hard_gates(&state) {
        assert!(
            gate.is_open,
            "gate {} is in open_hard_gates but is_open = false",
            gate.id
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EDGE / BOUNDARY CASES
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn empty_string_regulatory_context_does_not_trigger_regulated_gate() {
    // Some("") means regulatory_context is set but contains no text — not regulated
    let mut state = fresh();
    state.regulatory_context = Some(String::new());
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(!gates.iter().any(|g| g.id == "GATE-REGULATED-SOURCE-DOC"));
}

#[test]
fn whitespace_only_regulatory_context_is_treated_as_regulated() {
    // "  " is non-empty → is_regulated = true
    let mut state = fresh();
    state.regulatory_context = Some("  ".to_string());
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(gates.iter().any(|g| g.id == "GATE-REGULATED-SOURCE-DOC"));
}

#[test]
fn multiple_documents_still_resolves_regulated_gate() {
    let mut state = fresh();
    state.regulatory_context = Some("PCI-DSS, ISO27001".to_string());
    for _ in 0..5 {
        state.documents_indexed.push(Uuid::new_v4());
    }
    let gates = GateManager::new().open_hard_gates(&state);
    assert!(!gates.iter().any(|g| g.id == "GATE-REGULATED-SOURCE-DOC"));
}

#[test]
fn gate_manager_default_equals_gate_manager_new() {
    let state = fresh();
    let gm_new = GateManager::new();
    let gm_default = GateManager;
    assert_eq!(
        gm_new.open_hard_gates(&state).len(),
        gm_default.open_hard_gates(&state).len()
    );
}

#[test]
fn open_hard_gates_is_idempotent_for_same_state() {
    let state = fresh();
    let gm = GateManager::new();
    let first = gm.open_hard_gates(&state);
    let second = gm.open_hard_gates(&state);
    assert_eq!(first.len(), second.len());
    let first_ids: Vec<&str> = first.iter().map(|g| g.id.as_str()).collect();
    let second_ids: Vec<&str> = second.iter().map(|g| g.id.as_str()).collect();
    assert_eq!(first_ids, second_ids);
}

#[test]
fn regulated_gate_requires_both_context_and_no_documents_to_open() {
    // Only context → open
    let mut state_context_only = fresh();
    state_context_only.regulatory_context = Some("HIPAA".to_string());

    // Only documents (no context) → closed
    let mut state_docs_only = fresh();
    state_docs_only.documents_indexed.push(Uuid::new_v4());

    // Both context and documents → closed
    let mut state_both = fresh();
    state_both.regulatory_context = Some("HIPAA".to_string());
    state_both.documents_indexed.push(Uuid::new_v4());

    let gm = GateManager::new();
    let open_context = gm
        .open_hard_gates(&state_context_only)
        .iter()
        .any(|g| g.id == "GATE-REGULATED-SOURCE-DOC");
    let open_docs = gm
        .open_hard_gates(&state_docs_only)
        .iter()
        .any(|g| g.id == "GATE-REGULATED-SOURCE-DOC");
    let open_both = gm
        .open_hard_gates(&state_both)
        .iter()
        .any(|g| g.id == "GATE-REGULATED-SOURCE-DOC");

    assert!(
        open_context,
        "gate should open with context and no documents"
    );
    assert!(
        !open_docs,
        "gate should not open with documents but no context"
    );
    assert!(
        !open_both,
        "gate should close when both context and documents are present"
    );
}
