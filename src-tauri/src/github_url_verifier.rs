use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GithubInfo {
    pub owner: String,
    pub repo: String,
    pub resource_type: Option<String>, // e.g., "issues", "pull", "commit", "tree"
    pub resource_id: Option<String>,   // e.g., issue number, pull request number, commit hash, branch/tag name
}

#[tauri::command]
pub fn verify_and_extract_github_info(url_str: &str) -> Result<GithubInfo, String> {
    // Parse the input string into a URL object.
    let url = match url::Url::parse(url_str) {
        Ok(u) => u,
        Err(e) => return Err(format!("Invalid URL: {}", e)),
    };

    // Check if the host is "github.com".
    if url.host_str() != Some("github.com") {
        return Err("Not a GitHub URL".to_string());
    }

    let owner = "owner".to_string();
    let repo = "repo".to_string();
    let resource_type = None;
    let resource_id = None;

    Ok(GithubInfo {
        owner,
        repo,
        resource_type,
        resource_id,
    })
}