/// see https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

/// * move construction of `app` into function


async fn request(
    app: Router,
    path: &str,
    body: Bytes,
) -> Result<(Parts, Bytes), anyhow::Error> {
    let response = app
        .oneshot(Request::builder().uri(path).body(Body::from(body))?)
        .await
        .unwrap();

    let (parts, body) = response.into_parts();
    let body = hyper::body::to_bytes(body).await?;
    Ok((parts, body))
}

#[tokio::test]
async fn hello_world() -> Result<(), anyhow::Error> {
    let app = app().await?;

    let (parts, body) = request(app, "/", Bytes::new()).await?;
    assert_eq!(parts.status, StatusCode::OK);
    assert_eq!(from_utf8(&body)?, "Hello World!");

    Ok(())
}
