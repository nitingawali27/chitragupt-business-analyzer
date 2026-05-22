// Tests for state::phase — SessionPhase enum transitions, terminal state, display names, and AC dispatch.

use chitragupt_state_machine::state::phase::SessionPhase;
use chitragupt_state_machine::state::session::SessionState;
use uuid::Uuid;

fn fresh_state() -> SessionState {
    SessionState::new(Uuid::new_v4(), Uuid::new_v4())
}

// ─────────────────────────────────────────────────────────────────────────────
// VALID TRANSITIONS — POSITIVE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn problem_intake_has_exactly_one_valid_transition() {
    assert_eq!(SessionPhase::ProblemIntake.valid_transitions().len(), 1);
}

#[test]
fn problem_intake_advances_to_stakeholder_discovery() {
    let t = SessionPhase::ProblemIntake.valid_transitions();
    assert!(t.contains(&SessionPhase::StakeholderDiscovery));
}

#[test]
fn stakeholder_discovery_has_exactly_one_valid_transition() {
    assert_eq!(
        SessionPhase::StakeholderDiscovery.valid_transitions().len(),
        1
    );
}

#[test]
fn stakeholder_discovery_advances_to_requirement_elicitation() {
    let t = SessionPhase::StakeholderDiscovery.valid_transitions();
    assert!(t.contains(&SessionPhase::RequirementElicitation));
}

#[test]
fn requirement_elicitation_advances_to_constraint_capture() {
    let t = SessionPhase::RequirementElicitation.valid_transitions();
    assert!(t.contains(&SessionPhase::ConstraintCapture));
}

#[test]
fn constraint_capture_advances_to_architecture_alignment() {
    let t = SessionPhase::ConstraintCapture.valid_transitions();
    assert!(t.contains(&SessionPhase::ArchitectureAlignment));
}

#[test]
fn architecture_alignment_advances_to_review_and_sign_off() {
    let t = SessionPhase::ArchitectureAlignment.valid_transitions();
    assert!(t.contains(&SessionPhase::ReviewAndSignOff));
}

#[test]
fn review_and_sign_off_advances_to_signed_off() {
    let t = SessionPhase::ReviewAndSignOff.valid_transitions();
    assert!(t.contains(&SessionPhase::SignedOff));
}

#[test]
fn signed_off_has_no_valid_transitions() {
    assert!(SessionPhase::SignedOff.valid_transitions().is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// INVALID / SKIPPED TRANSITIONS — NEGATIVE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn problem_intake_cannot_skip_to_requirement_elicitation() {
    let t = SessionPhase::ProblemIntake.valid_transitions();
    assert!(!t.contains(&SessionPhase::RequirementElicitation));
}

#[test]
fn problem_intake_cannot_skip_to_constraint_capture() {
    let t = SessionPhase::ProblemIntake.valid_transitions();
    assert!(!t.contains(&SessionPhase::ConstraintCapture));
}

#[test]
fn problem_intake_cannot_skip_to_architecture_alignment() {
    let t = SessionPhase::ProblemIntake.valid_transitions();
    assert!(!t.contains(&SessionPhase::ArchitectureAlignment));
}

#[test]
fn problem_intake_cannot_skip_to_signed_off() {
    let t = SessionPhase::ProblemIntake.valid_transitions();
    assert!(!t.contains(&SessionPhase::SignedOff));
}

#[test]
fn problem_intake_cannot_stay_in_same_phase() {
    let t = SessionPhase::ProblemIntake.valid_transitions();
    assert!(!t.contains(&SessionPhase::ProblemIntake));
}

#[test]
fn stakeholder_discovery_cannot_go_backwards_to_problem_intake() {
    let t = SessionPhase::StakeholderDiscovery.valid_transitions();
    assert!(!t.contains(&SessionPhase::ProblemIntake));
}

#[test]
fn constraint_capture_cannot_skip_to_review_and_sign_off() {
    let t = SessionPhase::ConstraintCapture.valid_transitions();
    assert!(!t.contains(&SessionPhase::ReviewAndSignOff));
}

#[test]
fn constraint_capture_cannot_skip_to_signed_off() {
    let t = SessionPhase::ConstraintCapture.valid_transitions();
    assert!(!t.contains(&SessionPhase::SignedOff));
}

#[test]
fn review_and_sign_off_cannot_go_backwards_to_problem_intake() {
    let t = SessionPhase::ReviewAndSignOff.valid_transitions();
    assert!(!t.contains(&SessionPhase::ProblemIntake));
}

#[test]
fn review_and_sign_off_cannot_stay_in_same_phase() {
    let t = SessionPhase::ReviewAndSignOff.valid_transitions();
    assert!(!t.contains(&SessionPhase::ReviewAndSignOff));
}

// ─────────────────────────────────────────────────────────────────────────────
// TERMINAL STATE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn signed_off_is_terminal() {
    assert!(SessionPhase::SignedOff.is_terminal());
}

