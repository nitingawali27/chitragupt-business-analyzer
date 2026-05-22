use tracing::{info, warn};

use crate::error::{Result, StateError};
use crate::gates::manager::GateManager;
use crate::state::phase::SessionPhase;
use crate::state::session::SessionState;

pub struct TransitionEngine;

impl TransitionEngine {
    /// Attempt to advance the session to `target`. Returns the updated state on success.
    pub fn attempt(
        state: &SessionState,
        target: SessionPhase,
        gate_manager: &GateManager,
    ) -> Result<()> {
        if state.current_phase.is_terminal() {
            return Err(StateError::SessionTerminated);
        }

        let allowed = state.current_phase.valid_transitions();
        if !allowed.contains(&target) {
            return Err(StateError::InvalidTransition {
                from: state.current_phase,
                to: target,
            });
        }

        // Hard gates must be clear before any transition is attempted.
        let open_hard_gates = gate_manager.open_hard_gates(state);
        if !open_hard_gates.is_empty() {
            warn!(
                session_id = %state.session_id,
                gate_count = open_hard_gates.len(),
                "transition blocked by hard gates"
            );
            return Err(StateError::HardGateBlocking {
                gate_count: open_hard_gates.len(),
            });
        }

        // AC for the current phase must be fully met.
        let ac = state.current_phase.evaluate_ac(state);
        if !ac.transition_ready {
            warn!(
                session_id = %state.session_id,
                unmet = ac.unmet_count(),
                "transition blocked by unmet AC"
            );
            return Err(StateError::AcNotMet {
                unmet_count: ac.unmet_count(),
            });
        }

        info!(
            session_id = %state.session_id,
            from = %state.current_phase,
            to = %target,
            "phase transition approved"
        );

        Ok(())
    }
}
