use crate::domain::entities::{ CpmResult, Inititive, PriorityMatrix, };
use anyhow::Result;

pub trait PriorityMatrixCalculator {
    fn calculate_priority_matrix(
        &self,
        initiatives: &[Inititive],
        cpm_results: &[CpmResult],
    ) -> Result<PriorityMatrix>;
} 

