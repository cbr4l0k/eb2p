use crate::domain::entities::{ CpmResult, Task, TaskDependency, };
use anyhow::Result;

pub trait CpmCalculator {
    fn calculate_cmp(
        &self,
        tasks: &[Task],
        dependencies: &[TaskDependency],
    ) -> Result<CpmResult>;
}
