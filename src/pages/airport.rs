use askama::Template;
use axum::{extract::{Path, Query}, response::{Html, IntoResponse, Response}, Form};
use serde::Deserialize;

use crate::data::airports::{Airport, AirportHash, AIRPORTS};

use super::ErrorTemplate;

static ARRIVAL: &'static str = "Arrival";
static DEPARTURE: &'static str = "Departure";

#[derive(Deserialize)]
pub struct SelectAirportConfig {
    aircraft_type: String,
    identifier: String,
    is_arrival: Option<bool>
}

#[derive(Deserialize)]
pub struct AirportParameters {
    actual_metar: Option<String>,
    custom_metar: Option<String>
}

#[derive(Template)]
#[template(path = "airport.html")]
pub struct AirportTemplate<'a> {
    airport: &'a Airport,
    mode: &'a str,
    is_take_off: bool,
    aircraft_type: String,
    metars: Vec<String>,
    metar: String
}

async fn load_metars(identifier: &String) -> String {
    reqwest::get(format!("https://aviationweather.gov/api/data/metar?ids={identifier}&hours=24"))
    .await.unwrap()
    .text()
    .await.unwrap()
}

fn select_metar(airport_parameters: &Query<AirportParameters>) -> String {
    let empty_string = String::from("");
    let actual_or_empty = match &airport_parameters.actual_metar { Some(value) => value.clone(), None => empty_string };
    if actual_or_empty.len() > 0 {
        actual_or_empty
    }
    else {
        match &airport_parameters.custom_metar { Some(value) => value.clone(), None => actual_or_empty }
    }
}

async fn template(identifier: String, mode: &str, aircraft_type: String, metar: String) -> Response {
    let uppercased_identifier = identifier.to_uppercase();

    match AIRPORTS.load_by_identifier(&uppercased_identifier) {
        Some(airport) => {
            let metars = match metar.len() {
                0 => load_metars(&uppercased_identifier).await
                    .split('\n')
                    .filter(|metar| metar.len() > 0)
                    .map(|metar| metar.to_string())
                    .collect(),
                _ => vec![]
            };

            let template = AirportTemplate {
                airport,
                mode,
                is_take_off: mode == DEPARTURE,
                aircraft_type,
                metars,
                metar
            };

            Html(template.render().unwrap()).into_response()
        },
        None => {
            let template = ErrorTemplate::new(format!("{uppercased_identifier} not found."));
            Html(template.render().unwrap()).into_response()
        }
    }
}

async fn get(identifier: String, mode: &str, aircraft_type: String, airport_parameters: Query<AirportParameters>) -> Response{
    let metar = select_metar(&airport_parameters);
    template(identifier, mode, aircraft_type, metar).await
}

pub async fn post(Form(select_airport): Form<SelectAirportConfig>) -> Response {
    let mode = if select_airport.is_arrival.is_some_and(|v| v) { &ARRIVAL } else { &DEPARTURE };
    
    template(select_airport.identifier, mode, select_airport.aircraft_type, String::from("")).await
}

pub async fn get_departure(Path((identifier, aircraft_type)): Path<(String, String)>, airport_parameters: Query<AirportParameters>) -> Response {
    get(identifier, &DEPARTURE, aircraft_type, airport_parameters).await
}

pub async fn get_arrival(Path((identifier, aircraft_type)): Path<(String, String)>, airport_parameters: Query<AirportParameters>) -> Response {
    get(identifier, &ARRIVAL, aircraft_type, airport_parameters).await
}