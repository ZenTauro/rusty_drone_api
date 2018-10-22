/// Contains the information regarding an user
#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    id: i64,
    login: String,
    email: String,
    avatar_url: String,
    admin: bool,
    active: bool,
}

/// Contains information about a repo
#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    id: usize,
    scm: String,
    owner: String,
    name: String,
    full_name: String,
    avatar_url: String,
    link_url: String,
    clone_url: String,
    default_branch: String,
    timeout: usize,
    private: bool,
    trusted: bool,
    allow_pr: bool,
    allow_push: bool,
    allow_deploys: bool,
    allow_tags: bool,
    visibility: String,
    gated: bool,
    active: bool,
    last_build: usize,
    config_file: String,
}

#[derive(Serialize, Deserialize)]
pub struct FeedElement {
    #[serde(rename = "owner")]
    owner: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "full_name")]
    full_name: String,
    #[serde(rename = "number")]
    number: i64,
    #[serde(rename = "event")]
    event: Event,
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "created_at")]
    created_at: i64,
    #[serde(rename = "started_at")]
    started_at: i64,
    #[serde(rename = "finished_at")]
    finished_at: i64,
    #[serde(rename = "commit")]
    commit: String,
    #[serde(rename = "branch")]
    branch: String,
    #[serde(rename = "ref")]
    test_ref: String,
    #[serde(rename = "message")]
    message: String,
    #[serde(rename = "author")]
    author: String,
    #[serde(rename = "author_avatar")]
    author_avatar: String,
    #[serde(rename = "author_email")]
    author_email: String,
    #[serde(rename = "refspec")]
    refspec: Option<String>,
    #[serde(rename = "title")]
    title: Option<String>,
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
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "number")]
    number: i64,
    #[serde(rename = "parent")]
    parent: i64,
    #[serde(rename = "event")]
    event: String,
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "created_at")]
    created_at: i64,
    #[serde(rename = "enqueued_at")]
    enqueued_at: i64,
    #[serde(rename = "started_at")]
    started_at: i64,
    #[serde(rename = "finished_at")]
    finished_at: i64,
    #[serde(rename = "deploy_to")]
    deploy_to: String,
    #[serde(rename = "commit")]
    commit: String,
    #[serde(rename = "branch")]
    branch: String,
    #[serde(rename = "ref")]
    build_ref: String,
    #[serde(rename = "remote")]
    remote: String,
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "message")]
    message: String,
    #[serde(rename = "timestamp")]
    timestamp: i64,
    #[serde(rename = "sender")]
    sender: String,
    #[serde(rename = "author")]
    author: String,
    #[serde(rename = "author_avatar")]
    author_avatar: String,
    #[serde(rename = "author_email")]
    author_email: String,
    #[serde(rename = "link_url")]
    link_url: String,
    #[serde(rename = "signed")]
    signed: bool,
    #[serde(rename = "verified")]
    verified: bool,
    #[serde(rename = "reviewed_by")]
    reviewed_by: String,
    #[serde(rename = "reviewed_at")]
    reviewed_at: i64,
}
