use axum::{routing::get, Router};
use std::net::{Ipv6Addr, SocketAddr};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //initialize tracing / logging
    tracing_subscriber::fmt::init();

    // build our application
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run the application
    let listening_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 3000));
    info!("Listing on {listening_address}");
    axum::Server::bind(&listening_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
