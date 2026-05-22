// Tests for state::session — SessionState construction and default field values.

use chitragupt_state_machine::state::phase::SessionPhase;
use chitragupt_state_machine::state::session::SessionState;
use uuid::Uuid;

fn ids() -> (Uuid, Uuid) {
    (Uuid::new_v4(), Uuid::new_v4())
}

// ─────────────────────────────────────────────────────────────────────────────
// CONSTRUCTION — POSITIVE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_session_starts_in_problem_intake() {
    let (ws, proj) = ids();
    let state = SessionState::new(ws, proj);
    assert_eq!(state.current_phase, SessionPhase::ProblemIntake);
}

#[test]
fn new_session_workspace_id_matches_argument() {
    let (ws, proj) = ids();
    let state = SessionState::new(ws, proj);
    assert_eq!(state.workspace_id, ws);
}

#[test]
fn new_session_project_id_matches_argument() {
    let (ws, proj) = ids();
    let state = SessionState::new(ws, proj);
    assert_eq!(state.project_id, proj);
}

#[test]
fn new_session_has_non_nil_session_id() {
    let (ws, proj) = ids();
    let state = SessionState::new(ws, proj);
    assert_ne!(state.session_id, Uuid::nil());
}

#[test]
fn new_session_session_id_is_version_4() {
    let (ws, proj) = ids();
    let state = SessionState::new(ws, proj);
    // UUIDv4 has version nibble = 4 and variant bits set
    assert_eq!(state.session_id.get_version_num(), 4);
}

// ─────────────────────────────────────────────────────────────────────────────
// PHASE 1 FIELDS — PROBLEM INTAKE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_session_problem_statement_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).problem_statement.is_none());
}

#[test]
fn new_session_business_domain_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).business_domain.is_none());
}

#[test]
fn new_session_primary_goal_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).primary_goal.is_none());
}

// ─────────────────────────────────────────────────────────────────────────────
// PHASE 2 FIELDS — STAKEHOLDER DISCOVERY
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_session_actors_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).actors.is_empty());
}

#[test]
fn new_session_external_systems_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).external_systems.is_empty());
}

#[test]
fn new_session_success_definition_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).success_definition.is_none());
}

#[test]
fn new_session_regulatory_context_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).regulatory_context.is_none());
}

// ─────────────────────────────────────────────────────────────────────────────
// PHASE 3 & 4 FIELDS
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_session_requirements_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).requirements.is_empty());
}

#[test]
fn new_session_constraints_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).constraints.is_empty());
}

#[test]
fn new_session_assumptions_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).assumptions.is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// PHASE 5 FIELDS — ARCHITECTURE ALIGNMENT
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_session_architecture_approach_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).architecture_approach.is_none());
}

#[test]
fn new_session_deployment_environment_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).deployment_environment.is_none());
}

// ─────────────────────────────────────────────────────────────────────────────
// AC AND GATE TRACKING
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_session_ac_met_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).ac_met.is_empty());
}

#[test]
fn new_session_ac_waived_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).ac_waived.is_empty());
}

#[test]
fn new_session_documents_indexed_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).documents_indexed.is_empty());
}

#[test]
fn new_session_upload_gates_resolved_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).upload_gates_resolved.is_empty());
}

#[test]
fn new_session_upload_gates_waived_is_empty() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).upload_gates_waived.is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// OUTPUT ARTIFACTS AND SIGN-OFF
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_session_brd_artifact_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).brd_artifact_id.is_none());
}

#[test]
fn new_session_hld_artifact_is_none() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).hld_artifact_id.is_none());
}

#[test]
fn new_session_client_signature_not_confirmed() {
    let (ws, proj) = ids();
    assert!(!SessionState::new(ws, proj).client_signature_confirmed);
}

// ─────────────────────────────────────────────────────────────────────────────
// COST AND TURN TRACKING
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_session_turn_count_is_zero() {
    let (ws, proj) = ids();
    assert_eq!(SessionState::new(ws, proj).turn_count, 0);
}

#[test]
fn new_session_llm_cost_is_exactly_zero() {
    let (ws, proj) = ids();
    assert_eq!(SessionState::new(ws, proj).total_llm_cost_usd, 0.0_f64);
}

#[test]
fn new_session_cost_is_non_negative() {
    let (ws, proj) = ids();
    assert!(SessionState::new(ws, proj).total_llm_cost_usd >= 0.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// BOUNDARY / EDGE CASES
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn two_sessions_with_same_ids_have_different_session_ids() {
    let (ws, proj) = ids();
    let s1 = SessionState::new(ws, proj);
    let s2 = SessionState::new(ws, proj);
    assert_ne!(s1.session_id, s2.session_id);
}

#[test]
fn session_ids_are_unique_across_many_sessions() {
    let ws = Uuid::new_v4();
    let proj = Uuid::new_v4();
    let sessions: Vec<SessionState> = (0..20).map(|_| SessionState::new(ws, proj)).collect();
    let ids: std::collections::HashSet<Uuid> = sessions.iter().map(|s| s.session_id).collect();
    assert_eq!(ids.len(), 20, "collision in generated session IDs");
}

#[test]
fn shared_workspace_and_project_do_not_affect_session_uniqueness() {
    let ws = Uuid::new_v4();
    let proj = Uuid::new_v4();
    let s1 = SessionState::new(ws, proj);
    let s2 = SessionState::new(ws, proj);
    assert_eq!(s1.workspace_id, s2.workspace_id);
    assert_eq!(s1.project_id, s2.project_id);
    assert_ne!(s1.session_id, s2.session_id);
}

#[test]
fn created_at_and_updated_at_are_equal_on_fresh_session() {
    let (ws, proj) = ids();
    let state = SessionState::new(ws, proj);
    assert_eq!(state.created_at, state.updated_at);
}

#[test]
fn different_workspace_ids_are_preserved_independently() {
    let proj = Uuid::new_v4();
    let ws1 = Uuid::new_v4();
    let ws2 = Uuid::new_v4();
    let s1 = SessionState::new(ws1, proj);
    let s2 = SessionState::new(ws2, proj);
    assert_ne!(s1.workspace_id, s2.workspace_id);
    assert_eq!(s1.project_id, s2.project_id);
}

#[test]
fn nil_workspace_id_is_stored_as_given() {
    // The constructor does not reject a nil UUID — it stores whatever is given
    let nil = Uuid::nil();
    let proj = Uuid::new_v4();
    let state = SessionState::new(nil, proj);
    assert_eq!(state.workspace_id, nil);
}
