use chrono::NaiveDate;
use crate::domain::entities::{
    TaskId, NoteId
};

pub struct Note {
    pub id: NoteId,
    pub task_id: TaskId,
    pub content: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl Note {
    pub fn new<T: Into<String>>(
        task_id: TaskId,
        content: T,
        created_at: NaiveDate,
    ) -> Self {
        Self {
            id: NoteId::new(),
            task_id,
            content: content.into(),
            created_at,
            updated_at: created_at,
        }
    }

    pub fn update_content<T: Into<String>>(&mut self, new_content: T, updated_at: NaiveDate) {
        self.content = new_content.into();
        self.updated_at = updated_at;
    }
} 
