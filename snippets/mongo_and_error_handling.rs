/// https://docs.rs/mongodb/latest/mongodb/index.html

fn insert_one() {
    let hello_db = mongodb::Client::with_uri_str("mongodb://localhost:27017")
        .await?
        .database("hello");

    router
        .route("/hello", put(put_hello))
        .layer(Extension(hello_db));

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
}

fn update_one() {
    async fn put_hello(
        Json(full_name): Json<FullName>,
        Extension(hello_db): Extension<Database>,
    ) -> (StatusCode, Json<serde_json::Value>) {
        hello_db
            .collection::<FullName>("names")
            .update_one(
                doc! {"first_name": &full_name.first_name, "surname": &full_name.surname},
                UpdateModifications::Document(doc! {"$set": bson::to_document(&full_name).unwrap()}),
                UpdateOptions::builder().upsert(true).build(),
            )
            .await
            .unwrap();
        (StatusCode::CREATED, Json(json!({"created": true})))
    }
}

fn handle_error_manually() {
    router.route(
        "/hello",
        put(
            |full_name: Json<FullName>, hello_db: Extension<Database>| async {
                put_hello(full_name, hello_db).await.map_err(handle_error)
            },
        ),
    );

    async fn put_hello(
        Json(full_name): Json<FullName>,
        Extension(hello_db): Extension<Database>,
    ) -> Result<(StatusCode, Json<serde_json::Value>)> {
        hello_db
            .collection::<FullName>("names")
            .update_one(
                doc! {"first_name": &full_name.first_name, "surname": &full_name.surname},
                UpdateModifications::Document(doc! {"$set": bson::to_document(&full_name).unwrap()}),
                UpdateOptions::builder().upsert(true).build(),
            )
            .await
            .unwrap();
        Ok((StatusCode::CREATED, Json(json!({"created": true}))))
    }

    fn handle_error(err: anyhow::Error) -> (StatusCode, Json<serde_json::Value>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": err.to_string()})),
        )
    }
}


