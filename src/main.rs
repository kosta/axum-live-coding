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

async fn hello_name(Path(name): Path<u32>) -> String {
    format!("Hello, {}!", name)
}

#[derive(Debug, Snafu)]
enum HttpError {
    #[snafu(display("serializing to bson at {location}: {source}"), context(false))]
    BsonSerialization {
        source: mongodb::bson::ser::Error,
        backtrace: Backtrace,
        location: Location,
    },
    #[snafu(display("mongodb at {location} while {action}: {source}"))]
    Mongo {
        action: &'static str,
        source: mongodb::error::Error,
        backtrace: Backtrace,
        location: Location,
    },
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        if let Some(backtrace) = self.backtrace() {
            debug!("Caught Http error: {} with backtrace {}", self, backtrace);
        }
        error!("Caught http error: {}", self);

        let error_message = match self {
            HttpError::BsonSerialization { .. } => "serialization error",
            HttpError::Mongo { .. } => "database error",
        };

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error_message })),
        )
            .into_response()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FullName {
    first_name: String,
    surname: String,
    hair_color: Option<String>,
}

async fn hello_full_name(Path(full_name): Path<FullName>) -> (StatusCode, Json<FullName>) {
    (StatusCode::OK, Json(full_name))
}

async fn put_hello(
    Json(full_name): Json<FullName>,
    Extension(hello_db): Extension<Database>,
) -> Result<(StatusCode, Json<serde_json::Value>), HttpError> {
    hello_db
        .collection::<FullName>("names")
        .update_one(
            doc! {"first_name": &full_name.first_name, "surname": &full_name.surname},
            doc! {"$set": bson::to_document(&full_name)?},
            // bson::to_document(&full_name)?,
            UpdateOptions::builder().upsert(true).build(),
        )
        .await
        .context(MongoSnafu {
            action: "put_hello",
        })?;
    Ok((StatusCode::CREATED, Json(json!({"created": true}))))
}

async fn get_error() -> Result<(), HttpError> {
    mongodb::Client::with_uri_str("")
        .await
        .context(MongoSnafu {
            action: "get_error",
        })?;
    Ok::<_, HttpError>(())
}

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

    let hello_db = mongodb::Client::with_uri_str("mongodb://localhost:27017")
        .await?
        .database("hello");

    // build our application
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/oops", get(|| async { panic!("oops") }))
        .route("/error", get(get_error))
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
    axum::Server::bind(&LISTENING_ADDRESS.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
