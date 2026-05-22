// AC-S3: REQUIREMENT_ELICITATION → CONSTRAINT_CAPTURE
// Evaluates whether functional and non-functional requirements are sufficiently elicited.

use crate::ac::result::AcResult;
use crate::state::session::SessionState;

pub fn evaluate(_state: &SessionState) -> AcResult {
    AcResult::all_unmet(vec![
        (
            "AC-S3-01",
            "Minimum five functional requirements captured",
            "Let's keep going — what else does the system need to do? Walk me through a typical user workflow.",
        ),
        (
            "AC-S3-02",
            "At least one non-functional requirement captured (performance, security, availability)",
            "Are there any performance or reliability targets — like response times, uptime SLAs, or concurrent user counts?",
        ),
        (
            "AC-S3-03",
            "Each requirement linked to at least one actor",
            "Who specifically needs this feature — which user role does it serve?",
        ),
        (
            "AC-S3-04",
            "No unresolved requirement conflicts flagged as open",
            "Earlier we noted a potential conflict between two requirements — has that been resolved?",
        ),
        (
            "AC-S3-05",
            "At least one acceptance criterion written for the top-priority requirement",
            "For the most critical requirement, how would you test that it works correctly?",
        ),
        (
            "AC-S3-U1",
            "HARD GATE if regulated domain: at least one source document uploaded",
            "This is a regulated domain project. Please upload at least one reference document before we continue.",
        ),
        (
            "AC-S3-U2",
            "Optional: existing requirements doc, user stories, or process flow uploaded",
            "Do you have existing user stories, process flow diagrams, or a requirements document to share?",
        ),
    ])
}
