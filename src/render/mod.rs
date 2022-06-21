mod err;
mod header;
mod jobs;
mod pipeline;
mod util;
use crate::gitlabbing::GitlabProjectPipelines;
pub use err::render_error;
use jobs::generate_job_overview;
pub use util::clear_screen;

pub fn render(gitlab_project_pipelines: &GitlabProjectPipelines) -> String {
    let width = util::get_terminal_width();

    let mut overview = header::render_header(gitlab_project_pipelines, width);
    for (pip, jobs) in &gitlab_project_pipelines.pipelines {
        overview += &pipeline::generate_pipeline_overview(pip, width as usize);
        overview += &generate_job_overview(&jobs, width as usize);
    }
    overview
}
