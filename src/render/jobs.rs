use super::util::{center_truncate, duration_to_string};
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
        None => "[not picked up yet]".to_string(),
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

fn get_job_lines(job: &Job, width: usize) -> Vec<String> {
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
    let mut lines = Vec::new();

    let mut push_center = |text: &String| {
        lines.push(center_truncate(text, width));
    };

    push_center(&" ".to_string());
    push_center(&format!("{}  {}", symbol, job.name));

    push_center(&format!(
        "{} {}",
        duration_to_string(job.duration.unwrap_or(0.0)),
        get_runner_name_text(&job.runner)
    ));

    match job.coverage {
        None => (),
        Some(v) => push_center(&format!("Coverage: {}%", v)),
    }

    for artifact in job
        .artifacts
        .iter()
        .filter(|a| a.filename != "job.log")
        .filter(|a| a.filename != "metadata.gz")
    {
        push_center(&format!("Artifact: {}", artifact.filename));
    }

    lines
}

pub fn generate_job_overview(jobs: &Vec<Job>, width: usize) -> Vec<String> {
    let stages = get_stages(jobs);
    let width_per_stage = width as usize / stages.len() - 1;

    let mut lines_per_stage: HashMap<String, Vec<String>> = HashMap::new();
    for stage in &stages {
        lines_per_stage.insert(
            stage.clone(),
            vec![
                center_truncate(&"-".repeat(stage.len()), width_per_stage),
                center_truncate(stage, width_per_stage),
            ],
        );
    }

    for job in jobs.iter().rev() {
        lines_per_stage
            .get_mut(&job.stage)
            .unwrap()
            .append(&mut get_job_lines(job, width_per_stage));
    }

    let mut res = Vec::new();
    let mut first_line = lines_per_stage.get(&stages[0]).unwrap()[0].clone();
    let mut second_line = lines_per_stage.get(&stages[0]).unwrap()[1].clone();
    for stage_name in stages.iter().skip(1) {
        first_line = first_line + " " + &lines_per_stage.get(stage_name).unwrap()[0];
        second_line = second_line + "|" + &lines_per_stage.get(stage_name).unwrap()[1];
    }
    res.push(first_line);
    res.push(second_line);

    let max_length = lines_per_stage
        .values()
        .map(|v| v.len())
        .reduce(usize::max)
        .unwrap();
    let empty = " ".repeat(width_per_stage);
    for i in 2..max_length {
        let mut line_segements = Vec::new();
        for stage_name in stages.iter() {
            line_segements.push(
                lines_per_stage
                    .get(stage_name)
                    .unwrap()
                    .get(i)
                    .unwrap_or(&empty)
                    .clone(),
            );
        }
        res.push(line_segements.join(" "));
    }

    res
}
