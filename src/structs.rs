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
