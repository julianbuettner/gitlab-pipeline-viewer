use super::config::Configuration;
use git2::Repository;

#[derive(Debug)]
pub struct RepositoryDetails {
    pub origin: String,
    pub branch_or_ref: String, // "HEAD" if tag is checked out
    pub commit: String,
    pub commit_message: String,
    pub tag: Option<String>,
}

pub fn get_local_repository(conf: &Configuration) -> Result<RepositoryDetails, String> {
    let repo = Repository::discover("./");
    if repo.is_err() {
        return Err(format!(
            "No Git repository found in current working directory or above"
        ));
    }
    let repo = repo.unwrap();
    let head = repo.head();
    if head.is_err() {
        return Err(format!("No Git head found in current git project"));
    }
    let head = head.unwrap();
    if head.shorthand().is_none() {
        return Err(format!("Could not get branch of current git project"));
    }
    if head.target().is_none() {
        return Err(format!("Could not get HEAD of current git project"));
    }
    let oid = head.target().unwrap();
    let commit = repo.find_commit(oid);
    if commit.is_err() {
        return Err(format!("Could somehow not find commit of current HEAD "));
    }

    let remote_name = conf.remote.clone().unwrap_or("origin".to_string());
    let remote = repo.find_remote(&remote_name);
    if remote.is_err() {
        return Err(format!("Could not get remote \"origin\""));
    }
    let remote_origin = remote.unwrap();

    // Always None currently
    let tag = match repo.find_tag(oid) {
        Err(_) => None,
        Ok(v) => Some(format!("V:{:?}", v)),
    };

    let ok = Ok(RepositoryDetails {
        origin: remote_origin.url().unwrap().to_string().clone(),
        branch_or_ref: head.shorthand().unwrap().to_string(),
        commit: oid.to_string(),
        commit_message: commit.unwrap().summary().unwrap_or("").to_string(),
        tag,
    });
    ok
}
