/// https://docs.rs/tower-http/latest/tower_http/catch_panic/struct.CatchPanicLayer.html

fn one() {
    // must be added _after_ the tracing layer, see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    // idea: "onion" from bottom to top
    router.layer(CatchPanicLayer::new());
}
