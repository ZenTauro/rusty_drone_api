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
    pub fn new(auth_token: String, base_url: String) -> Client {
        Client {
            auth_token: format!("Bearer {}",    auth_token),
            base_url:   format!("https://{}/api", base_url),
            client:     reqwest::Client::new(),
        }
    }

    pub fn get_current_user(&self) -> Result<UserInfo, serde_json::Error> {
        let url = format!("{}/user", self.base_url);
        let serial = self.client.get(&url)
            .header("Authorization", self.auth_token.clone())
            .send().unwrap().text().unwrap();
        let user_info: UserInfo = serde_json::from_str(&serial)?;

        return Ok(user_info)
    }

    pub fn get_current_user_repos(&self) -> Result<Vec<Repo>, serde_json::Error> {
        let url = format!("{}/user/repos", self.base_url);
        let serial = self.client.get(&url)
            .header("Authorization", self.auth_token.clone())
            .send().unwrap().text().unwrap();
        let user_repos: Vec<Repo> = serde_json::from_str(&serial)?;

        return Ok(user_repos)
    }
}