// AC-S2: STAKEHOLDER_DISCOVERY → REQUIREMENT_ELICITATION
// Evaluates whether stakeholder map is sufficiently complete to begin requirements.

use crate::ac::result::AcResult;
use crate::state::session::SessionState;

pub fn evaluate(_state: &SessionState) -> AcResult {
    AcResult::all_unmet(vec![
        (
            "AC-S2-01",
            "At least two named actors identified",
            "Who are the main types of users that will interact with this system?",
        ),
        (
            "AC-S2-02",
            "Primary decision-maker identified",
            "Who is the single most important stakeholder that will sign off on this project?",
        ),
        (
            "AC-S2-03",
            "At least one external system or integration identified",
            "Does this system need to connect with any existing tools, databases, or external services?",
        ),
        (
            "AC-S2-04",
            "Success definition from stakeholder perspective captured",
            "How will the client or key stakeholder know this project has been successful?",
        ),
        (
            "AC-S2-05",
            "Regulatory or compliance context noted (or explicitly none)",
            "Are there any regulatory, legal, or compliance requirements we need to be aware of?",
        ),
        (
            "AC-S2-U1",
            "Optional: org chart, stakeholder map, or RACI uploaded",
            "Do you have a stakeholder map, org chart, or RACI matrix you'd like to upload?",
        ),
    ])
}
