use chrono::NaiveDate;
use crate::domain::entities::TaskId;

pub struct CpmTask {
    pub task_id: TaskId,
    pub early_start: NaiveDate,
    pub early_finish: NaiveDate,
    pub late_start: NaiveDate,
    pub late_finish: NaiveDate,
    pub slack_days: u64,
    pub is_critical: bool,
    pub duration_days: u64,
}

pub struct CpmResult {
    pub schedule: Vec<CpmTask>,
    pub critical_path: Vec<TaskId>,
    pub project_start: NaiveDate,
    pub project_end: NaiveDate,
    pub project_duration_days: u64,
}
