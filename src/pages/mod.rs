use askama::Template;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response}
};

pub mod airport;
pub mod runway;

pub mod cessna150j;

fn resolve_boolean(opt: Option<bool>) -> bool {
    match  opt {
        Some(value) => value,
        None => false
    }
}

pub struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T> where T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            ).into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "to_page.html", escape="none")]
pub struct ToPageTemplate {
    page_title: String,
    raw_html: String
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    message: String
}

impl ErrorTemplate{
    pub fn new(message: String) -> Self {
        ErrorTemplate {
            message
        }
    }
}