use askama::Template;
use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

use crate::{
    data::performance::aircraft::cessna150j::{Cessna150J, Landing, TakeOff},
    math::Velocity
};

use super::HtmlTemplate;

#[derive(Deserialize)]
pub struct PerformanceParameters {
    pub is_grass: Option<bool>,
    pub elevation_ft: i16,
    pub headwind_kts: i16,
    pub temperature_f: i16,
    pub standard_temperature_f: i16
}

#[derive(Template)]
#[template(path = "aircraft/cessna150j/take-off.html")]
pub struct TakeOffTemplate {
    is_grass: bool,
    calcs: TakeOff,
    cessna: Cessna150J
}

pub async fn template_for_take_off(parameters: Query<PerformanceParameters>) -> impl IntoResponse {
    let headwind = Velocity::Knots(parameters.headwind_kts);
    let cessna = Cessna150J::new(headwind, parameters.temperature_f, parameters.elevation_ft, parameters.standard_temperature_f);
    let calcs = cessna.calc_take_off();

    let template = TakeOffTemplate {
        is_grass: match parameters.is_grass { Some(value) => value, None => false },
        calcs,
        cessna
    };

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "aircraft/cessna150j/landing.html")]
pub struct LandingTemplate {
    is_grass: bool,
    calcs: Landing,
    cessna: Cessna150J
}

pub async fn template_for_landing(parameters: Query<PerformanceParameters>) -> impl IntoResponse {
    let headwind = Velocity::Knots(parameters.headwind_kts);
    let cessna = Cessna150J::new(headwind, parameters.temperature_f, parameters.elevation_ft, parameters.standard_temperature_f);
    let calcs = cessna.calc_landing();

    let template = LandingTemplate {
        is_grass: match parameters.is_grass { Some(value) => value, None => false },
        calcs,
        cessna
    };

    HtmlTemplate(template)
}