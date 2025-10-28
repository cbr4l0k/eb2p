pub mod goal_repository;
pub mod initiative_repository;
pub mod objective_repository;
pub mod task_repository;
pub mod vision_repository;

pub use goal_repository::GoalRepository;
pub use initiative_repository::InitiativeRepository;
pub use objective_repository::ObjectiveRepository;
pub use task_repository::TaskRepository;
pub use vision_repository::VisionRepository;

use crate::domain::entities::{ 
    GoalObjective,
    InitiativeGoal,
    TaskDependency,

    TaskId,
    GoalId, 
    InitiativeId,
    ObjectiveId,
};
use anyhow::Result;

pub trait GoalObjectiveRepository: Send + Sync {
    fn link<T: Into<GoalId>, U: Into<ObjectiveId>>(&mut self, goal_id: T, objective_id: U) -> Result<GoalObjective>;
    fn unlink(&mut self, link: GoalObjective) -> Result<()>;
    fn get_objectives_by_goal<T: Into<GoalId>>(&self, goal_id: T) -> Result<Vec<ObjectiveId>>;
    fn get_goals_by_objective<T: Into<ObjectiveId>>(&self, objective_id: T) -> Result<Vec<GoalId>>;
}

pub trait InitiativeGoalRepository: Send + Sync {
    fn link<T: Into<InitiativeId>, U: Into<GoalId>>(&mut self, initiative_id: T, goal_id: U) -> Result<InitiativeGoal>;
    fn unlink(&mut self, link: InitiativeGoal) -> Result<()>;
    fn get_goals_by_initiative<T: Into<InitiativeId>>(&self, initiative_id: T) -> Result<Vec<GoalId>>;
    fn get_initiatives_by_goal<T: Into<GoalId>>(&self, goal_id: T) -> Result<Vec<InitiativeId>>;
}

pub trait TaskDependencyRepository: Send + Sync {
    fn add_dependency<T: Into<TaskId>>(&mut self, task_id: T, depends_on_id: T) -> Result<TaskDependency>;
    fn remove_dependency(&mut self, dependency: TaskDependency) -> Result<()>;
    fn get_dependencies<T: Into<TaskId>>(&self, task_id: T) -> Result<Vec<TaskDependency>>;
    fn get_dependents<T: Into<TaskId>>(&self, depends_on_id: T) -> Result<Vec<TaskDependency>>;
}
