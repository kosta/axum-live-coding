/// docs.rs/thiserror
/// sadly, no backtraces :( https://github.com/dtolnay/thiserror/issues/130#issuecomment-846461366

#[derive(Debug, Error)]
enum HttpError {
    #[error("serializing to bson: {0}")]
    BsonSerialization(#[from] mongodb::bson::ser::Error),
    #[error("mongodb: {0}")]
    Mongo(#[from] mongodb::error::Error),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let error_message = match self {
            HttpError::BsonSerialization(_) => "serialization error",
            HttpError::Mongo(_) => "database error",
        };

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error_message })),
        )
            .into_response()
    }
}
