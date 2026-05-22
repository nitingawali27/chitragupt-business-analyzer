// Tests for ac::s1 – s6 evaluators — criteria count, IDs, status, gaps, and AcResult helpers.

use chitragupt_state_machine::ac::{result::AcResult, result::AcStatus, s1, s2, s3, s4, s5, s6};
use chitragupt_state_machine::state::session::SessionState;
use uuid::Uuid;

fn fresh() -> SessionState {
    SessionState::new(Uuid::new_v4(), Uuid::new_v4())
}

fn assert_all_unmet(result: &AcResult) {
    for c in &result.criteria {
        assert_eq!(
            c.status,
            AcStatus::Unmet,
            "criterion {} should be Unmet on a fresh session",
            c.id
        );
    }
}

fn assert_gaps_match_unmet_criteria(result: &AcResult) {
    let unmet_ids: Vec<&str> = result
        .criteria
        .iter()
        .filter(|c| c.status == AcStatus::Unmet)
        .map(|c| c.id.as_str())
        .collect();
    let gap_ids: Vec<&str> = result
        .gaps
        .iter()
        .map(|g| g.criterion_id.as_str())
        .collect();
    assert_eq!(
        unmet_ids, gap_ids,
        "gaps must correspond to unmet criteria in order"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// S1 — PROBLEM_INTAKE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn s1_returns_six_criteria() {
    assert_eq!(s1::evaluate(&fresh()).criteria.len(), 6);
}

#[test]
fn s1_all_criteria_unmet_on_fresh_session() {
    assert_all_unmet(&s1::evaluate(&fresh()));
}

#[test]
fn s1_transition_not_ready_on_fresh_session() {
    assert!(!s1::evaluate(&fresh()).transition_ready);
}

#[test]
fn s1_met_count_is_zero() {
    assert_eq!(s1::evaluate(&fresh()).met_count(), 0);
}

#[test]
fn s1_unmet_count_equals_six() {
    assert_eq!(s1::evaluate(&fresh()).unmet_count(), 6);
}

#[test]
fn s1_gaps_count_equals_criteria_count() {
    let r = s1::evaluate(&fresh());
    assert_eq!(r.gaps.len(), r.criteria.len());
}

#[test]
fn s1_criterion_ids_have_ac_s1_prefix() {
    for c in s1::evaluate(&fresh()).criteria {
        assert!(c.id.starts_with("AC-S1-"), "unexpected id: {}", c.id);
    }
}

#[test]
fn s1_criteria_have_non_empty_descriptions() {
    for c in s1::evaluate(&fresh()).criteria {
        assert!(!c.description.is_empty(), "{} has empty description", c.id);
    }
}

#[test]
fn s1_gaps_have_non_empty_suggested_questions() {
    for g in s1::evaluate(&fresh()).gaps {
        assert!(
            !g.suggested_question.is_empty(),
            "{} has empty question",
            g.criterion_id
        );
    }
}

#[test]
fn s1_gap_criterion_ids_match_criterion_ids_in_order() {
    assert_gaps_match_unmet_criteria(&s1::evaluate(&fresh()));
}

// ─────────────────────────────────────────────────────────────────────────────
// S2 — STAKEHOLDER_DISCOVERY
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn s2_returns_six_criteria() {
    assert_eq!(s2::evaluate(&fresh()).criteria.len(), 6);
}

#[test]
fn s2_all_criteria_unmet_on_fresh_session() {
    assert_all_unmet(&s2::evaluate(&fresh()));
}

#[test]
fn s2_transition_not_ready_on_fresh_session() {
    assert!(!s2::evaluate(&fresh()).transition_ready);
}

#[test]
fn s2_met_count_is_zero() {
    assert_eq!(s2::evaluate(&fresh()).met_count(), 0);
}

#[test]
fn s2_unmet_count_equals_six() {
    assert_eq!(s2::evaluate(&fresh()).unmet_count(), 6);
}

#[test]
fn s2_criterion_ids_have_ac_s2_prefix() {
    for c in s2::evaluate(&fresh()).criteria {
        assert!(c.id.starts_with("AC-S2-"), "unexpected id: {}", c.id);
    }
}

#[test]
fn s2_gaps_have_non_empty_suggested_questions() {
    for g in s2::evaluate(&fresh()).gaps {
        assert!(
            !g.suggested_question.is_empty(),
            "{} has empty question",
            g.criterion_id
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// S3 — REQUIREMENT_ELICITATION
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn s3_returns_seven_criteria() {
    assert_eq!(s3::evaluate(&fresh()).criteria.len(), 7);
}

#[test]
fn s3_all_criteria_unmet_on_fresh_session() {
    assert_all_unmet(&s3::evaluate(&fresh()));
}

#[test]
fn s3_transition_not_ready_on_fresh_session() {
    assert!(!s3::evaluate(&fresh()).transition_ready);
}

#[test]
fn s3_met_count_is_zero() {
    assert_eq!(s3::evaluate(&fresh()).met_count(), 0);
}

#[test]
fn s3_unmet_count_equals_seven() {
    assert_eq!(s3::evaluate(&fresh()).unmet_count(), 7);
}

#[test]
fn s3_criterion_ids_have_ac_s3_prefix() {
    for c in s3::evaluate(&fresh()).criteria {
        assert!(c.id.starts_with("AC-S3-"), "unexpected id: {}", c.id);
    }
}

#[test]
fn s3_gaps_have_non_empty_suggested_questions() {
    for g in s3::evaluate(&fresh()).gaps {
        assert!(
            !g.suggested_question.is_empty(),
            "{} has empty question",
            g.criterion_id
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// S4 — CONSTRAINT_CAPTURE
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn s4_returns_seven_criteria() {
    assert_eq!(s4::evaluate(&fresh()).criteria.len(), 7);
}

#[test]
fn s4_all_criteria_unmet_on_fresh_session() {
    assert_all_unmet(&s4::evaluate(&fresh()));
}

#[test]
fn s4_transition_not_ready_on_fresh_session() {
    assert!(!s4::evaluate(&fresh()).transition_ready);
}

#[test]
fn s4_met_count_is_zero() {
    assert_eq!(s4::evaluate(&fresh()).met_count(), 0);
}

#[test]
fn s4_unmet_count_equals_seven() {
    assert_eq!(s4::evaluate(&fresh()).unmet_count(), 7);
}

#[test]
fn s4_criterion_ids_have_ac_s4_prefix() {
    for c in s4::evaluate(&fresh()).criteria {
        assert!(c.id.starts_with("AC-S4-"), "unexpected id: {}", c.id);
    }
}

#[test]
fn s4_gaps_have_non_empty_suggested_questions() {
    for g in s4::evaluate(&fresh()).gaps {
        assert!(
            !g.suggested_question.is_empty(),
            "{} has empty question",
            g.criterion_id
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// S5 — ARCHITECTURE_ALIGNMENT
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn s5_returns_seven_criteria() {
    assert_eq!(s5::evaluate(&fresh()).criteria.len(), 7);
}

#[test]
fn s5_all_criteria_unmet_on_fresh_session() {
    assert_all_unmet(&s5::evaluate(&fresh()));
}

#[test]
fn s5_transition_not_ready_on_fresh_session() {
    assert!(!s5::evaluate(&fresh()).transition_ready);
}

#[test]
fn s5_met_count_is_zero() {
    assert_eq!(s5::evaluate(&fresh()).met_count(), 0);
}

#[test]
fn s5_unmet_count_equals_seven() {
    assert_eq!(s5::evaluate(&fresh()).unmet_count(), 7);
}

#[test]
fn s5_criterion_ids_have_ac_s5_prefix() {
    for c in s5::evaluate(&fresh()).criteria {
        assert!(c.id.starts_with("AC-S5-"), "unexpected id: {}", c.id);
    }
}

#[test]
fn s5_gaps_have_non_empty_suggested_questions() {
    for g in s5::evaluate(&fresh()).gaps {
        assert!(
            !g.suggested_question.is_empty(),
            "{} has empty question",
            g.criterion_id
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// S6 — REVIEW_AND_SIGN_OFF
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn s6_returns_six_criteria() {
    assert_eq!(s6::evaluate(&fresh()).criteria.len(), 6);
}

#[test]
fn s6_all_criteria_unmet_on_fresh_session() {
    assert_all_unmet(&s6::evaluate(&fresh()));
}

#[test]
fn s6_transition_not_ready_on_fresh_session() {
    assert!(!s6::evaluate(&fresh()).transition_ready);
}

#[test]
fn s6_met_count_is_zero() {
    assert_eq!(s6::evaluate(&fresh()).met_count(), 0);
}

#[test]
fn s6_unmet_count_equals_six() {
    assert_eq!(s6::evaluate(&fresh()).unmet_count(), 6);
}

#[test]
fn s6_criterion_ids_have_ac_s6_prefix() {
    for c in s6::evaluate(&fresh()).criteria {
        assert!(c.id.starts_with("AC-S6-"), "unexpected id: {}", c.id);
    }
}

#[test]
fn s6_gaps_have_non_empty_suggested_questions() {
    for g in s6::evaluate(&fresh()).gaps {
        assert!(
            !g.suggested_question.is_empty(),
            "{} has empty question",
            g.criterion_id
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ACRESULT TYPE — BOUNDARY AND EDGE CASES
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn all_unmet_with_empty_input_has_no_criteria_and_no_gaps() {
    let result = AcResult::all_unmet(vec![]);
    assert!(result.criteria.is_empty());
    assert!(result.gaps.is_empty());
    assert!(!result.transition_ready);
}

#[test]
fn all_unmet_met_count_is_zero_regardless_of_input_size() {
    let result = AcResult::all_unmet(vec![
        ("D-01", "First", "Question one?"),
        ("D-02", "Second", "Question two?"),
        ("D-03", "Third", "Question three?"),
    ]);
    assert_eq!(result.met_count(), 0);
}

#[test]
fn all_unmet_unmet_count_equals_input_length() {
    let input = vec![
        ("D-01", "desc 1", "q 1?"),
        ("D-02", "desc 2", "q 2?"),
        ("D-03", "desc 3", "q 3?"),
        ("D-04", "desc 4", "q 4?"),
    ];
    let result = AcResult::all_unmet(input);
    assert_eq!(result.unmet_count(), 4);
}

#[test]
fn all_unmet_is_never_transition_ready() {
    // No matter how many or few criteria: all_unmet ⟹ not ready
    for count in [0, 1, 3, 10] {
        let input: Vec<(&str, &str, &str)> = (0..count)
            .map(|i| {
                let id: &'static str = Box::leak(format!("D-{:02}", i).into_boxed_str());
                (id, "desc", "question?")
            })
            .collect();
        let result = AcResult::all_unmet(input);
        assert!(
            !result.transition_ready,
            "all_unmet with {} criteria should not be ready",
            count
        );
    }
}

#[test]
fn criterion_id_in_gap_matches_criterion_id_in_criteria() {
    let result = s1::evaluate(&fresh());
    for (criterion, gap) in result.criteria.iter().zip(result.gaps.iter()) {
        assert_eq!(
            criterion.id, gap.criterion_id,
            "gap criterion_id must match corresponding criterion id"
        );
    }
}

#[test]
fn all_phases_produce_non_empty_criteria_lists() {
    let state = fresh();
    let counts = [
        s1::evaluate(&state).criteria.len(),
        s2::evaluate(&state).criteria.len(),
        s3::evaluate(&state).criteria.len(),
        s4::evaluate(&state).criteria.len(),
        s5::evaluate(&state).criteria.len(),
        s6::evaluate(&state).criteria.len(),
    ];
    for count in counts {
        assert!(
            count > 0,
            "all active phases must have at least one criterion"
        );
    }
}

#[test]
fn criterion_ids_across_all_phases_are_globally_unique() {
    let state = fresh();
    let mut all_ids: Vec<String> = Vec::new();
    for r in [
        s1::evaluate(&state),
        s2::evaluate(&state),
        s3::evaluate(&state),
        s4::evaluate(&state),
        s5::evaluate(&state),
        s6::evaluate(&state),
    ] {
        for c in r.criteria {
            all_ids.push(c.id);
        }
    }
    let unique: std::collections::HashSet<&str> = all_ids.iter().map(String::as_str).collect();
    assert_eq!(
        all_ids.len(),
        unique.len(),
        "duplicate criterion ID found across phases"
    );
}
