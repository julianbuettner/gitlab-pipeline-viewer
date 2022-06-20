use super::util::{center_truncate, duration_to_string, status_to_emoji};
use chrono::Utc;
use gitlab::Pipeline;

pub fn generate_pipeline_overview(pipeline: &Pipeline, width: usize) -> Vec<String> {
    let mut result = Vec::new();

    let mut push_center = |text: &String| {
        result.push(center_truncate(text, width));
    };

    match &pipeline.ref_ {
        None => (),
        Some(v) => push_center(&v),
    }
    push_center(pipeline.sha.value());
    push_center(&pipeline.web_url);
    match &pipeline.coverage {
        None => (),
        Some(v) => push_center(&format!("Coverage: {}%", v)),
    }
    push_center(&pipeline.user.name);

    let icon = status_to_emoji(pipeline.status);

    let label = pipeline
        .detailed_status
        .get("label")
        .unwrap()
        .as_str()
        .unwrap_or("unknown");

    if pipeline.duration.is_none() {
        push_center(&format!("{}   {}", icon, label,));
    } else {
        push_center(&format!(
            "{}   {} in {}",
            icon,
            label,
            duration_to_string(pipeline.duration.unwrap_or(0) as f64)
        ));
    }
    if pipeline.created_at.is_some() {
        let now = Utc::now();
        let delta = now - pipeline.created_at.unwrap();
        let delta_ms = delta.num_milliseconds();
        push_center(&format!(
            "Created {} ago",
            duration_to_string(delta_ms as f64 / 1000.0),
        ));
    }

    push_center(&"".to_string());
    result
}
