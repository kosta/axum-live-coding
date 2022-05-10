/// https://docs.rs/anyhow/latest/anyhow/index.html

struct HttpError(anyhow::Error);

impl<E> From<E> for HttpError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        HttpError(anyhow::Error::from(error))
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        error!("Caught error: {} at {}", self.0, self.0.backtrace());

        let error_message = if self.0.is::<mongodb::error::Error>() {
            "database error"
        } else {
            "an error occured"
        };

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": &error_message })),
        )
            .into_response()
    }
}

async fn put_hello(
    Json(full_name): Json<FullName>,
    Extension(hello_db): Extension<Database>,
) -> Result<(StatusCode, Json<serde_json::Value>), HttpError> {
    hello_db
        .collection::<FullName>("names")
        .update_one(
            doc! {"first_name": &full_name.first_name, "surname": &full_name.surname},
            // doc! {"$set": bson::to_document(&full_name)?},
            bson::to_document(&full_name)?,
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(json!({"created": true}))))
}