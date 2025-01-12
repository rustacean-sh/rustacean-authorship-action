use anyhow::Result;
use axum::routing::get;
use axum::Router;

const PULL_REQUEST_API_RESPONSE: &str = include_str!("../fixtures/pulls.json");
const PULL_REQUEST_FILES_API_RESPONSE: &str = include_str!("../fixtures/pulls-files.json");

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/pulls/22", get(|| async { PULL_REQUEST_API_RESPONSE }))
        .route(
            "/pulls/22/files",
            get(|| async { PULL_REQUEST_FILES_API_RESPONSE }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
