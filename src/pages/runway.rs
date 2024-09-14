use askama::Template;
use axum::{extract::Query, response::{Html, IntoResponse}, Form};
use serde::Deserialize;

use crate::math::{wind::WindCalcs, Pressure, Temperature, Velocity};

use super::{aircraft_pages::{self, PerformanceParameters}, ToPageTemplate};

#[derive(Deserialize)]
pub struct RunwayParameters {
    aircraft_type: String,
    is_take_off: Option<bool>,
    is_grass: Option<bool>,
    headwind_kts: i16,
    pressure_in_hg: Option<f32>,
    temperature_f: i16,
    elevation_ft: i16,
    aircraft_weight_lbs: Option<i16>
}

#[derive(Deserialize)]
pub struct RunwayConfig {
    aircraft_type: String,
    is_take_off: Option<bool>,
    is_grass: Option<bool>,
    metar: String,
    heading: i16,
    elevation_ft: i16,
    aircraft_weight_lbs: Option<i16>
}

#[derive(Template)]
#[template(path = "partials/runway.html")]
pub struct RunwayTemplate {
    mode: String,
    surface: String,
    is_grass: bool,
    elevation_ft: i16,
    headwind_kts: i16,
    standard_temperature_f: i16,
    temperature_f: i16,
    temperature_diff_from_standard: i16
}

async fn get_response(params: RunwayParameters, start_landing_flow: bool) -> impl IntoResponse {
    let is_take_off = params.is_take_off.unwrap_or(false);
    let is_grass = params.is_grass.unwrap_or(false);

    let standard_temperature_f = Temperature::standard_temperature(params.elevation_ft).fahrenheit();

    let mode = if is_take_off { "Take Off".to_string() } else { "Landing".to_string() };
    let page_title = format!("{mode} Performance");

    let template = RunwayTemplate {
        mode,
        surface: if is_grass { "grass".to_string() } else { "pavement".to_string() },
        is_grass,
        elevation_ft: params.elevation_ft,
        headwind_kts: params.headwind_kts,
        standard_temperature_f,
        temperature_f: params.temperature_f,
        temperature_diff_from_standard: params.temperature_f - standard_temperature_f
    };

    let runway_raw_html = template.render().unwrap();

    let performance = PerformanceParameters {
        headwind: Velocity::Knots(params.headwind_kts),
        pressure: match params.pressure_in_hg { Some(in_hg) => Some(Pressure::InchesOfMercury(in_hg)), None => None },
        temperature: Temperature::Fahrenheit(params.temperature_f),
        elevation_ft: params.elevation_ft,
        standard_temperature: Temperature::Fahrenheit(standard_temperature_f),
        is_grass,
        aircraft_weight_lbs: params.aircraft_weight_lbs
    };

    let aircraft_raw_html = if is_take_off {
        aircraft_pages::get_raw_html_for_take_off(params.aircraft_type, performance, start_landing_flow)
    } else {
        aircraft_pages::get_raw_html_for_landing(params.aircraft_type, performance)
    };

    let page = ToPageTemplate {
        page_title,
        raw_html: format!("{runway_raw_html}{aircraft_raw_html}")
    };

    Html(page.render().unwrap()).into_response()
}

pub async fn get(Query(parameters): Query<RunwayParameters>) -> impl IntoResponse {
    get_response(parameters, false).await
}

pub async fn post(Form(config): Form<RunwayConfig>) -> impl IntoResponse {
    let metar = metar::Metar::parse(&config.metar).expect("To decode the METAR");
    let headwind = metar.wind.calc_headwind_component_from_metar_wind_value(config.heading);
    let temperature = Temperature::Celsius(*metar.temperature.unwrap() as i16);
    let pressure = Pressure::from_metar(metar).expect("To get the pressure value from the metar");

    let headwind_kts = headwind.knots();
    let temperature_f = temperature.fahrenheit();
    let pressure_in_hg = pressure.in_hg();

    let params = RunwayParameters {
        aircraft_type: config.aircraft_type,
        is_take_off: config.is_take_off,
        is_grass: config.is_grass,
        headwind_kts,
        pressure_in_hg: Some(pressure_in_hg),
        temperature_f,
        elevation_ft: config.elevation_ft,
        aircraft_weight_lbs: config.aircraft_weight_lbs
    };
    
    let start_landing_flow = params.is_take_off.unwrap_or_default();

    get_response(params, start_landing_flow).await
}