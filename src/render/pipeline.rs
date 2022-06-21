use super::util::{duration_to_string, render_columns, status_to_emoji, RenderColumnsAlignment};
use chrono::Utc;
use gitlab::Pipeline;

pub fn generate_pipeline_overview(pipeline: &Pipeline, width: usize) -> String {
    let mut pipeline_col = vec![];

    pipeline_col.push(format!("====   Pipeline {}   ====", pipeline.id));
    pipeline_col.push(pipeline.web_url.clone());

    if pipeline.created_at.is_some() {
        let now = Utc::now();
        let delta = now - pipeline.created_at.unwrap();
        let delta_ms = delta.num_milliseconds();
        pipeline_col.push(format!(
            "by {} {} ago",
            pipeline.user.name,
            duration_to_string(delta_ms as f64 / 1000.0),
        ));
    }

    pipeline_col.push(match &pipeline.ref_ {
        None => pipeline.sha.value().to_string(),
        Some(v) => format!("{} @ {}", v.clone(), pipeline.sha.value()),
    });

    let icon = status_to_emoji(pipeline.status);

    let label = pipeline
        .detailed_status
        .get("label")
        .unwrap()
        .as_str()
        .unwrap_or("unknown");

    if pipeline.duration.is_none() {
        pipeline_col.push(format!("{}   {}", icon, label,));
    } else {
        pipeline_col.push(format!(
            "{}  {} in {}",
            icon,
            label,
            duration_to_string(pipeline.duration.unwrap_or(0) as f64)
        ));
    }
    match &pipeline.coverage {
        None => (),
        Some(v) => pipeline_col.push(format!("{}% coverage", v)),
    }
    pipeline_col.push("".to_string());
    pipeline_col.push("".to_string());

    render_columns(
        vec![pipeline_col],
        vec![width],
        vec![
            RenderColumnsAlignment::Center,
            RenderColumnsAlignment::Center,
        ],
    )
}
