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
