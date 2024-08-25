use askama::Template;
use axum::response::IntoResponse;

use crate::data::airports::{Airport, AirportHash, AIRPORTS};

use super::HtmlTemplate;

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

pub async fn template(axum::extract::Path(identifier): axum::extract::Path<String>) -> impl IntoResponse {
    let uppercased_identifier = identifier.to_uppercase();
    let airport = AIRPORTS.load_by_identifier(uppercased_identifier).unwrap();
    let metars = load_metars(&identifier).await
        .split('\n')
        .filter(|metar| metar.len() > 0)
        .map(|metar| metar.to_string())
        .collect();

    let template = AirportTemplate {
        airport,
        metars
    };

    HtmlTemplate(template)
}