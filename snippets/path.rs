// https://docs.rs/axum/latest/axum/extract/struct.Path.html

fn hello() {
    async fn hello_name(Path(name): Path<u32>) -> String {
        format!("Hello, {}!", name)
    }

    router.route("/hello/:first_name", get(hello_name))
}

fn hello_full() {
    async fn hello_full_name(
        Path((
            first_name,
            surname,
        )): Path<(String, String)>,
    ) -> String {
        format!("Hello, {surname}, {first_name}!")
    }

    router.route("/hello/:first_name/:surname", get(hello_full_name))
}

fn hello_full_serde() {
    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct FullName {
        first_name: String,
        surname: String,
    }

    async fn hello_full_name(
        Path(FullName {
            first_name,
            surname,
        }): Path<FullName>,
    ) -> String {
        format!("Hello, {surname}, {first_name}!")
    }

    router.route("/hello/:first_name/:surname", get(hello_full_name))
}

fn hello_json() {
    async fn hello_full_name(Path(full_name): Path<FullName>) -> Json<FullName> {
        Json(full_name)
    }
}

fn hello_status_code_json() {
    async fn hello_full_name(Path(full_name): Path<FullName>) -> (StatusCode, Json<FullName>) {
        (StatusCode::CREATED, Json(full_name))
    }
}