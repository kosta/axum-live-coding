use anyhow::Result;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, put},
    Extension, Json, Router,
};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

async fn hello_name(Path(name): Path<u32>) -> String {
    format!("Hello, {}!", name)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FullName {
    first_name: String,
    surname: String,
}

async fn hello_full_name(Path(full_name): Path<FullName>) -> (StatusCode, Json<FullName>) {
    (StatusCode::OK, Json(full_name))
}

async fn put_hello(
    Json(full_name): Json<FullName>,
    Extension(hello_db): Extension<Database>,
) -> (StatusCode, Json<serde_json::Value>) {
    hello_db
        .collection("names")
        .insert_one(full_name, None)
        .await
        .unwrap();
    (StatusCode::CREATED, Json(json!({"created": true})))
}

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

    let hello_db = mongodb::Client::with_uri_str("mongodb://localhost:27017")
        .await?
        .database("hello");

    // build our application
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/oops", get(|| async { panic!("oops") }))
        .route("/hello/:first_name", get(hello_name))
        .route("/hello/:first_name/:surname", get(hello_full_name))
        .route("/hello", put(put_hello))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(hello_db))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .include_headers(false)
                                .level(Level::INFO),
                        )
                        .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Millis),
                        ),
                )
                .layer(CatchPanicLayer::new()),
        );

    // run the application
    const LISTENING_ADDRESS: &str = "[::]:3000";
    info!("Listing on {LISTENING_ADDRESS}");
    axum::Server::bind(&LISTENING_ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
