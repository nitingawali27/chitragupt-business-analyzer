use serde::{Deserialize, Serialize};

pub mod manager;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateType {
    /// Transition is unreachable until gate is resolved. Cannot be bypassed.
    Hard,
    /// System must issue the checkpoint prompt before offering transition.
    /// BA may decline, but the question must be asked.
    RequiredPrompt,
    /// System detects a reference and asks. BA may decline.
    /// Decline is recorded and affects confidence scoring.
    Triggered,
    /// System suggests once. BA may ignore entirely.
    Recommended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadGate {
    pub id: String,
    pub gate_type: GateType,
    pub description: String,
    pub resolution_prompt: String,
    pub is_open: bool,
}
