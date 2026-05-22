use crate::gates::{GateType, UploadGate};
use crate::state::session::SessionState;

pub struct GateManager;

impl GateManager {
    pub fn new() -> Self {
        Self
    }

    /// Returns all hard gates that are currently open for this session.
    pub fn open_hard_gates(&self, state: &SessionState) -> Vec<UploadGate> {
        self.all_gates_for_session(state)
            .into_iter()
            .filter(|g| g.gate_type == GateType::Hard && g.is_open)
            .collect()
    }

    /// Returns all gates (any type) that are currently open.
    pub fn open_gates(&self, state: &SessionState) -> Vec<UploadGate> {
        self.all_gates_for_session(state)
            .into_iter()
            .filter(|g| g.is_open)
            .collect()
    }

    fn all_gates_for_session(&self, state: &SessionState) -> Vec<UploadGate> {
        let mut gates = Vec::new();

        // HARD: BRD artifact must exist before Review → Signed Off transition.
        gates.push(UploadGate {
            id: "GATE-BRD-EXISTS".to_string(),
            gate_type: GateType::Hard,
            description: "BRD artifact must be generated before sign-off".to_string(),
            resolution_prompt: "Generating BRD now — this will take a moment.".to_string(),
            is_open: state.brd_artifact_id.is_none(),
        });

        // HARD: HLD artifact must exist before Review → Signed Off transition.
        gates.push(UploadGate {
            id: "GATE-HLD-EXISTS".to_string(),
            gate_type: GateType::Hard,
            description: "HLD artifact must be generated before sign-off".to_string(),
            resolution_prompt: "Generating HLD diagram now — this will take a moment.".to_string(),
            is_open: state.hld_artifact_id.is_none(),
        });

        // HARD: Client signature required for final sign-off.
        gates.push(UploadGate {
            id: "GATE-CLIENT-SIGNATURE".to_string(),
            gate_type: GateType::Hard,
            description: "Client signature is required before the project is signed off"
                .to_string(),
            resolution_prompt:
                "We're waiting for the client signature. Who at the client side will be signing?"
                    .to_string(),
            is_open: !state.client_signature_confirmed,
        });

        // HARD: Regulated domain requires at least one source document.
        let is_regulated = state
            .regulatory_context
            .as_deref()
            .map(|r| !r.is_empty())
            .unwrap_or(false);
        let has_documents = !state.documents_indexed.is_empty();
        gates.push(UploadGate {
            id: "GATE-REGULATED-SOURCE-DOC".to_string(),
            gate_type: GateType::Hard,
            description: "Regulated domain projects require at least one source document".to_string(),
            resolution_prompt: "This is a regulated domain. Please upload at least one reference document before continuing, or confirm an explicit waiver.".to_string(),
            is_open: is_regulated && !has_documents,
        });

        gates
    }
}

impl Default for GateManager {
    fn default() -> Self {
        Self::new()
    }
}
