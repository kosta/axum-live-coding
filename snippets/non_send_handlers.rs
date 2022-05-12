// error: future cannot be sent between threads safely
//    --> src/main.rs:110:1
//     |
// 110 | async fn wont_compile() {
//     | ^^^^^ future returned by `wont_compile` is not `Send`
//     |
//     = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<()>`
// note: future is not `Send` as this value is used across an await

#[debug_handler]
async fn wont_compile() {
    let rc = Rc::new(());
    sleep(Duration::from_millis(1)).await;
    *rc
}
