use std::{collections::{HashMap, HashSet}, u64};
use chrono::{Duration, NaiveDate, Utc};

use crate::domain::{
    entities::{CpmResult, CpmTask, Task, TaskDependency, TaskId},
    ports::services::CmpCalculator,
};
use anyhow::{Context, Result};

pub struct CpmService;

impl CpmService {
    pub fn new() -> Self {
        Self
    }

    fn calculate_duration(start: NaiveDate, end: NaiveDate) -> u64 {
        (end - start).num_days() as u64
    }
}

impl CmpCalculator for CpmService {
    fn calculate_cmp(
        &self,
        tasks: &[Task],
        dependencies: &[TaskDependency],
    ) -> Result<CpmResult> {
        if tasks.is_empty() {
            let today = Utc::now().naive_utc().date();
            return Ok(CpmResult {
                schedule: vec![],
                critical_path: vec![],
                project_start: today,
                project_end: today,
                project_duration_days: 0
            });
        }

        // Map of TaskId -> (Task, start, end, duration)
        let mut task_map: HashMap<TaskId, (&Task, NaiveDate, NaiveDate, u64)> = HashMap::new();
        for task in tasks {
            let duration = Self::calculate_duration(task.start_date, task.end_date);
            task_map.insert(
                task.id.clone(), 
                (
                    task,
                    task.start_date,
                    task.end_date,
                    duration
                ));
        }


        // Build reverse dependency graph
		let mut predecessors: HashMap<TaskId, Vec<TaskId>> = HashMap::new();
        let mut successors: HashMap<TaskId, Vec<TaskId>> = HashMap::new();

        for dep in dependencies {
            predecessors
                .entry(dep.task_id.clone())
                .or_default()
                .push(dep.depends_on_id.clone());
            successors
                .entry(dep.depends_on_id.clone())
                .or_default()
                .push(dep.task_id.clone());
        }

        // --- Forward pass ---
        let mut early_start: HashMap<TaskId, NaiveDate> = HashMap::new();
        let mut early_finish: HashMap<TaskId, NaiveDate> = HashMap::new();
        let mut processed: HashSet<TaskId> = HashSet::new();

        fn forward_pass(
            task_id: &TaskId,
            task_map: &HashMap<TaskId, (&Task, NaiveDate, NaiveDate, u64)>,
            predecessors: &HashMap<TaskId, Vec<TaskId>>,
            early_start: &mut HashMap<TaskId, NaiveDate>,
            early_finish: &mut HashMap<TaskId, NaiveDate>,
            processed: &mut HashSet<TaskId>,
        ) -> Result<()> {
            if processed.contains(task_id) {
                return Ok(());
            }

            let (_, start_date, _, duration) = task_map
                .get(task_id)
                .context("Task not found in map")?;

            let mut max_finish = *start_date;
            if let Some(deps) = predecessors.get(task_id) {
                for dep_id in deps {
                    forward_pass(dep_id, task_map, predecessors, early_start, early_finish, processed)?;
                    if let Some(dep_finish) = early_finish.get(dep_id) {
                        if *dep_finish > max_finish {
                            max_finish = *dep_finish;
                        }
                    }
                }
            }

            let es = max_finish;
            let ef = es + Duration::days(*duration as i64);

            early_start.insert(task_id.clone(), es);
            early_finish.insert(task_id.clone(), ef);
            processed.insert(task_id.clone());

            Ok(())
        }

        for task_id in task_map.keys() {
            forward_pass(
                task_id,
                &task_map,
                &predecessors,
                &mut early_start,
                &mut early_finish,
                &mut processed,
            )?;
        }

        // Project start and end
        let project_start = *early_start
            .values()
            .min()
            .context("No tasks processed")?;
        let project_end = *early_finish
            .values()
            .max()
            .context("No tasks processed")?;


        // --- Backward pass --- 
        let mut late_start: HashMap<TaskId, NaiveDate> = HashMap::new();
        let mut late_finish: HashMap<TaskId, NaiveDate> = HashMap::new();
        let mut processed_back: HashSet<TaskId> = HashSet::new();

        fn backward_pass(
            task_id: &TaskId,
            task_map: &HashMap<TaskId, (&Task, NaiveDate, NaiveDate, u64)>,
            successors: &HashMap<TaskId, Vec<TaskId>>,
            late_start: &mut HashMap<TaskId, NaiveDate>,
            late_finish: &mut HashMap<TaskId, NaiveDate>,
            processed_back: &mut HashSet<TaskId>,
            project_end: NaiveDate,
        ) -> Result<()> {
            if processed_back.contains(task_id) {
                return Ok(());
            }

            let (_, _, _, duration) = task_map
                .get(task_id)
                .context("Task not found in map")?;

            let mut min_start = project_end;
            if let Some(succs) = successors.get(task_id) {
                for succ_id in succs {
                    backward_pass(succ_id, task_map, successors, late_start, late_finish, processed_back, project_end)?;
                    if let Some(succ_start) = late_start.get(succ_id) {
                        if *succ_start < min_start {
                            min_start = *succ_start;
                        }
                    }
                }
            }

            let lf = min_start;
            let ls = lf - Duration::days(*duration as i64);

            late_finish.insert(task_id.clone(), lf);
            late_start.insert(task_id.clone(), ls);
            processed_back.insert(task_id.clone());

            Ok(())
        }

        for task_id in task_map.keys() {
            backward_pass(
                task_id,
                &task_map,
                &successors,
                &mut
                late_start,
                &mut
                late_finish,
                &mut
                processed_back,
                project_end)?;
        }

        // --- Compute results ---
        let mut schedule: Vec<CpmTask> = Vec::new();
        let mut critical_path: Vec<TaskId> = Vec::new();

        for (task_id, (task, _, _, duration)) in &task_map {
            let es = early_start[task_id];
            let ef = early_finish[task_id];
            let ls = late_start[task_id];
            let lf = late_finish[task_id];
            let slack = (ls - es).num_days();
            let is_critical = slack == 0;

            if is_critical {
                critical_path.push(task_id.clone());
            }

            schedule.push(CpmTask {
                task_id: task.id.clone(),
                early_start: es,
                early_finish: ef,
                late_start: ls,
                late_finish: lf,
                slack_days: slack as u64,
                is_critical, 
                duration_days: *duration,
            });
        }

        let project_duration_days = (project_end - project_start).num_days() as u64;


        Ok(CpmResult {
            schedule,
            critical_path,
            project_start,
            project_end,
            project_duration_days,
        })
    }

}
