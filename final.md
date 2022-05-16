# axum-live-coding final (prepared)

```rust
use std::{
    io::ErrorKind,
    net::{Ipv6Addr, SocketAddr},
    path::PathBuf,
};

use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use axum_macros::debug_handler;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use snafu::{Location, ResultExt, Snafu};
use tokio::{fs::File, io::AsyncReadExt};
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{error, info};

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Snafu)]
enum AppError {
    #[snafu(display("io at {location} reading {path:?}: {source}"))]
    Io {
        path: PathBuf,
        source: std::io::Error,
        location: Location,
    },
    #[snafu(display("mongodb error: {source}"))]
    Mongo { source: mongodb::error::Error },
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("Caught error: {}", self);

        let (status_code, message) = match self {
            AppError::Mongo { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "database error"),
            AppError::Io { source, .. } => {
                if source.kind() == ErrorKind::NotFound {
                    (StatusCode::NOT_FOUND, "not found")
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, "io error")
                }
            }
        };

        (status_code, Json(json!({ "error": message }))).into_response()
    }
}

#[debug_handler]
async fn hello(Path(person): Path<Person>) -> Result<Html<String>, AppError> {
    let mut greeting = String::new();
    let path = PathBuf::from("greetings").join(person.first_name);
    let mut file = File::open(&path)
        .await
        .with_context(|_| IoSnafu { path: path.clone() })?;
    file.read_to_string(&mut greeting)
        .await
        .with_context(|_| IoSnafu { path })?;
    Ok(Html(greeting))
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/hello/:first_name/:last_name", get(hello))
        .layer(CatchPanicLayer::new());

    let listening_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 3000));
    info!("Listening on {listening_address}");
    axum::Server::bind(&listening_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```