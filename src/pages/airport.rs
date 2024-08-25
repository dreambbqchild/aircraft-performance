use askama::Template;
use axum::response::IntoResponse;

use crate::data::airports::{Airport, AirportHash, AIRPORTS};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "airport.html")]
pub struct AirportTemplate<'a> {
    airport: &'a Airport
}

pub async fn template(axum::extract::Path(identifier): axum::extract::Path<String>) -> impl IntoResponse {
    let uppercased_identifier = identifier.to_uppercase();
    let airport = AIRPORTS.load_by_identifier(uppercased_identifier).unwrap();

    let template = AirportTemplate {
        airport: airport
    };

    HtmlTemplate(template)
}