# axum-live-coding final (prepared)

```rust
use std::{
    io::ErrorKind,
    net::{Ipv6Addr, SocketAddr},
    path::{Path, PathBuf},
    str::from_utf8,
};

use axum::{
    middleware::{self, Next},
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use axum_macros::debug_handler;
use http::{Request, StatusCode};
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};
use serde_json::json;
use snafu::{Location, ResultExt, Snafu};
use tokio::{fs::File, io::AsyncReadExt};
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{error, info};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Person {
    name: String,
}

#[derive(Debug, Snafu)]
enum AppError {
    #[snafu(display("io at {location} reading {path:?}: {source}"))]
    Io {
        path: Option<PathBuf>,
        source: std::io::Error,
        location: Location,
    },
    InvalidPath {
        path: PathBuf,
    },
    #[snafu(display("mongodb error: {source}"))]
    Mongo {
        source: mongodb::error::Error,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("Caught error: {}", self);

        let (status_code, message) = match self {
            AppError::Mongo { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "database error"),
            AppError::InvalidPath { .. } => (StatusCode::BAD_REQUEST, "invalid path"),
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
async fn hello(Extension(Person { name }): Extension<Person>) -> Result<Html<String>, AppError> {
    let root = Path::new("greetings");
    let name = Path::new(&name);
    let path = name
        .absolutize_virtually(root)
        .map_err(|_| AppError::InvalidPath {
            path: name.to_path_buf(),
        })?
        .to_path_buf();

    let mut greeting = String::new();
    let mut file = File::open(&path)
        .await
        .with_context(|_| IoSnafu { path: path.clone() })?;
    file.read_to_string(&mut greeting)
        .await
        .with_context(|_| IoSnafu { path })?;
    Ok(Html(greeting))
}

async fn authenticate<B>(mut request: Request<B>, next: Next<B>) -> impl IntoResponse {
    let authorization_header = request.headers().get(http::header::AUTHORIZATION);
    let name = match authorization_header {
        Some(value) => {
            let name = from_utf8(value.as_bytes());
            match name {
                Ok(name) => name.to_string(),
                Err(_) => return Err((StatusCode::FORBIDDEN, "bad Authorization header value")),
            }
        }
        None => return Err((StatusCode::UNAUTHORIZED, "missing Authorization header")),
    };
    request.extensions_mut().insert(Person {
        name: name.to_string(),
    });
    Ok(next.run(request).await)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/hello/:first_name/:last_name", get(hello))
        .layer(CatchPanicLayer::new())
        .layer(middleware::from_fn(authenticate));

    let listening_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 3000));
    info!("Listening on {listening_address}");
    axum::Server::bind(&listening_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```