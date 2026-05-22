use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcStatus {
    Met,
    Unmet,
    Waived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcCriterion {
    pub id: String,
    pub description: String,
    pub status: AcStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcGap {
    pub criterion_id: String,
    pub description: String,
    pub suggested_question: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcResult {
    pub criteria: Vec<AcCriterion>,
    pub gaps: Vec<AcGap>,
    pub transition_ready: bool,
}

impl AcResult {
    pub fn all_unmet(criteria: Vec<(&str, &str, &str)>) -> Self {
        let gaps: Vec<AcGap> = criteria
            .iter()
            .map(|(id, desc, question)| AcGap {
                criterion_id: id.to_string(),
                description: desc.to_string(),
                suggested_question: question.to_string(),
            })
            .collect();

        let criteria: Vec<AcCriterion> = criteria
            .into_iter()
            .map(|(id, desc, _)| AcCriterion {
                id: id.to_string(),
                description: desc.to_string(),
                status: AcStatus::Unmet,
            })
            .collect();

        AcResult {
            criteria,
            gaps,
            transition_ready: false,
        }
    }

    pub fn met_count(&self) -> usize {
        self.criteria
            .iter()
            .filter(|c| c.status == AcStatus::Met)
            .count()
    }

    pub fn unmet_count(&self) -> usize {
        self.criteria
            .iter()
            .filter(|c| c.status == AcStatus::Unmet)
            .count()
    }
}
