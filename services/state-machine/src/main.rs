use tracing::info;
use uuid::Uuid;

use chitragupt_state_machine::gates::manager::GateManager;
use chitragupt_state_machine::state::phase::SessionPhase;
use chitragupt_state_machine::state::session::SessionState;
use chitragupt_state_machine::state::transition::TransitionEngine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Chitragupt state machine kernel starting");

    let workspace_id = Uuid::new_v4();
    let project_id = Uuid::new_v4();
    let state = SessionState::new(workspace_id, project_id);
    let gate_manager = GateManager::new();

    info!(
        session_id = %state.session_id,
        phase = %state.current_phase,
        "new session created"
    );

    let transitions = state.current_phase.valid_transitions();
    info!(
        "valid transitions from {:?}: {:?}",
        state.current_phase, transitions
    );

    let ac_result = state.current_phase.evaluate_ac(&state);
    info!(
        met = ac_result.met_count(),
        unmet = ac_result.unmet_count(),
        transition_ready = ac_result.transition_ready,
        "AC evaluation for {:?}",
        state.current_phase
    );

    if !ac_result.gaps.is_empty() {
        info!("first gap → {}", ac_result.gaps[0].suggested_question);
    }

    let open_gates = gate_manager.open_gates(&state);
    info!("{} gate(s) currently open", open_gates.len());
    for gate in &open_gates {
        info!(
            "  [{:?}] {} — open: {}",
            gate.gate_type, gate.id, gate.is_open
        );
    }

    match TransitionEngine::attempt(&state, SessionPhase::StakeholderDiscovery, &gate_manager) {
        Ok(()) => info!("transition approved (unexpected on fresh session)"),
        Err(e) => info!("transition blocked as expected: {}", e),
    }

    info!("state machine kernel smoke test complete");

    // TODO: Sprint 1 EPIC-1 — wire gRPC server here
    // let addr = "[::1]:50051".parse()?;
    // Server::builder()
    //     .add_service(StateEngineServer::new(StateEngineService::new()))
    //     .serve(addr)
    //     .await?;

    Ok(())
}
