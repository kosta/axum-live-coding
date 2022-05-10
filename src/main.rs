use anyhow::Result;
use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    //initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    // build our application
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run the application
    const LISTENING_ADDRESS: &str = "[::]:3000";
    info!("Listing on {LISTENING_ADDRESS}");
    axum::Server::bind(&LISTENING_ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
