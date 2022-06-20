use super::util::center_truncate;
use crate::gitlabbing::GitlabProjectPipelines;

pub fn render_header(project: &GitlabProjectPipelines, width: usize) -> Vec<String> {
    let mut res = Vec::new();
    let mut push_center = |text: String| {
        res.push(center_truncate(&text, width));
    };
    push_center(format!("{}", project.name));
    push_center(format!("{}", project.web_url));
    if project.description.is_some() {
        push_center(format!("{}", project.description.as_ref().unwrap()));
    }
    if project.pipelines.is_empty() {
        push_center(format!(
            "There are no pipelines runing for the remote head of the current branch"
        ));
    }
    res
}
