


use axum::middleware;
use axum::response::Response;
use axum::routing::get_service;
use axum::{ response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;
pub use self::error::{Error, Result};
use axum::extract::{Path, Query};

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(main_response_mapper))
    .fallback_service(routes_static());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "HANDLER");

    println!();
    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}



#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}
