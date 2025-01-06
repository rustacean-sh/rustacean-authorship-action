use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
pub struct RustaceanMember {
    pub name: String,
    pub pull_request_number: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubFile {
    filename: String,
}

impl RustaceanMember {
    pub fn new(name: String, pull_request_number: String) -> Self {
        Self {
            name,
            pull_request_number,
        }
    }

    pub async fn is_rustacean_valid(&self) -> Result<bool> {
        let url = Url::parse(&format!(
            "https://api.github.com/repos/rustacean-sh/rustacean-authorship-action/pulls/{}/files",
            self.pull_request_number
        ))?;

        let pr_files = reqwest::get(url).await?.json::<Vec<GitHubFile>>().await?;

        println!("Files are: {:?}", pr_files[0]);
        Ok(true)
    }
}
