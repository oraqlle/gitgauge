use reqwest::{header::HeaderMap, Client, StatusCode};

use git2::Repository;
use model::repo::Repo;
use response::{branch::BranchReponse, contributor::{ContributorData, ContributorResponse}};
use tauri::http::HeaderValue;
use crate::response::contributor::ContributorInfo;

mod model;
mod response;

type Result2<T> = Result<T, &'static str>;

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        reqwest::header::ACCEPT,
        HeaderValue::from_static("application/vnd.github+json")
    );

    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28")
    );

    headers.insert(reqwest::header::USER_AGENT, HeaderValue::from_static("gitgauge"));

    return headers;
}

// Stubb for future call that will load data for in-memory model of repo
mod github_url_verifier;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn load_data_from(_repo: Repo) -> Result<(), &'static str> {
    Ok(())
}

#[tauri::command]
async fn get_branch_names(path: &str) -> Result<Vec<String>, git2::Error> {
    let repo = Repository::open(path)?;
    let mut branches = Vec::new();

    for branch in repo.branches(None)? {
        let (branch, _branch_type) = branch?;
        if let Some(name) = branch.name()? {
            branches.push(name.to_string());
        }
    }

    Ok(branches)
}

#[tauri::command]
async fn get_contributor_info(path: &str, branch: Option<&str>, date_range: Option<(i64, i64)>) -> Result<HashMap<String, ContributorInfo>, git2::Error> {
    /*
    date_range: Option<(i64, i64)> - Optional date range in UNIX timestamp format.
     */
    let repo = Repository::open(path)?;

    // Resolve branch reference
    let mut revwalk = repo.revwalk()?;
    let head = match branch {
        Some(b) => repo.find_branch(b, BranchType::Local)?.get().target().ok_or(git2::Error::from_str("Invalid branch head"))?,
        None => repo.head()?.target().ok_or(git2::Error::from_str("Invalid HEAD"))?,
    };

    revwalk.push(head)?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut contributors: HashMap<String, ContributorInfo> = HashMap::new();
    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;
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

        let commit_tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&commit_tree), None)?;
        let stats = diff.stats()?;
        
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


async fn clone_progress(progress: git2::Progress) {
    println!("Progress: {}/{}", progress.received_objects(), progress.total_objects());
}

#[tauri::command]
async fn bare_clone(url: &str, path: &str) -> Result<(), git2::Error> {
    //Check if path is a valid directory
    if std::path::Path::new(path).exists() {
        return Err(git2::Error::from_str("Path already exists"));
    }
    let mut callbacks = RemoteCallbacks::new();
    callbacks.transfer_progress(|progress| {
        // Run the progress callback in the main thread
        tauri::async_runtime::spawn(async move {
            clone_progress(progress).await;
        });
        true
    });

    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);

    let repo = RepoBuilder::new()
        .bare(true) // Set to true for a bare clone
        .fetch_options(fetch_opts)
        .clone(url, std::path::Path::new(path))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_branch_names,
            get_contributor_info,
            github_url_verifier::verify_and_extract_source_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}