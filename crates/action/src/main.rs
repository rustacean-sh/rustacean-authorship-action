mod github_pr_summary;
mod github_pr_validate;

use anyhow::Result;
use clap::Parser;

use self::github_pr_summary::{GitHubPullRequestSummary, PrNumber};
use self::github_pr_validate::validate;

/// URL used to reach GitHub's API.
/// E.g. "https://api.github.com/repos/rustacean-sh/rustacean.sh"
pub const GITHUB_BASE_URL: &str = env!("GITHUB_BASE_URL");

#[derive(Debug, Parser)]
pub struct Cli {
    /// PR Number to fetch from GitHub
    #[clap(long)]
    pr_number: PrNumber,
    /// PR Author to fetch from GitHub
    #[clap(long)]
    pr_author: String,
    /// GitHub Token
    #[clap(long)]
    github_token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    println!("Validating: PR #{} from {}", args.pr_number, args.pr_author);
    println!("Using GitHub Base URL: {GITHUB_BASE_URL}");

    let gh_pr_summary =
        GitHubPullRequestSummary::from_api(args.pr_number, args.github_token).await?;

    validate(&gh_pr_summary)?;

    Ok(())
}
