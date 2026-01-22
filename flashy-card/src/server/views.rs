use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use super::super::templates;

/// Render the home page of the application and return response
pub async fn home_page() -> Response {
    let content = templates::HomePage.render().unwrap();
    Html(content).into_response()
}
