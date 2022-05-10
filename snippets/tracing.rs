/// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Layer.html

fn json() {
    tracing_subscriber::registry().with(fmt::layer().event_format(fmt::format().json()))
}
