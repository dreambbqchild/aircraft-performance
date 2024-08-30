use askama::Template;
use axum::{extract::Query, response::{Html, IntoResponse, Response}};
use serde::Deserialize;

use crate::{
    data::performance::aircraft::cessna150j::{Cessna150J, Landing, TakeOff},
    math::Velocity
};

use super::{ErrorTemplate, ToPageTemplate};

#[derive(Deserialize)]
pub struct PerformanceParameters {
    pub is_grass: Option<bool>,
    pub elevation_ft: i16,
    pub headwind_kts: i16,
    pub temperature_f: i16,
    pub standard_temperature_f: i16
}

#[derive(Template)]
#[template(path = "partials/aircraft/cessna150j/take-off.html")]
pub struct TakeOffTemplate {
    is_grass: bool,
    calcs: TakeOff,
    cessna: Cessna150J
}

fn get_error_response(parameters: &Query<PerformanceParameters>) -> Response {
    let tailwind = parameters.headwind_kts.abs();
    let template = ErrorTemplate::new(format!("Tailind of {tailwind} kts detected. unable to compute"));
    Html(template.render().unwrap()).into_response()
}

pub fn get_raw_html<T>(headwind_kts: i16, temperature_f: i16, elevation_ft: i16, standard_temperature_f: i16, callback: &dyn Fn(Cessna150J) -> T) -> String where T : Template {
    let headwind = Velocity::Knots(headwind_kts);
    let cessna = Cessna150J::new(headwind, temperature_f, elevation_ft, standard_temperature_f);

    let template = callback(cessna);
    template.render().unwrap()
}

pub fn get_raw_html_for_take_off(headwind_kts: i16, temperature_f: i16, elevation_ft: i16, standard_temperature_f: i16, is_grass: Option<bool>) -> String {
    get_raw_html(headwind_kts, temperature_f, elevation_ft, standard_temperature_f, &|cessna| {
        let calcs = cessna.calc_take_off(); 
        TakeOffTemplate {
            is_grass: match is_grass { Some(value) => value, None => false },
            calcs,
            cessna
        }
    })
}

pub async fn get_for_take_off(parameters: Query<PerformanceParameters>) -> Response {
    if parameters.headwind_kts < 0 {
        get_error_response(&parameters)
    } else {
        let raw_html = get_raw_html_for_take_off(parameters.headwind_kts, parameters.temperature_f, parameters.elevation_ft, parameters.standard_temperature_f, parameters.is_grass);
        let page = ToPageTemplate {
            page_title:String::from("Cessna 150 J Take Off Performance"),
            raw_html
        };

        Html(page.render().unwrap()).into_response()
    }
}

#[derive(Template)]
#[template(path = "partials/aircraft/cessna150j/landing.html")]
pub struct LandingTemplate {
    is_grass: bool,
    calcs: Landing,
    cessna: Cessna150J
}

pub fn get_raw_html_for_landing(headwind_kts: i16, temperature_f: i16, elevation_ft: i16, standard_temperature_f: i16, is_grass: Option<bool>) -> String {
    get_raw_html(headwind_kts, temperature_f, elevation_ft, standard_temperature_f, &|cessna| {
        let calcs = cessna.calc_landing(); 
        LandingTemplate {
            is_grass: match is_grass { Some(value) => value, None => false },
            calcs,
            cessna
        }
    })
}

pub async fn get_response_for_landing(parameters: Query<PerformanceParameters>) -> Response {
    if parameters.headwind_kts < 0 {
        get_error_response(&parameters)
    } else {
        let raw_html = get_raw_html_for_landing(parameters.headwind_kts, parameters.temperature_f, parameters.elevation_ft, parameters.standard_temperature_f, parameters.is_grass);
        let page = ToPageTemplate {
            page_title:String::from("Cessna 150 J Landing Performance"),
            raw_html
        };

        Html(page.render().unwrap()).into_response()
    }
}