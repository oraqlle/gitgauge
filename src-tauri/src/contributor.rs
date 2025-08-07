use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Contacts {
    Email(String),
    EmailList(Vec<String>),
}

#[derive(Debug, Serialize)]
pub struct Contributor {
    pub username: String,
    pub contacts: Contacts,
    pub total_commits: u64,
    pub additions: u64,
    pub deletions: u64,
    // pub changes: u64,
    pub bitmap_hash: String,
    pub bitmap: String, // tmp until using actual bitmap type
}

