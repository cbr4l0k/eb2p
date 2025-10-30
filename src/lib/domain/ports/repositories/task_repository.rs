use anyhow::Result;
use chrono::NaiveDate;
use crate::domain::entities::{InitiativeId, Task, TaskDependency, TaskId};

pub struct CreateTask {
    pub initiative_id: InitiativeId,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub trait TaskRepository: Send + Sync {
    fn get_all(&self) -> Result<Vec<Task>>;
    fn get_by_id<T: Into<TaskId>>(&self, id: T) -> Result<Option<Task>>;
    fn create(&mut self, task: CreateTask) -> Result<Task>;
    fn update<T: Into<TaskId>>(&mut self, id: T, task: Task) -> Result<()>;
    fn delete<T: Into<TaskId>>(&mut self, id: T) -> Result<()>;

    fn get_dependencies<T: Into<TaskId>>(&self, task_id: T) -> Result<Vec<TaskDependency>>;
}

