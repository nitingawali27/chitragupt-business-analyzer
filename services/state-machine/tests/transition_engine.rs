// Tests for state::transition — TransitionEngine::attempt() covering
// invalid targets, terminal state, hard gate blocking, and AC-not-met behaviour.

use chitragupt_state_machine::error::StateError;
use chitragupt_state_machine::gates::manager::GateManager;
use chitragupt_state_machine::state::phase::SessionPhase;
use chitragupt_state_machine::state::session::SessionState;
use chitragupt_state_machine::state::transition::TransitionEngine;
use uuid::Uuid;

fn fresh() -> (SessionState, GateManager) {
    let state = SessionState::new(Uuid::new_v4(), Uuid::new_v4());
    (state, GateManager::new())
}

fn in_phase(phase: SessionPhase) -> (SessionState, GateManager) {
    let mut state = SessionState::new(Uuid::new_v4(), Uuid::new_v4());
    state.current_phase = phase;
    (state, GateManager::new())
}

// ─────────────────────────────────────────────────────────────────────────────
// INVALID TRANSITIONS — NEGATIVE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn problem_intake_to_requirement_elicitation_is_invalid() {
    let (state, gm) = fresh();
    let result = TransitionEngine::attempt(&state, SessionPhase::RequirementElicitation, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn problem_intake_to_constraint_capture_is_invalid() {
    let (state, gm) = fresh();
    let result = TransitionEngine::attempt(&state, SessionPhase::ConstraintCapture, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn problem_intake_to_architecture_alignment_is_invalid() {
    let (state, gm) = fresh();
    let result = TransitionEngine::attempt(&state, SessionPhase::ArchitectureAlignment, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn problem_intake_to_review_and_sign_off_is_invalid() {
    let (state, gm) = fresh();
    let result = TransitionEngine::attempt(&state, SessionPhase::ReviewAndSignOff, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn problem_intake_to_signed_off_is_invalid() {
    let (state, gm) = fresh();
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn problem_intake_to_itself_is_invalid() {
    let (state, gm) = fresh();
    let result = TransitionEngine::attempt(&state, SessionPhase::ProblemIntake, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn stakeholder_discovery_to_problem_intake_is_invalid() {
    let (state, gm) = in_phase(SessionPhase::StakeholderDiscovery);
    let result = TransitionEngine::attempt(&state, SessionPhase::ProblemIntake, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn stakeholder_discovery_to_constraint_capture_is_invalid() {
    let (state, gm) = in_phase(SessionPhase::StakeholderDiscovery);
    let result = TransitionEngine::attempt(&state, SessionPhase::ConstraintCapture, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn constraint_capture_to_review_and_sign_off_is_invalid() {
    let (state, gm) = in_phase(SessionPhase::ConstraintCapture);
    let result = TransitionEngine::attempt(&state, SessionPhase::ReviewAndSignOff, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn constraint_capture_to_signed_off_is_invalid() {
    let (state, gm) = in_phase(SessionPhase::ConstraintCapture);
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn architecture_alignment_to_problem_intake_is_invalid() {
    let (state, gm) = in_phase(SessionPhase::ArchitectureAlignment);
    let result = TransitionEngine::attempt(&state, SessionPhase::ProblemIntake, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn review_and_sign_off_to_problem_intake_is_invalid() {
    let (state, gm) = in_phase(SessionPhase::ReviewAndSignOff);
    let result = TransitionEngine::attempt(&state, SessionPhase::ProblemIntake, &gm);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

// ─────────────────────────────────────────────────────────────────────────────
// TERMINAL STATE — NEGATIVE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn signed_off_cannot_transition_to_itself() {
    let (state, gm) = in_phase(SessionPhase::SignedOff);
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    assert!(matches!(result, Err(StateError::SessionTerminated)));
}

#[test]
fn signed_off_cannot_transition_to_any_other_phase() {
    let gm = GateManager::new();
    let non_terminal = [
        SessionPhase::ProblemIntake,
        SessionPhase::StakeholderDiscovery,
        SessionPhase::RequirementElicitation,
        SessionPhase::ConstraintCapture,
        SessionPhase::ArchitectureAlignment,
        SessionPhase::ReviewAndSignOff,
    ];
    for target in non_terminal {
        let (state, _) = in_phase(SessionPhase::SignedOff);
        let result = TransitionEngine::attempt(&state, target, &gm);
        assert!(
            matches!(result, Err(StateError::SessionTerminated)),
            "SignedOff → {:?} should return SessionTerminated",
            target
        );
    }
}

#[test]
fn terminal_check_takes_precedence_over_invalid_target_check() {
    // SignedOff → ProblemIntake is both terminal and invalid, but terminal fires first
    let (state, gm) = in_phase(SessionPhase::SignedOff);
    let result = TransitionEngine::attempt(&state, SessionPhase::ProblemIntake, &gm);
    assert!(matches!(result, Err(StateError::SessionTerminated)));
}

// ─────────────────────────────────────────────────────────────────────────────
// HARD GATE BLOCKING — POSITIVE (blocking is the correct behaviour)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn review_phase_to_signed_off_blocked_by_hard_gates() {
    // BRD, HLD and client signature gates are all open on a fresh session
    let (state, gm) = in_phase(SessionPhase::ReviewAndSignOff);
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    assert!(matches!(result, Err(StateError::HardGateBlocking { .. })));
}

#[test]
fn hard_gate_blocking_error_reports_at_least_one_gate() {
    let (state, gm) = in_phase(SessionPhase::ReviewAndSignOff);
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    if let Err(StateError::HardGateBlocking { gate_count }) = result {
        assert!(gate_count > 0, "gate_count should be > 0");
    } else {
        panic!("expected HardGateBlocking, got {:?}", result);
    }
}

#[test]
fn hard_gate_check_precedes_ac_check() {
    // Hard gates are global — even ProblemIntake → StakeholderDiscovery is blocked by
    // HardGateBlocking on a fresh session (BRD, HLD, signature all open).
    let (state, gm) = fresh();
    let result = TransitionEngine::attempt(&state, SessionPhase::StakeholderDiscovery, &gm);
    assert!(
        matches!(result, Err(StateError::HardGateBlocking { .. })),
        "expected HardGateBlocking before AcNotMet"
    );
}

#[test]
fn hard_gates_block_every_transition_not_just_final() {
    // Hard gates are evaluated globally, not per-phase — all forward transitions are blocked
    // on a fresh session until BRD, HLD, and signature are resolved.
    let gm = GateManager::new();
    let transitions = [
        (
            SessionPhase::ProblemIntake,
            SessionPhase::StakeholderDiscovery,
        ),
        (
            SessionPhase::StakeholderDiscovery,
            SessionPhase::RequirementElicitation,
        ),
        (
            SessionPhase::RequirementElicitation,
            SessionPhase::ConstraintCapture,
        ),
        (
            SessionPhase::ConstraintCapture,
            SessionPhase::ArchitectureAlignment,
        ),
        (
            SessionPhase::ArchitectureAlignment,
            SessionPhase::ReviewAndSignOff,
        ),
        (SessionPhase::ReviewAndSignOff, SessionPhase::SignedOff),
    ];
    for (from, to) in transitions {
        let (state, _) = in_phase(from);
        let result = TransitionEngine::attempt(&state, to, &gm);
        assert!(
            matches!(result, Err(StateError::HardGateBlocking { .. })),
            "{:?} → {:?} should be HardGateBlocking on a fresh session",
            from,
            to
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AC NOT MET — POSITIVE (AC correctly blocks when gates are clear)
// ─────────────────────────────────────────────────────────────────────────────

fn gates_resolved() -> (SessionState, GateManager) {
    // Hard gates are global; resolve all three so early-phase transitions reach the AC check.
    let mut state = SessionState::new(Uuid::new_v4(), Uuid::new_v4());
    state.brd_artifact_id = Some(Uuid::new_v4());
    state.hld_artifact_id = Some(Uuid::new_v4());
    state.client_signature_confirmed = true;
    (state, GateManager::new())
}

fn in_phase_gates_resolved(phase: SessionPhase) -> (SessionState, GateManager) {
    let (mut state, gm) = gates_resolved();
    state.current_phase = phase;
    (state, gm)
}

#[test]
fn problem_intake_to_stakeholder_discovery_blocked_by_unmet_ac() {
    // Hard gates are global — resolve them so the AC evaluator fires, not the gate check.
    let (state, gm) = gates_resolved();
    let result = TransitionEngine::attempt(&state, SessionPhase::StakeholderDiscovery, &gm);
    assert!(matches!(result, Err(StateError::AcNotMet { .. })));
}

#[test]
fn stakeholder_discovery_to_requirement_elicitation_blocked_by_unmet_ac() {
    let (state, gm) = in_phase_gates_resolved(SessionPhase::StakeholderDiscovery);
    let result = TransitionEngine::attempt(&state, SessionPhase::RequirementElicitation, &gm);
    assert!(matches!(result, Err(StateError::AcNotMet { .. })));
}

#[test]
fn ac_not_met_error_reports_at_least_one_unmet_criterion() {
    let (state, gm) = gates_resolved();
    let result = TransitionEngine::attempt(&state, SessionPhase::StakeholderDiscovery, &gm);
    if let Err(StateError::AcNotMet { unmet_count }) = result {
        assert!(unmet_count > 0, "unmet_count should be > 0");
    } else {
        panic!("expected AcNotMet, got {:?}", result);
    }
}

#[test]
fn review_phase_with_all_gates_resolved_fails_on_ac_not_gates() {
    // Resolve all hard gates so the AC evaluator gets a chance to run
    let mut state = SessionState::new(Uuid::new_v4(), Uuid::new_v4());
    state.current_phase = SessionPhase::ReviewAndSignOff;
    state.brd_artifact_id = Some(Uuid::new_v4());
    state.hld_artifact_id = Some(Uuid::new_v4());
    state.client_signature_confirmed = true;
    // regulatory_context = None → regulated gate not triggered
    let gm = GateManager::new();
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    assert!(
        matches!(result, Err(StateError::AcNotMet { .. })),
        "with all hard gates resolved, failure should be AcNotMet, not HardGateBlocking"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// ERROR PAYLOAD — BOUNDARY
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn invalid_transition_error_carries_correct_from_phase() {
    let (state, gm) = fresh(); // ProblemIntake
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    if let Err(StateError::InvalidTransition { from, .. }) = result {
        assert_eq!(from, SessionPhase::ProblemIntake);
    } else {
        panic!("expected InvalidTransition");
    }
}

#[test]
fn invalid_transition_error_carries_correct_to_phase() {
    let (state, gm) = fresh(); // ProblemIntake
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    if let Err(StateError::InvalidTransition { to, .. }) = result {
        assert_eq!(to, SessionPhase::SignedOff);
    } else {
        panic!("expected InvalidTransition");
    }
}

#[test]
fn state_error_implements_display() {
    let (state, gm) = fresh();
    let result = TransitionEngine::attempt(&state, SessionPhase::SignedOff, &gm);
    if let Err(e) = result {
        let msg = e.to_string();
        assert!(
            !msg.is_empty(),
            "StateError Display should produce a non-empty string"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EDGE / BOUNDARY CASES
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn all_forward_phase_skips_from_problem_intake_are_invalid() {
    let gm = GateManager::new();
    let skipped = [
        SessionPhase::RequirementElicitation,
        SessionPhase::ConstraintCapture,
        SessionPhase::ArchitectureAlignment,
        SessionPhase::ReviewAndSignOff,
        SessionPhase::SignedOff,
    ];
    for target in skipped {
        let (state, _) = fresh();
        let result = TransitionEngine::attempt(&state, target, &gm);
        assert!(
            matches!(result, Err(StateError::InvalidTransition { .. })),
            "ProblemIntake → {:?} should be InvalidTransition",
            target
        );
    }
}

#[test]
fn every_phase_rejects_itself_as_transition_target() {
    let gm = GateManager::new();
    let non_terminal = [
        SessionPhase::ProblemIntake,
        SessionPhase::StakeholderDiscovery,
        SessionPhase::RequirementElicitation,
        SessionPhase::ConstraintCapture,
        SessionPhase::ArchitectureAlignment,
        SessionPhase::ReviewAndSignOff,
    ];
    for phase in non_terminal {
        let (state, _) = in_phase(phase);
        let result = TransitionEngine::attempt(&state, phase, &gm);
        assert!(
            matches!(result, Err(StateError::InvalidTransition { .. })),
            "{:?} → {:?} should be InvalidTransition (self-loop not allowed)",
            phase,
            phase
        );
    }
}

#[test]
fn attempt_is_non_mutating_same_phase_after_blocked_attempt() {
    let (state, gm) = gates_resolved(); // gates resolved so AC check runs, not gate check
    let phase_before = state.current_phase;
    let _ = TransitionEngine::attempt(&state, SessionPhase::StakeholderDiscovery, &gm);
    // attempt() takes &SessionState (immutable ref) — phase cannot have changed
    assert_eq!(state.current_phase, phase_before);
}

#[test]
fn two_sequential_blocked_attempts_return_same_error_type() {
    let (state, gm) = gates_resolved();
    let r1 = TransitionEngine::attempt(&state, SessionPhase::StakeholderDiscovery, &gm);
    let r2 = TransitionEngine::attempt(&state, SessionPhase::StakeholderDiscovery, &gm);
    assert!(matches!(r1, Err(StateError::AcNotMet { .. })));
    assert!(matches!(r2, Err(StateError::AcNotMet { .. })));
}
