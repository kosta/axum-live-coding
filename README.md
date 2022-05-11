# Live coding a rust microservice

* Authorization middleware
* Rc / Send issues
* Swagger / Doku
* Testing
* Add records to spans
* Jaeger tracing
* Streaming insert / output
* Circuit Braker
* Exponential Backoff
* Message Broker
* Service Mesh ? Istio
* Domain Driven Design / Clean architecture / Hexagonal Architecture / Cloud Ready / 12 factor
* ORM

## Catching panics

Use the [tower-http](https://crates.io/crates/tower-http) `CatchPanicLayer`

## Logging / Tracing

Initialize using [tracing-subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/index.html).

Use `trace!()`/`debug!()`/`info!()`/`warn!()`/`error!()` logging macros.

## Path and JSON

see snippets

## Connect to a mongodb

Add an `Extension` layer to the router: `router.layer(Extension(hello_db))`. Use it as `Extension(hello_db): Extension<Database>`
in a handler function. Note how you cannot just ignore errors, you need to explicitely handle them, at least through `unwrap()`.

## Handle Errors

anyhow or snafu. Don't forget feature `backtraces`. Or just use snafu
[`Location`](https://docs.rs/snafu/latest/snafu/struct.Location.html).

## ORM

No experience, possible venues:

* [Diesel](https://crates.io/crates/diesel)
* [SeaORM](https://github.com/SeaQL/sea-orm)
* [ormx](https://crates.io/crates/ormx)
