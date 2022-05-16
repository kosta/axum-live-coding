use std::net::{Ipv6Addr, SocketAddr};

use axum::{routing::get, Router};

use tracing::info;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
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
    let listening_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 3000));
    info!("Listing on {listening_address}");
    axum::Server::bind(&listening_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
