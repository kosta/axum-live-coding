fn from_fn() {
    async fn authenticate<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .map(|header| header.as_bytes());

        match auth_header {
            Some(auth_header) if &auth_header == b"admin=1" => Ok(next.run(req).await),
            Some(_) => Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header value",
            )),
            _ => Err((StatusCode::UNAUTHORIZED, "Missing Authorization header")),
        }
    }

    router.layer(axum::middleware::from_fn(authenticate));
}

fn from_fn_with_extension() {
    async fn authenticate<B>(
        mut req: Request<B>,
        next: Next<B>,
    ) -> Result<axum::response::Response, (StatusCode, &'static str)> {
        let authorization_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .map(|header| header.as_bytes());

        let user = match authorization_header {
            Some(header) if &header == b"admin=1" => Ok(User { admin: true }),
            Some(header) if &header == b"admin=0" => Ok(User { admin: false }),
            Some(_) => Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header value",
            )),
            _ => Err((StatusCode::UNAUTHORIZED, "Missing Authorization header")),
        }?;

        Span::current().record("admin", &user.admin);
        req.extensions_mut().insert(user);
        Ok(next.run(req).await)
    }

    async fn hello_name(Path(name): Path<String>, user: Extension<User>) -> String;
}

fn from_extractor() {
    struct User {
        admin: bool,
    }

    #[async_trait]
    impl<B> FromRequest<B> for User
    where
        B: Send,
    {
        type Rejection = (StatusCode, &'static str);

        async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
            let authorization_header = req
                .headers()
                .get(http::header::AUTHORIZATION)
                .map(|value| value.as_bytes());

            match authorization_header {
                Some(header) if &header == b"admin=1" => Ok(User { admin: true }),
                Some(header) if &header == b"admin=0" => Ok(User { admin: false }),
                Some(_) => Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid Authorization header value",
                )),
                _ => Err((StatusCode::UNAUTHORIZED, "Missing Authorization header")),
            }
        }
    }

    router.layer(axum::middleware::from_extractor::<User>())
}

fn with_span() {
    TraceLayer.make_span_with(|request: &Request<_>| {
        tracing::span!(
            Level::INFO,
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            admin = Empty,
        )
    });

    Span::current().record("admin", &user.admin);
}