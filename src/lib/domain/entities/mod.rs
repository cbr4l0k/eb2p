pub mod cpm;
pub mod goal;
pub mod ids;
pub mod initiative;
pub mod misc;
pub mod note;
pub mod objective;
pub mod priority_matrix;
pub mod task;
pub mod vision;

pub use goal::Goal;
pub use ids::{VisionId, GoalId, InitiativeId, TaskId, ObjectiveId, NoteId};
pub use misc::{Quantity, Year, Percent, Quarter};
pub use note::Note;
pub use objective::Objective;
pub use vision::Vision;
pub use initiative::Inititive;
pub use task::{Task, TaskDependency};
pub use cpm::{CpmTask, CpmResult};
pub use priority_matrix::{PriorityItem, PriorityMatrix};

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
