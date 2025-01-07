use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
pub struct RustaceanMember {
    pub name: String,
    pub pull_request_number: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubFile {
    filename: String,
}

impl RustaceanMember {
    pub fn new(name: String, pull_request_number: String, token: String) -> Self {
        Self {
            name,
            pull_request_number,
            token,
        }
    }

    pub async fn is_rustacean_valid(&self) -> Result<bool> {
        let url = Url::parse(&format!(
            "https://api.github.com/repos/rustacean-sh/rustacean.sh/pulls/{}/files",
            self.pull_request_number
        ))?;

        let client = reqwest::Client::new();

        let pr_files = client
            .get(url)
            .header("Accept", "application/json")
            .header("User-Agent", "Rust")
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Vec<GitHubFile>>()
            .await?;

        if pr_files.len() == 2
            && pr_files[1].filename == format!("data/rustaceans/{}.toml", self.name)
        {
            Ok(true)
        } else {
            println!(
                "the only file that you need to add is: data/rustaceans/{}.toml",
                self.name
            );
            Ok(false)
        }
    }
}
