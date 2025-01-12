use anyhow::{bail, Result};

use crate::github_pr_summary::GitHubPullRequestSummary;

pub fn validate(gh_pr_summary: &GitHubPullRequestSummary) -> Result<()> {
    check_file_updates(gh_pr_summary)?;

    Ok(())
}

fn check_file_updates(gh_pr_summary: &GitHubPullRequestSummary) -> Result<bool> {
    let Some(rustacean_file) = gh_pr_summary.files.first() else {
        bail!("No files found in PR #{}", gh_pr_summary.pr_number);
    };

    if rustacean_file.filename != format!("data/rustaceans/{}.toml", gh_pr_summary.pr_author) {
        bail!(
            "You are only allowed to access \"data/rustaceans/{}.toml\"",
            gh_pr_summary.pr_author
        );
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use crate::github_pr_summary::GitHubFile;

    use super::*;

    #[test]
    fn validate_updates_its_own_profile_match_user_and_filename() {
        let gh_pr_summary = GitHubPullRequestSummary {
            pr_number: 1,
            pr_author: "john".to_string(),
            files: vec![GitHubFile {
                filename: "data/rustaceans/john.toml".to_string(),
                raw_url: "https://example.com".to_string(),
            }],
        };

        assert!(check_file_updates(&gh_pr_summary).unwrap());
    }

    #[test]
    fn invalidate_is_updating_its_own_profile_have_no_files() {
        let gh_pr_summary = GitHubPullRequestSummary {
            pr_number: 1,
            pr_author: "john".to_string(),
            files: vec![],
        };

        let result = check_file_updates(&gh_pr_summary);

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert_eq!(err.to_string(), "No files found in PR #1");
    }

    #[test]
    fn invalidate_is_updating_its_own_profile_wrong_file() {
        let gh_pr_summary = GitHubPullRequestSummary {
            pr_number: 1,
            pr_author: "john".to_string(),
            files: vec![GitHubFile {
                filename: "data/rustaceans/felipe.toml".to_string(),
                raw_url: "https://example.com".to_string(),
            }],
        };

        let result = check_file_updates(&gh_pr_summary);

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert_eq!(
            err.to_string(),
            "You are only allowed to access \"data/rustaceans/john.toml\""
        );
    }
}
