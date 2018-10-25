/// Contains the information regarding an user
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: i64,
    pub login: String,
    pub email: String,
    pub avatar_url: String,
    pub admin: bool,
    pub active: bool,
}

/// Contains information about a repo
#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub id: usize,
    pub scm: String,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub avatar_url: String,
    pub link_url: String,
    pub clone_url: String,
    pub default_branch: String,
    pub timeout: usize,
    pub private: bool,
    pub trusted: bool,
    pub allow_pr: bool,
    pub allow_push: bool,
    pub allow_deploys: bool,
    pub allow_tags: bool,
    pub visibility: String,
    pub gated: bool,
    pub active: bool,
    pub last_build: usize,
    pub config_file: String,
}

/// Contains a feed element
#[derive(Serialize, Deserialize)]
pub struct FeedElement {
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub number: i64,
    pub event: Event,
    pub status: Status,
    pub created_at: i64,
    pub started_at: i64,
    pub finished_at: i64,
    pub commit: String,
    pub branch: String,
    #[serde(rename = "ref")]
    pub test_ref: String,
    pub message: String,
    pub author: String,
    pub author_avatar: String,
    pub author_email: Option<String>,
    pub refspec: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "pull_request")]
    PullRequest,
    #[serde(rename = "push")]
    Push,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "success")]
    Success,
}

#[derive(Serialize, Deserialize)]
pub struct Build {
    pub id: i64,
    pub number: i64,
    pub parent: i64,
    pub event: String,
    pub status: Status,
    pub created_at: i64,
    pub enqueued_at: i64,
    pub started_at: i64,
    pub finished_at: i64,
    pub deploy_to: String,
    pub commit: String,
    pub branch: String,
    #[serde(rename = "ref")]
    pub build_ref: String,
    pub remote: String,
    pub title: String,
    pub message: String,
    pub timestamp: i64,
    pub sender: String,
    pub author: String,
    pub author_avatar: String,
    pub author_email: Option<String>,
    pub link_url: String,
    pub signed: bool,
    pub verified: bool,
    pub reviewed_by: String,
    pub reviewed_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RegistryInfo {
    pub id: i64,
    pub address: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub token: String,
}
