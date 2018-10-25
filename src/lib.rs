extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

pub mod structs;
use self::structs::*;


pub struct Client {
    auth_token:   String,
    base_url:     String,
    client:       reqwest::Client,
}

impl Client {
    /// Creates a new client for making requests
    /// # Arguments
    /// * `auth_token` - The authorization token provided in your account
    /// * `base_url`   - The url with no path
    ///
    /// # Examples
    /// ```
    /// extern crate drone_api;
    /// use rusty_drone::Client;
    ///
    /// let drone_client = Client::new(
    ///     "randomsupersecrettoken".to_owned(),
    ///     "drone.example.org".to_owned()
    /// );
    /// ```
    /// note that there is no trasport in the url
    pub fn new(auth_token: String, base_url: String) -> Client {
        Client {
            auth_token: format!("Bearer {}",    auth_token),
            base_url:   format!("https://{}/api", base_url),
            client:     reqwest::Client::new(),
        }
    }

    fn get(&self, url: &String) -> String {
        self.client.get(url)
            .header("Authorization", self.auth_token.clone())
            .send().unwrap().text().unwrap()
    }

    /// Retrieves the user information corresponfing to the
    /// provided authorization token.
    pub fn get_current_user(&self) -> Result<UserInfo, serde_json::Error> {
        let url = format!("{}/user", self.base_url);
        let serial = self.get(&url);
        let user_info: UserInfo = serde_json::from_str(&serial)?;

        return Ok(user_info)
    }

    /// Retrieves the repos registered to the corresponding
    /// user.
    pub fn get_current_user_repos(&self) -> Result<Vec<Repo>, serde_json::Error> {
        let url = format!("{}/user/repos", self.base_url);
        let serial = self.get(&url);
        let user_repos: Vec<Repo> = serde_json::from_str(&serial)?;

        return Ok(user_repos)
    }

    pub fn get_current_user_feed(&self) -> Result<Vec<FeedElement>, serde_json::Error> {
        let url = format!("{}/user/feed", self.base_url);
        let serial = self.get(&url);
        let user_feed: Vec<FeedElement> = serde_json::from_str(&serial)?;

        return Ok(user_feed)
    }

    pub fn get_users(&self) -> Result<Vec<UserInfo>, serde_json::Error> {
        let url = format!("{}/users", self.base_url);
        let serial = self.get(&url);
        let users: Vec<UserInfo> = serde_json::from_str(&serial)?;

        return Ok(users)
    }

    pub fn get_build_info(&self, owner: &String, repo: &String, build: &String) -> Result<Build, serde_json::Error> {
        let url = format!("{}/repos/{owner}/{repo}/builds/{build}", self.base_url,
            owner = owner, repo = repo, build = build);
        let serial = self.get(&url);
        let build: Build = serde_json::from_str(&serial)?;

        return Ok(build)
    }
}
