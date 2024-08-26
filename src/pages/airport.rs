use askama::Template;
use axum::response::{Html, IntoResponse, Response};

use crate::data::airports::{Airport, AirportHash, AIRPORTS};

use super::ErrorTemplate;

#[derive(Template)]
#[template(path = "airport.html")]
pub struct AirportTemplate<'a> {
    airport: &'a Airport,
    metars: Vec<String>
}

async fn load_metars(identifier: &String) -> String {
    reqwest::get(format!("https://aviationweather.gov/api/data/metar?ids={identifier}&hours=24"))
    .await.unwrap()
    .text()
    .await.unwrap()
}

pub async fn template(axum::extract::Path(identifier): axum::extract::Path<String>) -> Response {
    let uppercased_identifier = identifier.to_uppercase();

    match AIRPORTS.load_by_identifier(&uppercased_identifier) {
        Some(airport) => {
            let metars = load_metars(&identifier).await
            .split('\n')
            .filter(|metar| metar.len() > 0)
            .map(|metar| metar.to_string())
            .collect();

            let template = AirportTemplate {
                airport,
                metars
            };

            Html(template.render().unwrap()).into_response()
        },
        None => {
            let template = ErrorTemplate::new(format!("{uppercased_identifier} not found."));
            Html(template.render().unwrap()).into_response()
        }
    }
}