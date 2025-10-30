use std::{collections::{HashMap, HashSet}, u64};

use crate::domain::{
    entities::{CpmResult, InitiativeGoal, InitiativeId, PriorityItem, PriorityMatrix, Task, TaskId}, 
    ports::services::PriorityMatrixCalculator
};
use anyhow::Result;

pub struct PriorityMatrixService;

impl PriorityMatrixService {
    pub fn new() -> Self {
        Self
    }

    fn calculate_strategic_score(
        initiative_id: &InitiativeId,
        initiative_goals: &[InitiativeGoal],
        total_unique_goals: usize,
    ) -> f64 {
        let goal_count = initiative_goals
            .iter()
            .filter(|ig| ig.initiative_id == *initiative_id)
            .count();
        (goal_count as f64 / total_unique_goals.max(1) as f64)  * 100.0
    }
}

impl PriorityMatrixCalculator for PriorityMatrixService {
    fn calculate_priority_matrix(
        &self,
        initiative_goals: &[InitiativeGoal],
        tasks: &[Task],
        cpm_results: &[CpmResult],
    ) -> Result<PriorityMatrix> {
        let total_goals = initiative_goals
            .iter()
            .map(|ig| &ig.goal_id)
            .collect::<HashSet<_>>()
            .len();


        // Build task map from CPM 
        let mut task_cpm_map: HashMap<TaskId, (u64, u64)> = HashMap::new(); 
        for cpm_result in cpm_results {
            for cpm_task in &cpm_result.schedule {
                task_cpm_map.insert(
                    cpm_task.task_id.clone(), 
                    (cpm_task.slack_days, cpm_task.duration_days)
                );
            }
        }

        // Create priority items
        let mut items:Vec<PriorityItem> = Vec::new();
        for task in tasks {
            let (slack, duration) = task_cpm_map
                .get(&task.id)
                .copied()
                .unwrap_or((0, 1));

            let strategic_score = Self::calculate_strategic_score(
                &task.initiative_id,
                initiative_goals,
                total_goals,
            );

            items.push(PriorityItem {
                initiative_id: task.initiative_id.clone(),
                task_id: task.id.clone(),
                strategic_score,
                slack,
                duration,
                start_date: task.start_date,
                end_date: task.end_date,
            });
        }

        if items.is_empty() {
            return Ok(PriorityMatrix {
                high_strategic_high_critical: vec![],
                high_strategic_low_critical: vec![],
                low_strategic_high_critical: vec![],
                low_strategic_low_critical: vec![],
            });
        }

        // Calculate medians
        let mut strategic_scores: Vec<f64> = items.iter().map(|i|
            i.strategic_score).collect();
        let mut slacks: Vec<u64> = items.iter().map(|i|
            i.slack).collect();

        strategic_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        slacks.sort();

        let strategic_median = strategic_scores[strategic_scores.len() /
            2];
        let slack_median = slacks[slacks.len() / 2];

        // Categorize
        let mut high_strategic_high_critical = Vec::new();
        let mut high_strategic_low_critical = Vec::new();
        let mut low_strategic_high_critical = Vec::new();
        let mut low_strategic_low_critical = Vec::new();

        for item in items {
            let is_high_strategic = item.strategic_score >=
                strategic_median;
            let is_high_critical = item.slack <= slack_median;

            match (is_high_strategic, is_high_critical) {
                (true, true) => high_strategic_high_critical.push(item),
                (true, false) => high_strategic_low_critical.push(item),
                (false, true) => low_strategic_high_critical.push(item),
                (false, false) => low_strategic_low_critical.push(item),
            }
        }

        Ok(PriorityMatrix {
            high_strategic_high_critical,
            high_strategic_low_critical,
            low_strategic_high_critical,
            low_strategic_low_critical,
        })
    }






}





















































