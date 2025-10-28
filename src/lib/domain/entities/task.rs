use crate::domain::entities::{
    TaskId,
    InitiativeId,
    DateYMD,
};

pub struct Task {
    pub id: TaskId,
    pub initiative_id: InitiativeId,
    pub name: String,
    pub start_date: DateYMD,
    pub end_date: DateYMD,
    pub completed: bool,
}

pub struct TaskDependency {
    pub task_id: TaskId,
    pub depends_on_id: TaskId,
}
