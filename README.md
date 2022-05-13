# Live coding a rust microservice

# TODO

* Streaming insert / output
* Message Broker
* Service Mesh ? Istio
* Domain Driven Design / Clean architecture / Hexagonal Architecture / Cloud Ready / 12 factor

## Testing

see https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

and snippets/testing.rs

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

## Dont compile non-send handlers with bad and good error messages

see `snippets/non_send_handlers.rs`

## Authentication Middleware (and adding a record to a tracing span)

see https://docs.rs/axum/latest/axum/middleware/index.html

* [`from_fn`](https://docs.rs/axum/latest/axum/middleware/fn.from_fn.html)

`async fn auth<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {...}`

* [`from_extractor`](https://docs.rs/axum/0.5.5/axum/middleware/fn.from_extractor.html)

```
struct User {
    admin: bool
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        ...
    }
}

router.from_extractor::<User>()
```

## Graceful shutdown

* hyper: https://docs.rs/hyper/0.14.18/hyper/server/struct.Server.html#method.with_graceful_shutdown

* signal handling: https://docs.rs/tokio/latest/tokio/signal/index.html

e.g.

```rust
        server.with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c()
                .await
                .map_err(|e| error!("Error waiting for ctrl-c: {}", e));
            warn!("Shutting down...");
        })
```

## Exponential Backoff

[backoff](https://crates.io/crates/backoff)

## Opentelemetry

tracing integration: https://crates.io/crates/tracing-opentelemetry

## OpenAPI / Swagger

* By annotating the handler functions https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs#L132
* Run example, then navigate to http://127.0.0.1:8080/swagger-ui/#/todo/create_todo
* Integration might improve someday? https://github.com/tokio-rs/axum/issues/50
* Other frameworks might be better at this? (warp, rocket)

## Circuit Breaker

No experience, possible venues:

* [recloser](https://crates.io/recloser)
* [failsafe](https://crates.io/failsafe)
* [crius](https://crates.io/crius)

## ORM

No experience, possible venues:

* [Diesel](https://crates.io/crates/diesel)
* [SeaORM](https://crates.io/crates/sea-orm)
* [ormx](https://crates.io/crates/ormx)
