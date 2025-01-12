use serde::Deserialize;
use std::env;
use url::Url;

use anyhow::{bail, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut pull_request_summary = GitHubPullRequestSummary::get_env_vars().await?;

    pull_request_summary.files = pull_request_summary.get_pr_files().await?;

    if pull_request_summary.author.is_empty() {
        bail!("PR Author Cannot Be Empty!");
    }

    let rustacean_valid =
        GitHubPullRequestSummary::is_rustacean_valid(&pull_request_summary).await?;

    println!(
        "Hello {:?} your PR number is {:?}, and your changes are {}",
        pull_request_summary.author,
        pull_request_summary.id,
        if rustacean_valid { "valid" } else { "invalid" }
    );

    Ok(())
}

struct GitHubPullRequestSummary {
    /// PR Number
    id: i32,
    /// Updated files in this PR
    files: Vec<GitHubPullRequestFile>,
    /// PR Author
    author: String,
    /// Token
    token: String,
}
#[derive(Deserialize, Debug)]

pub struct GitHubPullRequestFile {
    filename: String,
}

impl GitHubPullRequestSummary {
    pub fn new(id: i32, files: Vec<GitHubPullRequestFile>, author: String, token: String) -> Self {
        Self {
            id,
            files,
            author,
            token,
        }
    }

    pub async fn get_env_vars() -> Result<GitHubPullRequestSummary> {
        let id = env::var("PR_NUMBER")?;
        let author = env::var("PR_AUTHOR")?;
        let token = env::var("GITHUB_TOKEN")?;

        Ok(GitHubPullRequestSummary::new(
            id.parse::<i32>()?,
            vec![],
            author,
            token,
        ))
    }
    pub async fn get_pr_files(&self) -> Result<Vec<GitHubPullRequestFile>> {
        let url = Url::parse(&format!(
            "https://api.github.com/repos/rustacean-sh/rustacean.sh/pulls/{}/files",
            self.id
        ))?;
        let client = reqwest::Client::new();

        let pr_files = client
            .get(url)
            .header("Accept", "application/json")
            .header("User-Agent", "Rust")
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Vec<GitHubPullRequestFile>>()
            .await?;

        Ok(pr_files)
    }

    pub async fn is_rustacean_valid(&self) -> Result<bool> {
        if self.files.len() == 2
            && self.files[1].filename == format!("data/rustaceans/{}.toml", self.author)
        {
            return Ok(true);
        }

        println!(
            "the only file that you need to add is: data/rustaceans/{}.toml",
            self.author
        );

        Ok(false)
    }
}
