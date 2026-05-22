use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::phase::SessionPhase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub is_decision_maker: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftRequirement {
    pub id: String,
    pub description: String,
    pub actor_id: Option<Uuid>,
    pub requirement_type: RequirementType,
    pub confidence: f32,
    pub source_chunk_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequirementType {
    Functional,
    NonFunctional,
    Constraint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftConstraint {
    pub id: String,
    pub description: String,
    pub constraint_type: ConstraintType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    Budget,
    Timeline,
    Technology,
    Regulatory,
    DataResidency,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub session_id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Uuid,
    pub current_phase: SessionPhase,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // Phase 1 — Problem Intake
    pub problem_statement: Option<String>,
    pub business_domain: Option<String>,
    pub primary_goal: Option<String>,

    // Phase 2 — Stakeholder Discovery
    pub actors: Vec<Actor>,
    pub external_systems: Vec<String>,
    pub success_definition: Option<String>,
    pub regulatory_context: Option<String>,

    // Phase 3 — Requirement Elicitation
    pub requirements: Vec<DraftRequirement>,

    // Phase 4 — Constraint Capture
    pub constraints: Vec<DraftConstraint>,
    pub assumptions: Vec<String>,

    // Phase 5 — Architecture Alignment
    pub architecture_approach: Option<String>,
    pub deployment_environment: Option<String>,

    // AC tracking
    pub ac_met: Vec<String>,
    pub ac_waived: Vec<String>,

    // Upload gate tracking
    pub documents_indexed: Vec<Uuid>,
    pub upload_gates_resolved: Vec<String>,
    pub upload_gates_waived: Vec<String>,

    // Output artifacts
    pub brd_artifact_id: Option<Uuid>,
    pub hld_artifact_id: Option<Uuid>,
    pub client_signature_confirmed: bool,

    // Cost tracking
    pub total_llm_cost_usd: f64,
    pub turn_count: u32,
}

impl SessionState {
    pub fn new(workspace_id: Uuid, project_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            session_id: Uuid::new_v4(),
            workspace_id,
            project_id,
            current_phase: SessionPhase::ProblemIntake,
            created_at: now,
            updated_at: now,

            problem_statement: None,
            business_domain: None,
            primary_goal: None,

            actors: Vec::new(),
            external_systems: Vec::new(),
            success_definition: None,
            regulatory_context: None,

            requirements: Vec::new(),

            constraints: Vec::new(),
            assumptions: Vec::new(),

            architecture_approach: None,
            deployment_environment: None,

            ac_met: Vec::new(),
            ac_waived: Vec::new(),

            documents_indexed: Vec::new(),
            upload_gates_resolved: Vec::new(),
            upload_gates_waived: Vec::new(),

            brd_artifact_id: None,
            hld_artifact_id: None,
            client_signature_confirmed: false,

            total_llm_cost_usd: 0.0,
            turn_count: 0,
        }
    }
}