#[test]
fn problem_intake_is_not_terminal() {
    assert!(!SessionPhase::ProblemIntake.is_terminal());
}

#[test]
fn stakeholder_discovery_is_not_terminal() {
    assert!(!SessionPhase::StakeholderDiscovery.is_terminal());
}

#[test]
fn requirement_elicitation_is_not_terminal() {
    assert!(!SessionPhase::RequirementElicitation.is_terminal());
}

#[test]
fn constraint_capture_is_not_terminal() {
    assert!(!SessionPhase::ConstraintCapture.is_terminal());
}

#[test]
fn architecture_alignment_is_not_terminal() {
    assert!(!SessionPhase::ArchitectureAlignment.is_terminal());
}

#[test]
fn review_and_sign_off_is_not_terminal() {
    assert!(!SessionPhase::ReviewAndSignOff.is_terminal());
}

#[test]
fn exactly_one_terminal_phase_exists() {
    let all = [
        SessionPhase::ProblemIntake,
        SessionPhase::StakeholderDiscovery,
        SessionPhase::RequirementElicitation,
        SessionPhase::ConstraintCapture,
        SessionPhase::ArchitectureAlignment,
        SessionPhase::ReviewAndSignOff,
        SessionPhase::SignedOff,
    ];
    let terminal_count = all.iter().filter(|p| p.is_terminal()).count();
    assert_eq!(terminal_count, 1);
}

