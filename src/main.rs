#![allow(dead_code)]

mod data;
mod pages;
mod math;

use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/airport/:identifier", axum::routing::get(pages::airport::template))
        .route("/aircraft/cessna150j/take-off", axum::routing::get(pages::cessna150j::template_for_take_off))
        .route("/aircraft/cessna150j/landing", axum::routing::get(pages::cessna150j::template_for_landing))
        .route("/runway", axum::routing::get(pages::runway::template));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}