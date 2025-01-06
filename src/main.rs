use std::env;
pub mod services;
use services::rustacean_author::RustaceanMember;

use anyhow::{bail, Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let pr_author = args.get(1).context("PR Author is required")?;
    let pr_number = args.get(2).context("PR Number is required")?;

    if pr_author.is_empty() {
        bail!("PR Author Cannot Be Empty!");
    }

    let rustacean = RustaceanMember::new(pr_author.to_string(), pr_number.to_string());

    let rustacean_valid =
        services::rustacean_author::RustaceanMember::is_rustacean_valid(&rustacean).await?;

    println!("Hello {pr_author} your PR number is {pr_number}, and you are {rustacean_valid}");

    Ok(())
}
