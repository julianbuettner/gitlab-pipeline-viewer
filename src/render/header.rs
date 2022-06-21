use super::util::{render_columns, RenderColumnsAlignment};
use crate::gitlabbing::GitlabProjectPipelines;

pub fn render_header(project: &GitlabProjectPipelines, width: usize) -> String {
    let mut left = Vec::new();
    left.push(format!("====   {}   ====", project.name));
    left.push(project.web_url.clone());
    if project.description.is_some() {
        left.push(project.description.clone().unwrap());
    }
    if project.pipelines.is_empty() {
        left.push(format!(
            "There are no pipelines runing for the remote head of the current branch"
        ));
    }
    left.push("".to_string());
    render_columns(
        vec![left],
        vec![width],
        vec![RenderColumnsAlignment::Center],
    )
}
