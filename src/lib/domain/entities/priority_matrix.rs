use chrono::NaiveDate;
use super::{InitiativeId, TaskId};

#[derive(Clone, Debug)]
pub struct PriorityItem {
    pub initiative_id: InitiativeId,
    pub task_id: TaskId,
    pub strategic_score: f64,
    pub slack: u64,
    pub duration: u64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub struct PriorityMatrix {
    pub high_strategic_high_critical: Vec<PriorityItem>,
    pub high_strategic_low_critical: Vec<PriorityItem>,
    pub low_strategic_high_critical: Vec<PriorityItem>,
    pub low_strategic_low_critical: Vec<PriorityItem>,
}
