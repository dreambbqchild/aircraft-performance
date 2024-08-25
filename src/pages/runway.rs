use askama::Template;
use axum::{extract::Query, response::IntoResponse};
use metar::Metar;
use serde::Deserialize;

use crate::math::{wind::WindCalcs, Temperature};

use super::{resolve_boolean, HtmlTemplate};

#[derive(Deserialize)]
pub struct AtmosphericConditions {
    is_takeoff: Option<bool>,
    is_grass: Option<bool>,
    metar: String,
    heading: i16,
    elevation: i16
}

#[derive(Template)]
#[template(path = "runway.html")]
pub struct RunwayTemplate {
    action: String,
    surface: String,
    elevation: i16,
    headwind_kts: i16,
    standard_temperature_f: i16,
    temperature_f: i16,
    temperature_diff_from_standard: i16
}

pub async fn template(conditions: Query<AtmosphericConditions>) -> impl IntoResponse {
    let metar = Metar::parse(&conditions.metar).expect("To decode the METAR");
    let headwind = metar.wind.calc_headwind_component_from_metar_wind_value(conditions.heading);
    let temperature = Temperature::Celsius(*metar.temperature.unwrap() as i16);

    let is_takeoff = resolve_boolean(conditions.is_takeoff);
    let is_grass = resolve_boolean(conditions.is_grass);

    let standard_temperature_f = (59.0 - ((conditions.elevation as f64 / 1000.0) * 3.5)) as i16;
    let temperature_f = temperature.fahrenheit().expect("To convert to farenheight");

    let template = RunwayTemplate {
        action: if is_takeoff { "Take Off".to_string() } else { "Landing".to_string()  },
        surface: if is_grass {"grass".to_string() } else { "pavement".to_string() },
        elevation: conditions.elevation,
        headwind_kts: headwind.knots().expect("To convert to knots"),
        standard_temperature_f,
        temperature_f,
        temperature_diff_from_standard: temperature_f - standard_temperature_f
    };

    HtmlTemplate(template)
}