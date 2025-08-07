use std::collections::HashMap;

use git2::{build::RepoBuilder, BranchType, RemoteCallbacks, Repository, Sort};
use model::repo::Repo;
use crate::response::contributor::{Author, ContributorInfo};

mod model;
mod response;

// Stubb for future call that will load data for in-memory model of repo
mod github_url_verifier;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn load_data_from(_repo: Repo) -> Result<(), &'static str> {
    Ok(())
}

#[tauri::command]
async fn get_branch_names(path: &str) -> Result<Vec<String>, String> {

    let repo = Repository::open(path).map_err(|e| e.to_string())?;
    let mut branches = Vec::new();

    for branch in repo.branches(None).map_err(|e| e.to_string())? {
        let (branch, _branch_type) = branch.map_err(|e| e.to_string())?;
        if let Some(name) = branch.name().map_err(|e| e.to_string())? {
            branches.push(name.to_string());
        }
    }

    Ok(branches)
}

#[tauri::command]
async fn get_contributor_info(path: &str, branch: Option<&str>, date_range: Option<(i64, i64)>) -> Result<HashMap<String, ContributorInfo>, String> {
    /*
    date_range: Option<(i64, i64)> - Optional date range in UNIX timestamp format.
     */
    let repo = match Repository::open(path) {
        Ok(repo) => {
            log::info!("Successfully opened repository at {}", path);
            repo
        },
        Err(e) => {
            return Err("Error opening repository".to_string());
        }
    };
    
    let mut branches: Vec<String> = Vec::new();
    for branch in repo.branches(None).map_err(|e| e.to_string())? {
        let (branch, _branch_type) = branch.map_err(|e| e.to_string())?;
        if let Some(name) = branch.name().map_err(|e| e.to_string())? {
            branches.push(name.to_string());
        }
    }

    // Ensure the branch exists before proceeding
    let target_branch = branch.unwrap_or("main");
    if !branches.contains(&target_branch.to_string()) {
        log::error!("Branch {} not found in the repository.", target_branch);
        return Err(format!("Branch {} not found.", target_branch));
    }
    // Resolve branch reference
    let mut revwalk = repo.revwalk().map_err(|e| e.to_string())?;
    let head = match branch {
        Some(b) => repo.find_branch(b, BranchType::Local).map_err(|e| e.to_string())?.get().target().ok_or(git2::Error::from_str("Invalid branch head")).map_err(|e| e.to_string())?,
        None => repo.head().map_err(|e| e.to_string())?.target().ok_or(git2::Error::from_str("Invalid HEAD")).map_err(|e| e.to_string())?,
    };

    revwalk.push(head).map_err(|e| e.to_string())?;
    revwalk.set_sorting(Sort::TIME).map_err(|e| e.to_string())?;

    let mut contributors: HashMap<String, ContributorInfo> = HashMap::new();
    for oid_result in revwalk {
        let oid = oid_result.map_err(|e| e.to_string())?;
        let commit = repo.find_commit(oid).map_err(|e| e.to_string())?;
        let time = commit.time().seconds();
        if let Some((start, end)) = date_range {
            if time < start || time > end {
                continue; // Skip commits outside the date range
            }
        }
        let author_signature = commit.author();
        
        let email = author_signature.email().unwrap_or("").to_string();
        let gravatar_hash = md5::compute(email.trim().to_lowercase());
        
        let author = Author {
            login: author_signature.name().unwrap_or("Unknown").to_string(),
            avatar_url: format!("https://www.gravatar.com/avatar/{:x}?d=identicon", gravatar_hash),
        };

        let commit_tree = commit.tree().map_err(|e| e.to_string())?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0).map_err(|e| e.to_string())?.tree().map_err(|e| e.to_string())?)
        } else {
            None
        };

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&commit_tree), None).map_err(|e| e.to_string())?;
        let stats = diff.stats().map_err(|e| e.to_string())?;
        
        let additions = stats.insertions() as u64;
        let deletions = stats.deletions() as u64;

        let entry = contributors.entry(email).or_insert_with(|| ContributorInfo {
            author: author.clone(),
            total_commits: 0,
            additions: 0,
            deletions: 0,
        });

        entry.total_commits += 1;
        entry.additions += additions;
        entry.deletions += deletions;
    }
    Ok(contributors)

}

fn clone_progress(cur_progress: usize, total_progress: usize) {
    println!("Progress: {}/{}", cur_progress, total_progress);
}

#[tauri::command]
async fn bare_clone(url: &str, path: &str) -> Result<(), String> {
    //Check if path is a valid directory
     if is_repo_cloned(path) {
        log::info!("Repository already exists at: {}", path);
        return Ok(());  // Repository is already cloned, no need to clone again
    }
    log::info!("Cloning repository from {} to {}", url, path);

    let mut callbacks = RemoteCallbacks::new();
    callbacks.transfer_progress(|progress| {
        clone_progress(progress.received_objects(), progress.total_objects());
        true
    });

    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);

    let repo = RepoBuilder::new()
        .bare(true) // Set to true for a bare clone
        .fetch_options(fetch_opts)
        .clone(url, std::path::Path::new(path))
        .map_err(|e| e.to_string())?;
    
    log::info!("Repository cloned successfully at: {}", path);
    Ok(())
}
#[tauri::command]
fn is_repo_cloned(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            bare_clone,
            get_branch_names,
            get_contributor_info,
            github_url_verifier::verify_and_extract_source_info,
            is_repo_cloned
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}