use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GithubInfo {
    pub owner: String,
    pub repo: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
}