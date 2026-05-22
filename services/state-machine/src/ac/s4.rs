// AC-S4: CONSTRAINT_CAPTURE → ARCHITECTURE_ALIGNMENT
// Evaluates whether constraints are documented and open assumptions are resolved.

use crate::ac::result::AcResult;
use crate::state::session::SessionState;

pub fn evaluate(_state: &SessionState) -> AcResult {
    AcResult::all_unmet(vec![
        (
            "AC-S4-01",
            "Budget envelope captured (or explicitly deferred)",
            "Is there a budget range or ceiling for this project, or is that still being determined?",
        ),
        (
            "AC-S4-02",
            "Timeline or delivery deadline captured",
            "Is there a target delivery date or deadline we're working towards?",
        ),
        (
            "AC-S4-03",
            "Technology constraints noted (mandated stack, forbidden tools)",
            "Are there any technology constraints — platforms the client requires, or tools that are off-limits?",
        ),
        (
            "AC-S4-04",
            "Data residency or sovereignty constraints noted (or none)",
            "Where does the data need to be stored — are there geographic or data sovereignty requirements?",
        ),
        (
            "AC-S4-05",
            "Key assumptions listed and BA-confirmed",
            "Let me read back the assumptions we've been working from — do these look correct?",
        ),
        (
            "AC-S4-U1",
            "HARD GATE if existing system: architecture doc or system diagram uploaded",
            "Since this builds on an existing system, please upload the current architecture diagram before we continue.",
        ),
        (
            "AC-S4-U2",
            "Optional: contract terms, SLA docs, or compliance certification uploaded",
            "Do you have any contract terms, SLA documents, or compliance certifications to share?",
        ),
    ])
}
