use anyhow::Result;
use reqwest::RequestBuilder;
use serde::Deserialize;
use url::Url;

use super::GITHUB_BASE_URL;

pub type PrNumber = i32;

#[derive(Deserialize, Debug)]
struct GitHubPrUser {
    pub login: String,
}

#[derive(Deserialize, Debug)]
struct GitHubPr {
    pub number: PrNumber,
    pub user: GitHubPrUser,
}

#[derive(Deserialize, Debug)]
pub struct GitHubFile {
    pub filename: String,
    #[allow(unused)] // rm this when pr content is validated
    pub raw_url: String,
}

pub struct GitHubPullRequestSummary {
    /// PR Number
    pub pr_number: PrNumber,
    /// PR Author
    pub pr_author: String,
    /// Files updated in this PR
    pub files: Vec<GitHubFile>,
}

impl GitHubPullRequestSummary {
    fn make_authenticated_request(url: Url, gh_token: &String) -> RequestBuilder {
        let client = reqwest::Client::new();

        client
            .get(url)
            .header("Accept", "application/json")
            .header("User-Agent", "Rust")
            .bearer_auth(gh_token)
    }

    pub async fn from_api(pr_number: PrNumber, gh_token: String) -> Result<Self> {
        let gh_pr_url = Url::parse(&format!("{GITHUB_BASE_URL}/pulls/{}", pr_number))?;
        let github_pr_req = Self::make_authenticated_request(gh_pr_url, &gh_token);
        let github_pr = github_pr_req.send().await?.json::<GitHubPr>().await?;

        let gh_pr_files_url = Url::parse(&format!("{GITHUB_BASE_URL}/pulls/{}/files", pr_number))?;
        let github_pr_files_req = Self::make_authenticated_request(gh_pr_files_url, &gh_token);
        let github_pr_files = github_pr_files_req
            .send()
            .await?
            .json::<Vec<GitHubFile>>()
            .await?;

        Ok(Self {
            pr_number: github_pr.number,
            pr_author: github_pr.user.login,
            files: github_pr_files,
        })
    }
}
