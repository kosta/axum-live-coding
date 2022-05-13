use std::net::{Ipv6Addr, SocketAddr};

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use mongodb::{
    bson::{self, doc},
    options::UpdateOptions,
    Database,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use snafu::{Backtrace, ErrorCompat, Location, ResultExt, Snafu};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{debug, error, info, Level};
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
