// AC-S6: REVIEW_AND_SIGN_OFF → SIGNED_OFF
// HARD GATE: BRD artifact, HLD artifact, and client signature are all required.

use crate::ac::result::AcResult;
use crate::state::session::SessionState;

pub fn evaluate(_state: &SessionState) -> AcResult {
    AcResult::all_unmet(vec![
        (
            "AC-S6-01",
            "HARD GATE: BRD artifact generated and stored",
            "The BRD has not been generated yet. Generating it now before we can proceed to sign-off.",
        ),
        (
            "AC-S6-02",
            "HARD GATE: HLD artifact generated and stored",
            "The High-Level Architecture Diagram has not been generated. Generating it now.",
        ),
        (
            "AC-S6-03",
            "HARD GATE: Client signature received on BRD",
            "We're waiting for the client signature. Who at the client side will be signing — and have they received the document?",
        ),
        (
            "AC-S6-04",
            "BA has reviewed all SYNTHESIZED and INFERRED items in the BRD",
            "There are flagged items in the BRD that need your review before it can be signed off.",
        ),
        (
            "AC-S6-05",
            "All open conflicts resolved or explicitly deferred with BA justification",
            "There are unresolved conflicts in the requirements. Let's work through each one.",
        ),
        (
            "AC-S6-U1",
            "HARD GATE: Signed BRD document uploaded or e-signature confirmed",
            "Please upload the signed BRD or confirm that the e-signature has been completed.",
        ),
    ])
}
