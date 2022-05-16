/// https://docs.rs/snafu/latest/snafu/

fn with_backtrace() {
    #[derive(Debug, Snafu)]
    enum HttpError {
        #[snafu(display("serializing to bson: {source}"), context(false))]
        BsonSerialization {
            source: mongodb::bson::ser::Error,
            backtrace: Backtrace,
        },
        #[snafu(display("mongodb: {source}"), context(false))]
        Mongo {
            source: mongodb::error::Error,
            backtrace: Backtrace,
        },
    }

    impl IntoResponse for HttpError {
        fn into_response(self) -> axum::response::Response {
            if let Some(backtrace) = self.backtrace() {
                error!("Caught HttpError: {}; {}", self, backtrace);
            } else {
                error!("Caught HttpError (no backtrace): {}", self);
            }

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
}

fn with_location_and_maybe_backtrace() {
    #[derive(Debug, Snafu)]
    enum HttpError {
        #[snafu(display("serializing to bson at {location}: {source}"), context(false))]
        BsonSerialization {
            source: mongodb::bson::ser::Error,
            backtrace: Backtrace,
            location: Location,
        },
        #[snafu(display("mongodb at {location}: {source}"), context(false))]
        Mongo {
            // action: String,
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
}

fn with_location_and_action_and_maybe_backtrace() {
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

    foo.context(MongoSnafu {
        action: "put_hello",
    })?;
}

fn with_different_status_code() {
    #[derive(Debug, Snafu)]
    enum AppError {
        #[snafu(display("serializing to bson at {location}: {source}"), context(false))]
        BsonSerialization {
            source: mongodb::bson::ser::Error,
            backtrace: Backtrace,
            location: Location,
        },
        #[snafu(display("mongodb at {location}: {source}"), context(false))]
        Mongo {
            source: mongodb::error::Error,
            backtrace: Backtrace,
            location: Location,
        },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct AppErrorBody {
        error: &'static str,
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> axum::response::Response {
            error!("Caught AppError {self}");

            use mongodb::error::{ErrorKind, WriteError, WriteFailure};

            let (status_code, message) = match self {
                AppError::BsonSerialization { .. } => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "bson serialization error",
                ),
                AppError::Mongo { source, .. } => {
                    if let ErrorKind::Write(WriteFailure::WriteError(WriteError {
                        code: 11000,
                        ..
                    })) = *source.kind
                    {
                        (StatusCode::CONFLICT, "conflict")
                    } else {
                        (StatusCode::INTERNAL_SERVER_ERROR, "mongodb error")
                    }
                }
            };

            (status_code, Json(AppErrorBody { error: message })).into_response()
        }
    }

    // or db.person.createIndex({name:1}, {unique: 1})
    database
        .collection::<Person>("person")
        .create_index(
            IndexModel::builder()
                .keys(doc! {"name": 1u32})
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await?;
}
