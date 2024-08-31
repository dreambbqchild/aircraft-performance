use askama::Template;
use axum::{extract::Query, response::{Html, IntoResponse}, Form};
use metar::Metar;
use serde::Deserialize;

use crate::math::{wind::WindCalcs, Temperature};

use super::{cessna150j, resolve_boolean, ToPageTemplate};

#[derive(Deserialize)]
pub struct RunwayParameters {
    is_take_off: Option<bool>,
    is_grass: Option<bool>,
    headwind_kts: i16,
    temperature_f: i16,
    aircraft_type: String,
    heading: i16,
    elevation: i16
}

#[derive(Deserialize)]
pub struct RunwayConfig {
    is_take_off: Option<bool>,
    is_grass: Option<bool>,
    metar: String,
    aircraft_type: String,
    heading: i16,
    elevation: i16
}

#[derive(Template)]
#[template(path = "partials/runway.html")]
pub struct RunwayTemplate {
    mode: String,
    surface: String,
    is_grass: bool,
    elevation: i16,
    headwind_kts: i16,
    standard_temperature_f: i16,
    temperature_f: i16,
    temperature_diff_from_standard: i16
}

async fn get_response(is_take_off: Option<bool>, is_grass: Option<bool>, elevation_ft: i16, temperature_f: i16, headwind_kts: i16) -> impl IntoResponse {

    let is_take_off = resolve_boolean(is_take_off);
    let is_grass = resolve_boolean(is_grass);

    let standard_temperature_f = (59.0 - ((elevation_ft as f64 / 1000.0) * 3.5)) as i16;

    let mode = if is_take_off { "Take Off".to_string() } else { "Landing".to_string() };
    let page_title = format!("{mode} Performance");

    let template = RunwayTemplate {
        mode,
        surface: if is_grass { "grass".to_string() } else { "pavement".to_string() },
        is_grass,
        elevation: elevation_ft,
        headwind_kts,
        standard_temperature_f,
        temperature_f,
        temperature_diff_from_standard: temperature_f - standard_temperature_f
    };

    let runway_raw_html = template.render().unwrap();
    let aircraft_raw_html =  if is_take_off {
        cessna150j::get_raw_html_for_take_off(headwind_kts, temperature_f, elevation_ft, standard_temperature_f, Some(is_grass), true)
    } else {
        cessna150j::get_raw_html_for_landing(headwind_kts, temperature_f, elevation_ft, standard_temperature_f, Some(is_grass))
    };

    let page = ToPageTemplate {
        page_title,
        raw_html: format!("{runway_raw_html}{aircraft_raw_html}")
    };

    Html(page.render().unwrap()).into_response()
}

pub async fn get(Query(parameters): Query<RunwayParameters>) -> impl IntoResponse {
    get_response(parameters.is_take_off, parameters.is_grass, parameters.elevation, parameters.temperature_f, parameters.headwind_kts).await
}

pub async fn post(Form(config): Form<RunwayConfig>) -> impl IntoResponse {
    let metar = Metar::parse(&config.metar).expect("To decode the METAR");
    let headwind = metar.wind.calc_headwind_component_from_metar_wind_value(config.heading);
    let temperature = Temperature::Celsius(*metar.temperature.unwrap() as i16);

    let temperature_f = temperature.fahrenheit().expect("To convert to farenheight");
    let headwind_kts = headwind.knots().expect("To convert to knots");

    get_response(config.is_take_off, config.is_grass, config.elevation, temperature_f, headwind_kts).await
}