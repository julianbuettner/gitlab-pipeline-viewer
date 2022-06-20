use serde::Deserialize;
use serde_yaml::from_str;
use std::{collections::HashMap, env, fs};

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    pub remote: Option<String>, // defaults to "origin"
    #[serde(rename = "gitlab-tokens")]
    pub gitlab_token_mapping: HashMap<String, String>,
    pub cooldown: Option<f32>, // defaults to 5.0
}

pub fn read_config() -> Result<Configuration, String> {
    let home = match env::var("HOME") {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("HOME variable undefined")),
    }?;
    let content = fs::read_to_string(&format!("{}/.gitlab-pipeline-viewer.yaml", home));
    if content.is_err() {
        return Err(format!("Failed to read ~/.gitlab-pipeline-viewer.yaml"));
    }
    let content = content.unwrap();
    match from_str(&content) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!(
            "Failed to parse ~/.gitlab-pipeline-viewer.yaml\n{}",
            e
        )),
    }
}
