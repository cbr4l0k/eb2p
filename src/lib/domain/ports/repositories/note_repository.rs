use crate::domain::entities::{Note, NoteId, Task, TaskId};
use anyhow::Result;

pub struct CreateNote {
    pub task_id: TaskId,
    pub content: String,
}

pub trait NoteRepository: Send + Sync {
    fn get_all(&self) -> Result<Vec<Note>>;
    fn get_by_id<T: Into<NoteId>>(&self, id: T) -> Result<Option<Note>>;
    fn create(&mut self, note: Note) -> Result<Note>;
    fn update<T: Into<NoteId>>(&mut self, id: T, task: Task) -> Result<()>;
    fn delete<T: Into<NoteId>>(&mut self, id: T) -> Result<()>;

    fn get_by_task_id<T: Into<TaskId>>(&self, task_id: T) -> Result<Vec<Note>>;
}

