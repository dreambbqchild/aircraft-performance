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
        .route("/airport", axum::routing::post(pages::airport::post))
        .route("/airport/:identifier/departure/:aircraft_type", axum::routing::get(pages::airport::get_departure))
        .route("/airport/:identifier/arrival/:aircraft_type", axum::routing::get(pages::airport::get_arrival))
        .route("/aircraft/cessna150j/take-off", axum::routing::get(pages::aircraft_pages::cessna150j::get_for_take_off))
        .route("/aircraft/cessna150j/landing", axum::routing::get(pages::aircraft_pages::cessna150j::get_for_landing))
        .route("/aircraft/cessna172m/take-off", axum::routing::get(pages::aircraft_pages::cessna172m::get_for_take_off))
        .route("/aircraft/cessna172m/landing", axum::routing::get(pages::aircraft_pages::cessna172m::get_for_landing))
        .route("/runway", axum::routing::post(pages::runway::post))
        .route("/runway", axum::routing::get(pages::runway::get));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}