use std::env;

use anyhow::{bail, Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let pr_author = args.get(1).context("PR Author is required")?;

    if pr_author.is_empty() {
        bail!("PR Author Cannot Be Empty!");
    }

    println!("Hello {pr_author}!");

    Ok(())
}
