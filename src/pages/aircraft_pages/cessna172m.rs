use askama::Template;
use axum::{extract::Query, response::{Html, IntoResponse, Response}};

use crate::{data::performance::aircraft::cessna172m::{Cessna172M, Performance}, pages::{ErrorTemplate, ToPageTemplate}};

use super::{PerformanceParameters, QueryPerformanceParameters};

#[derive(Template)]
#[template(path = "partials/aircraft/cessna172m/performance.html")]
pub struct PerformanceTemplate {
    start_landing_flow: bool,
    is_grass: bool,
    performance: Performance,
    cessna: Cessna172M
}

fn get_tailwind_error_response(parameters: &PerformanceParameters) -> Response {
    let tailwind = parameters.headwind.knots().abs();
    let template = ErrorTemplate::new(format!("Tailind of {tailwind} kts detected which is greater than the limit of 10 kts. Unable to compute."));
    Html(template.render().unwrap()).into_response()
}

fn get_raw_html<T>(parameters: &PerformanceParameters, callback: &dyn Fn(Cessna172M) -> T) -> String where T : Template {
    let cessna = Cessna172M::new(parameters.headwind, parameters.elevation_ft, parameters.pressure, parameters.temperature.celsius());

    let template = callback(cessna);
    template.render().unwrap()
}

pub fn get_raw_html_for_take_off(parameters: &PerformanceParameters, start_landing_flow: bool) -> String {
    get_raw_html(parameters, &|cessna| {
        let performance = cessna.calc_take_off(2100/*parameters.aircraft_weight_lbs.expect("The take off weight of the aircraft is requried for the calculation")*/); 
        PerformanceTemplate {
            start_landing_flow,
            is_grass: parameters.is_grass,
            performance,
            cessna
        }
    })
}

pub async fn get_for_take_off(query_parameters: Query<QueryPerformanceParameters>) -> Response {
    let parameters = query_parameters.to_performance_parameters();
    if parameters.headwind.knots() < 10 {
        get_tailwind_error_response(&parameters)
    } else {
        let raw_html = get_raw_html_for_take_off(&parameters, false);
        let page = ToPageTemplate {
            page_title:String::from("Cessna 172 M Take Off Performance"),
            raw_html
        };

        Html(page.render().unwrap()).into_response()
    }
}

pub fn get_raw_html_for_landing(parameters: &PerformanceParameters) -> String {
    get_raw_html(&parameters, &|cessna| {
        let performance = cessna.calc_landing(); 
        PerformanceTemplate {
            is_grass: parameters.is_grass,
            start_landing_flow: false,
            performance,
            cessna
        }
    })
}

pub async fn get_for_landing(query_parameters: Query<QueryPerformanceParameters>) -> Response {
    let parameters = query_parameters.to_performance_parameters();

    if parameters.headwind.knots() < 10 {
        get_tailwind_error_response(&parameters)
    } else {
        let raw_html = get_raw_html_for_landing(&parameters);
        let page = ToPageTemplate {
            page_title:String::from("Cessna 172 M Landing Performance"),
            raw_html
        };

        Html(page.render().unwrap()).into_response()
    }
}