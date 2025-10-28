use chrono::NaiveDate;
use crate::domain::entities::{
    TaskId,
    InitiativeId,
};

pub struct Task {
    pub id: TaskId,
    pub initiative_id: InitiativeId,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub completed: bool,
}

impl Task {
    pub fn new<T: Into<String>>(
        initiative_id: InitiativeId,
        name: T,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        Self {
            id: TaskId::new(),
            initiative_id,
            name: name.into(),
            start_date,
            end_date,
            completed: false,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn update_name<T: Into<String>>(&mut self, new_name: T) {
        self.name = new_name.into();
    }

    pub fn update_start_date(&mut self, new_start_date: NaiveDate) {
        assert!(new_start_date <= self.end_date, "Start date cannot be after end date");
        self.start_date = new_start_date;
    }

    pub fn update_end_date(&mut self, new_end_date: NaiveDate) {
        assert!(new_end_date >= self.start_date, "End date cannot be before start date");
        self.end_date = new_end_date;
    }
}

pub struct TaskDependency {
    pub task_id: TaskId,
    pub depends_on_id: TaskId,
}

impl TaskDependency {
    pub fn new(task_id: TaskId, depends_on_id: TaskId) -> Self {
        Self {
            task_id,
            depends_on_id,
        }
    }
    
}
