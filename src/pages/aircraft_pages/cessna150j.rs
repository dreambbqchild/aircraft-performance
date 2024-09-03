use askama::Template;
use axum::{extract::Query, response::{Html, IntoResponse, Response}};

use crate::{
    data::performance::aircraft::cessna150j::{Cessna150J, Landing, TakeOff},
    pages::{ErrorTemplate, ToPageTemplate}
};

use super::{PerformanceParameters, QueryPerformanceParameters};

#[derive(Template)]
#[template(path = "partials/aircraft/cessna150j/take-off.html")]
pub struct TakeOffTemplate {
    start_landing_flow: bool,
    is_grass: bool,
    calcs: TakeOff,
    cessna: Cessna150J
}

fn get_tailwind_error_response(parameters: &PerformanceParameters) -> Response {
    let tailwind = parameters.headwind.knots().abs();
    let template = ErrorTemplate::new(format!("Tailind of {tailwind} kts detected. Unable to compute."));
    Html(template.render().unwrap()).into_response()
}

fn get_raw_html<T>(parameters: &PerformanceParameters, callback: &dyn Fn(Cessna150J) -> T) -> String where T : Template {
    let cessna = Cessna150J::new(parameters.headwind, parameters.temperature.fahrenheit(), parameters.elevation_ft, parameters.standard_temperature.fahrenheit());

    let template = callback(cessna);
    template.render().unwrap()
}

pub fn get_raw_html_for_take_off(parameters: &PerformanceParameters, start_landing_flow: bool) -> String {
    get_raw_html(parameters, &|cessna| {
        let calcs = cessna.calc_take_off(); 
        TakeOffTemplate {
            start_landing_flow,
            is_grass: parameters.is_grass,
            calcs,
            cessna
        }
    })
}

pub async fn get_for_take_off(query_parameters: Query<QueryPerformanceParameters>) -> Response {
    let parameters = query_parameters.to_performance_parameters();
    if parameters.headwind.knots() < 0 {
        get_tailwind_error_response(&parameters)
    } else {
        let raw_html = get_raw_html_for_take_off(&parameters, false);
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

pub fn get_raw_html_for_landing(parameters: &PerformanceParameters) -> String {
    get_raw_html(&parameters, &|cessna| {
        let calcs = cessna.calc_landing(); 
        LandingTemplate {
            is_grass: parameters.is_grass,
            calcs,
            cessna
        }
    })
}

pub async fn get_for_landing(query_parameters: Query<QueryPerformanceParameters>) -> Response {
    let parameters = query_parameters.to_performance_parameters();
    if parameters.headwind.knots() < 0 {
        get_tailwind_error_response(&parameters)
    } else {
        let raw_html = get_raw_html_for_landing(&parameters);
        let page = ToPageTemplate {
            page_title:String::from("Cessna 150 J Landing Performance"),
            raw_html
        };

        Html(page.render().unwrap()).into_response()
    }
}