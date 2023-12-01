use axum::{Router, Server, routing::get};
use jwt_simple::prelude::*;

#[tokio::main]
async fn main() {
    HS512Key::generate();

    let router = Router::new().route("/", get(|| async { "Hello, World!" }));


    Server::bind(&"127.0.0.1".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
