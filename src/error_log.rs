use gitlab::api::{self, projects, Query};
use gitlab::{Gitlab, Job, Pipeline, PipelineBasic, Project};
use std::str;

const PROJECT: &'static str = "julianbuettner/pipelinetesting";