#[test]
fn all_non_terminal_phases_have_valid_transitions() {
    let non_terminal = [
        SessionPhase::ProblemIntake,
        SessionPhase::StakeholderDiscovery,
        SessionPhase::RequirementElicitation,
        SessionPhase::ConstraintCapture,
        SessionPhase::ArchitectureAlignment,
        SessionPhase::ReviewAndSignOff,
    ];
    for phase in non_terminal {
        assert!(
            !phase.valid_transitions().is_empty(),
            "{:?} has no valid transitions but is not terminal",
            phase
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DISPLAY NAMES
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn problem_intake_display_name_is_correct() {
    assert_eq!(SessionPhase::ProblemIntake.display_name(), "Problem Intake");
}

#[test]
fn stakeholder_discovery_display_name_is_correct() {
    assert_eq!(
        SessionPhase::StakeholderDiscovery.display_name(),
        "Stakeholder Discovery"
    );
}

#[test]
fn requirement_elicitation_display_name_is_correct() {
    assert_eq!(
        SessionPhase::RequirementElicitation.display_name(),
        "Requirement Elicitation"
    );
}

#[test]
fn constraint_capture_display_name_is_correct() {
    assert_eq!(
        SessionPhase::ConstraintCapture.display_name(),
        "Constraint Capture"
    );
}

#[test]
fn architecture_alignment_display_name_is_correct() {
    assert_eq!(
        SessionPhase::ArchitectureAlignment.display_name(),
        "Architecture Alignment"
    );
}

#[test]
fn review_and_sign_off_display_name_is_correct() {
    assert_eq!(
        SessionPhase::ReviewAndSignOff.display_name(),
        "Review & Sign-Off"
    );
}

#[test]
fn signed_off_display_name_is_correct() {
    assert_eq!(SessionPhase::SignedOff.display_name(), "Signed Off");
}

#[test]
fn all_display_names_are_non_empty() {
    let all = [
        SessionPhase::ProblemIntake,
        SessionPhase::StakeholderDiscovery,
        SessionPhase::RequirementElicitation,
        SessionPhase::ConstraintCapture,
        SessionPhase::ArchitectureAlignment,
        SessionPhase::ReviewAndSignOff,
        SessionPhase::SignedOff,
    ];
    for phase in all {
        assert!(
            !phase.display_name().is_empty(),
            "{:?} has an empty display name",
            phase
        );
    }
}

#[test]
fn all_display_names_are_distinct() {
    let all = [
        SessionPhase::ProblemIntake,
        SessionPhase::StakeholderDiscovery,
        SessionPhase::RequirementElicitation,
        SessionPhase::ConstraintCapture,
        SessionPhase::ArchitectureAlignment,
        SessionPhase::ReviewAndSignOff,
        SessionPhase::SignedOff,
    ];
    let names: Vec<&str> = all.iter().map(|p| p.display_name()).collect();
    let unique: std::collections::HashSet<&str> = names.iter().copied().collect();
    assert_eq!(names.len(), unique.len(), "duplicate display names found");
}

// ─────────────────────────────────────────────────────────────────────────────
// AC DISPATCH
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn evaluate_ac_for_problem_intake_is_not_transition_ready() {
    let state = fresh_state();
    assert!(
        !SessionPhase::ProblemIntake
            .evaluate_ac(&state)
            .transition_ready
    );
}

#[test]
fn evaluate_ac_for_stakeholder_discovery_is_not_transition_ready() {
    let state = fresh_state();
    assert!(
        !SessionPhase::StakeholderDiscovery
            .evaluate_ac(&state)
            .transition_ready
    );
}

#[test]
fn evaluate_ac_for_signed_off_has_no_criteria() {
    let state = fresh_state();
    let result = SessionPhase::SignedOff.evaluate_ac(&state);
    assert!(result.criteria.is_empty());
    assert!(result.gaps.is_empty());
}

#[test]
fn evaluate_ac_for_signed_off_is_not_transition_ready() {
    // Terminal phase — there is nowhere to transition to
    let state = fresh_state();
    assert!(!SessionPhase::SignedOff.evaluate_ac(&state).transition_ready);
}

// ─────────────────────────────────────────────────────────────────────────────
// BOUNDARY / EDGE CASES
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn session_phase_is_copy() {
    // Copy means we can use the value after moving a copy
    let phase = SessionPhase::ProblemIntake;
    let _copy = phase;
    assert_eq!(phase, SessionPhase::ProblemIntake);
}

#[test]
fn session_phase_equality_is_reflexive() {
    let phase = SessionPhase::ConstraintCapture;
    assert_eq!(phase, phase);
}

#[test]
fn different_phases_are_not_equal() {
    assert_ne!(
        SessionPhase::ProblemIntake,
        SessionPhase::StakeholderDiscovery
    );
    assert_ne!(
        SessionPhase::RequirementElicitation,
        SessionPhase::ConstraintCapture
    );
    assert_ne!(SessionPhase::ArchitectureAlignment, SessionPhase::SignedOff);
}

#[test]
fn valid_transitions_slice_is_static_and_non_allocating() {
    // Calling valid_transitions twice returns the same content
    let t1 = SessionPhase::ProblemIntake.valid_transitions();
    let t2 = SessionPhase::ProblemIntake.valid_transitions();
    assert_eq!(t1, t2);
}
