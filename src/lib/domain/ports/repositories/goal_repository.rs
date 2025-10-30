use chrono::NaiveDate;
use crate::domain::entities::{Goal, GoalId, VisionId};
use anyhow::Result;

pub struct CreateGoal { 
    pub vision_id: VisionId,
    pub name: String,
    pub target_date: NaiveDate
}

pub trait GoalRepository: Send + Sync {
    fn get_all(&self) -> Result<Vec<Goal>>;
    fn get_by_id<T: Into<GoalId>>(&self, id: T) -> Result<Option<Goal>>;
    fn create(&mut self, goal: CreateGoal) -> Result<Goal>;
    fn update<T: Into<GoalId>>(&mut self, id: T, goal: Goal) -> Result<()>;
    fn delete<T: Into<GoalId>>(&mut self, id: T) -> Result<()>;
}
