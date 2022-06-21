use super::util::{duration_to_string, render_columns, RenderColumnsAlignment};
use crate::emoji::*;
use gitlab::{Job, Runner, StatusState};
use std::collections::HashMap;
/*
Graphical logic and generation is done here.

We determinde the width and divide it equally between the stages.
For each stage, we print the stage name at the top.
For each stage, we print all jobs and additional information
for that job (status, runner, running time, etc)
*/

fn get_runner_name_text(runner: &Option<Runner>) -> String {
    match runner {
        None => "".to_string(),
        Some(run) => match &run.name {
            None => "[unnamed runner]".to_string(),
            Some(v) => v.clone(),
        },
    }
}

fn get_stages(jobs: &Vec<Job>) -> Vec<String> {
    let mut stages = Vec::new();
    for job in jobs.iter().rev() {
        let stage_name = &job.stage;
        if !stages.contains(stage_name) {
            stages.push(stage_name.clone());
        }
    }
    stages
}

fn get_job_lines(job: &Job) -> Vec<String> {
    let mut symbol = match job.status {
        StatusState::Created => PAUSE,
        StatusState::WaitingForResource => PAUSE,
        StatusState::Preparing => PAUSE,
        StatusState::Pending => PAUSE,
        StatusState::Running => PLAY,
        StatusState::Success => GREEN_CHECK,
        StatusState::Failed => FAILED,
        StatusState::Canceled => STOP,
        StatusState::Skipped => FAST_FORWARD,
        StatusState::Manual => PAUSE_TOGGLE,
        StatusState::Scheduled => ALARM,
    };
    if job.status == StatusState::Failed && job.allow_failure == true {
        symbol = GREY_EXCLAMATION;
    }
    let mut column = Vec::new();

    column.push("".to_string());
    column.push(format!("{}  {}", symbol, job.name));

    column.push(format!(
        "{} {}",
        duration_to_string(job.duration.unwrap_or(0.0)),
        get_runner_name_text(&job.runner)
    ));

    match job.coverage {
        None => (),
        Some(v) => column.push(format!("Coverage: {}%", v)),
    }

    for artifact in job
        .artifacts
        .iter()
        .filter(|a| a.filename != "job.log")
        .filter(|a| a.filename != "metadata.gz")
    {
        column.push(format!("Artifact: {}", artifact.filename));
    }

    column
}

pub fn generate_job_overview(jobs: &Vec<Job>, width: usize) -> String {
    let stages = get_stages(jobs);
    let width_per_stage = width as usize / stages.len() - 1;

    let mut lines_per_stage: HashMap<String, Vec<String>> = HashMap::new();
    for stage in &stages {
        lines_per_stage.insert(stage.clone(), vec![format!("=====   {}   =====", stage)]);
    }

    for job in jobs.iter().rev() {
        lines_per_stage
            .get_mut(&job.stage)
            .unwrap()
            .append(&mut get_job_lines(job));
    }

    let mut columns = Vec::new();
    for stage_name in stages.iter() {
        columns.push(lines_per_stage.get(stage_name).unwrap().clone());
    }
    let alignments = stages
        .iter()
        .map(|_| RenderColumnsAlignment::Center)
        .collect::<Vec<RenderColumnsAlignment>>();
    let widths = stages
        .iter()
        .map(|_| width_per_stage)
        .collect::<Vec<usize>>();
    render_columns(columns, widths, alignments)
}
