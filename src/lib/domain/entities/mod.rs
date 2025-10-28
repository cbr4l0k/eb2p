pub mod goal;
pub mod ids;
pub mod misc;
pub mod objective;
pub mod vision;
pub mod initiative;
pub mod task;

pub use goal::Goal;
pub use ids::{VisionId, GoalId, InitiativeId, TaskId, ObjectiveId};
pub use misc::{Quantity, Year, Percent, Quarter, DateYMD};
pub use objective::Objective;
pub use vision::Vision;
pub use initiative::Inititive;
pub use task::{Task, TaskDependency};

// =============================
// Between Structs Relationships
// =============================

pub struct GoalObjective {
    pub goal_id: GoalId,
    pub objective_id: ObjectiveId,
}

pub struct InitiativeGoal {
    pub initiative_id: InitiativeId,
    pub goal_id: GoalId,
}
