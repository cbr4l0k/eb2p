use crate::domain::entities::{Objective, ObjectiveId, Percent, VisionId};
use anyhow::Result;

pub struct CreateObjective {
    pub vision_id: VisionId,
    pub name: String,
    pub metric: String,
    pub target: Percent,
}

pub trait ObjectiveRepository: Send + Sync {
    fn get_all(&self) -> Result<Vec<Objective>>;
    fn get_by_id<T: Into<ObjectiveId>>(&self, id: T) -> Result<Option<Objective>>;
    fn create(&mut self, objective: CreateObjective) -> Result<Objective>;
    fn update<T: Into<ObjectiveId>>(&mut self, id: T, objective: Objective) -> Result<()>;
    fn delete<T: Into<ObjectiveId>>(&mut self, id: T) -> Result<()>;
}

