use crate::domain::entities::{Task, TaskId, TaskDependency};
use anyhow::Result;

pub trait TaskRepository: Send + Sync {
    fn get_all(&self) -> Result<Vec<Task>>;
    fn get_task_by_id<T: Into<TaskId>>(&self, id: T) -> Result<Option<Task>>;
    fn create(&mut self, task: Task) -> Result<Task>;
    fn update<T: Into<TaskId>>(&mut self, id: T, task: Task) -> Result<()>;
    fn delete<T: Into<TaskId>>(&mut self, id: T) -> Result<()>;

    fn get_dependencies<T: Into<TaskId>>(&self, task_id: T) -> Result<Vec<TaskDependency>>;
}

