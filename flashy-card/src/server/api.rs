use axum::response::{NoContent, IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use sqlx::postgres::PgPool;

/// Health check endpoint function
pub async fn health() -> Response {
    log::debug!("Health: OK");
    NoContent.into_response()
}

/// Instantiate and setup routes for the API router. This router will handle endpoints for
/// the internal API, not meant to return HTML views.
pub fn make_api_router() -> Router<PgPool> {
    Router::new().route("/health", get(health))
}
