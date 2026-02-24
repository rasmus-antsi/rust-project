use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn login_page() -> impl IntoResponse {
    match LoginTemplate.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to render login page",
        )
            .into_response(),
    }
}

pub async fn register_page() -> impl IntoResponse {
    match RegisterTemplate.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to render register page",
        )
            .into_response(),
    }
}

pub async fn index_page() -> impl IntoResponse {
    match IndexTemplate.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to render index page",
        )
            .into_response(),
    }
}
