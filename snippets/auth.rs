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