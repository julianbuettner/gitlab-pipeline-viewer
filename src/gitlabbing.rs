use super::git::RepositoryDetails;
use crate::config::Configuration;
use gitlab::api::{projects, Query};
use gitlab::{types, Gitlab, Job, Pipeline, PipelineBasic, Project};
use regex::Regex;

pub struct GitlabProjectPipelines {
    pub project_id: u64,
    pub name: String,
    pub web_url: String,
    pub description: Option<String>,
    pub pipelines: Vec<(Pipeline, Vec<Job>)>,
}

fn parse_origin(origin: &String) -> Option<(String, String)> {
    let origin_match_ssh =
        Regex::new(r"git@(?P<domain>[a-zA-Z0-9\.]+):(?P<path>.*)\.git$").unwrap();
    let origin_match_http =
        Regex::new(r"https?://(?P<domain>[a-zA-Z0-9\.]+)/(?P<path>.*)$").unwrap();
    match origin_match_ssh.captures(origin) {
        None => (),
        Some(r) => return Some((r["domain"].to_string(), r["path"].to_string())),
    };
    match origin_match_http.captures(origin) {
        None => None,
        Some(r) => Some((r["domain"].to_string(), r["path"].to_string())),
    }
}

pub fn get_gitlab_pipelines(
    repo: &RepositoryDetails,
    conf: &Configuration,
) -> Result<GitlabProjectPipelines, String> {
    let domain_path = parse_origin(&repo.origin);
    if domain_path.is_none() {
        return Err("Could not parse remote origin".to_string());
    }
    let (domain, path) = domain_path.unwrap();

    let token = conf.gitlab_token_mapping.get(&domain);
    if token.is_none() {
        return Err(format!(
            "No token for origin \"{}\" found in config (\"gitlab-tokens\")",
            domain
        ));
    }
    let client_result = Gitlab::new(domain.clone(), token.unwrap());
    if client_result.is_err() {
        return Err(format!("Token or GitLab host {} invalid", domain));
    }
    let client = client_result.unwrap();

    let project_call = projects::Project::builder()
        .project(path.clone())
        .build()
        .unwrap();
    let project_result = project_call.query(&client);
    if project_result.is_err() {
        return Err(format!("Could not find {} on {}", path, domain));
    }

    let project: Project = project_result.unwrap();
    let pipeline_call = projects::pipelines::Pipelines::builder()
        .project(project.id.value())
        .ref_(repo.branch_or_ref.clone())
        .build()
        .unwrap();

    let pipelines_result: Result<Vec<PipelineBasic>, _> = pipeline_call.query(&client);
    if pipelines_result.is_err() {
        return Err(format!(
            "Could not get pipelines for {} ({})",
            path, repo.branch_or_ref
        ));
    }
    let pipelines = pipelines_result.unwrap();

    let mut pipelines_to_query = Vec::new();
    match pipelines.get(0) {
        None => (),
        Some(v) => pipelines_to_query.push(v),
    }
    pipelines_to_query.append(
        &mut pipelines
            .iter()
            .skip(1)
            .filter(|p| p.status == types::StatusState::Running)
            .take(5)
            .collect(),
    );
    let mut full_pipelines: Vec<(Pipeline, Vec<Job>)> = Vec::new();
    for pipeline in pipelines_to_query {
        let pipeline_query = projects::pipelines::Pipeline::builder()
            .project(project.id.value())
            .pipeline(pipeline.id.value())
            .build()
            .unwrap();
        let pipeline_result = pipeline_query.query(&client);
        if pipeline_result.is_err() {
            return Err(format!(
                "Could not query details of pipeline {}",
                pipeline.id.value(),
            ));
        }
        let jobs_query = projects::pipelines::PipelineJobs::builder()
            .project(project.id.value())
            .pipeline(pipeline.id.value())
            .build()
            .unwrap();
        let jobs_result = jobs_query.query(&client);
        if jobs_result.is_err() {
            return Err(format!(
                "Could not query jobs of pipeline {}",
                pipeline.id.value(),
            ));
        }
        let jobs: Vec<Job> = jobs_result.unwrap();
        full_pipelines.push((pipeline_result.unwrap(), jobs));
    }

    Ok(GitlabProjectPipelines {
        project_id: project.id.value(),
        name: project.name,
        web_url: project.web_url,
        description: project.description,
        pipelines: full_pipelines,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_origin() {
        assert_eq!(
            parse_origin(&"https://gitlab.com/julianbuettner/pipelinetesting".to_string()),
            Some((
                "gitlab.com".to_string(),
                "julianbuettner/pipelinetesting".to_string()
            ))
        );
        assert_eq!(
            parse_origin(&"git@gitlab.com:julianbuettner/pipelinetesting.git".to_string()),
            Some((
                "gitlab.com".to_string(),
                "julianbuettner/pipelinetesting".to_string()
            ))
        );
        assert_eq!(
            parse_origin(
                &"git@gitlab.com:gitlab-container-release-monitor/release-monitor-frontend.git"
                    .to_string()
            ),
            Some((
                "gitlab.com".to_string(),
                "gitlab-container-release-monitor/release-monitor-frontend".to_string()
            ))
        );
        assert_eq!(
            parse_origin(&"https://gitlab.com/julianbuettner/gitlab-pipeline-viewer".to_string()),
            Some((
                "gitlab.com".to_string(),
                "julianbuettner/gitlab-pipeline-viewer".to_string()
            ))
        );
    }
}
