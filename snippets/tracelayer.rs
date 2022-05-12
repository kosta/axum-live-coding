/// https://docs.rs/axum/latest/axum/middleware/index.html#applying-multiple-middleware
use anyhow::Result;
use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

fn json_output() {
    /// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Layer.html

    fn json() {
        tracing_subscriber::registry().with(fmt::layer().json().with_span_list(false))
    }
}

fn one() {
    router.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(State {})),
    );
}

fn two() {
    router.layer(
        ServiceBuilder::new().layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        ),
    );
}

fn three() {
    router.layer(
        ServiceBuilder::new().layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .include_headers(false)
                        .level(Level::INFO),
                )
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        ),
    );
}
