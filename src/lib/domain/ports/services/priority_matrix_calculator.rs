use crate::domain::entities::{ CpmResult, InitiativeGoal, PriorityMatrix, Task };
use anyhow::Result;

pub trait PriorityMatrixCalculator {
    fn calculate_priority_matrix(
        &self,
        initiative_goals: &[InitiativeGoal],
        tasks: &[Task],
        cpm_results: &[CpmResult],
    ) -> Result<PriorityMatrix>;
} 

