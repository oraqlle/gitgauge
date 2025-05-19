use serde::Serialize;
use regex::Regex;

#[derive(Serialize, Debug)]
pub struct GithubInfo {
    pub owner: String,
    pub repo: String,
}

#[tauri::command]
pub fn verify_and_extract_github_info(url_str: &str) -> Result<GithubInfo, String> {
    // Parse the URL
    let url = url::Url::parse(url_str).map_err(|e| format!("Invalid URL: {}", e))?;

    // Acceptable hostnames
    let host = url.host_str().unwrap_or_default();
    if host != "github.com" && host != "www.github.com" {
        return Err("URL is not from GitHub".to_string());
    }

    // Extract the path segments
    let mut path_segments: Vec<&str> = url
        .path_segments()
        .map_or_else(Vec::new, |segments| segments.filter(|s| !s.is_empty()).collect());

    if path_segments.len() < 2 {
        return Err("GitHub URL must contain at least an owner and a repository name.".to_string());
    }

    let owner = path_segments.remove(0).to_string();
    let repo = path_segments.remove(0).to_string();

    // Validate using GitHub naming rules (simplified):
    // - Alphanumeric characters, dashes, and underscores.
    // - Cannot start or end with a hyphen.
    // - No consecutive hyphens.
    let name_regex = Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9-_]*[a-zA-Z0-9])?$").unwrap();
    let hyphen_consecutive = Regex::new(r"--").unwrap();

    if !name_regex.is_match(&owner) || hyphen_consecutive.is_match(&owner) {
        return Err("Invalid GitHub owner name.".to_string());
    }

    if !name_regex.is_match(&repo) || hyphen_consecutive.is_match(&repo) {
        return Err("Invalid GitHub repository name.".to_string());
    }

    Ok(GithubInfo { owner, repo })
}