use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., "/hello?name=Yorick"
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}</strong>"))
}

//e.g., "hello2/Yorick"
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("->> LISTENING on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
