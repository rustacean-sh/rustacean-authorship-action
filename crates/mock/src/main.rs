use anyhow::Result;
use axum::routing::get;
use axum::Router;

const PULL_REQUEST_API_RESPONSE: &str = include_str!("../fixtures/pulls.json");
const PULL_REQUEST_FILES_API_RESPONSE: &str = include_str!("../fixtures/pulls-files.json");

#[tokio::main]
async fn main() -> Result<()> {
    println!("Started Mock Server...");

    let app = Router::new()
        .route(
            "/repos/rustacean-sh/rustacean.sh/pulls/22",
            get(|| async {
                println!("Got Request on /pulls/22");
                PULL_REQUEST_API_RESPONSE
            }),
        )
        .route(
            "/repos/rustacean-sh/rustacean.sh/pulls/22/files",
            get(|| async {
                println!("Got Request on /pulls/22/files");
                PULL_REQUEST_FILES_API_RESPONSE
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Serving Mock Server...");

    axum::serve(listener, app).await?;
    Ok(())
}
