use crate::domain::entities::{Goal, GoalId};
use anyhow::Result;

pub trait GoalRepository: Send + Sync {
    fn get_all(&self) -> Result<Vec<Goal>>;
    fn get_by_id<T: Into<GoalId>>(&self, id: T) -> Result<Option<Goal>>;
    fn create(&mut self, goal: Goal) -> Result<Goal>;
    fn update<T: Into<GoalId>>(&mut self, id: T, goal: Goal) -> Result<()>;
    fn delete<T: Into<GoalId>>(&mut self, id: T) -> Result<()>;
}
