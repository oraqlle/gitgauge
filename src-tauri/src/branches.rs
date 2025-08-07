use git2::{Repository};

#[tauri::command]
pub async fn get_branch_names(path: &str) -> Result<Vec<String>, String> {
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

