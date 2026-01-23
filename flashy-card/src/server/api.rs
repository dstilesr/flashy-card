use axum::response::{NoContent, IntoResponse, Response, Json, Redirect};
use axum::routing::{get, post};
use axum::{Router, Form};
use sqlx::postgres::PgPool;
use axum::extract::State;
use axum::http::StatusCode;

use super::super::types;
use super::super::repository;

/// Health check endpoint function
pub async fn health() -> Response {
    log::debug!("Health: OK");
    NoContent.into_response()
}


/// Get a list of the possible card types
pub async fn get_card_types(State(pool): State<PgPool>) -> (StatusCode, Response) {
    match repository::list_card_types(&pool).await {
        Err(error) => {
            log::error!("Error in list cards endpoint.");
            let err_rsp = Json(types::ErrorResponse{error});
            (StatusCode::INTERNAL_SERVER_ERROR, err_rsp.into_response())
        }
        Ok(card_types) => {
            log::debug!("Got {} card types", card_types.len());
            let response = Json(types::CardTypeList{card_types});
            (StatusCode::OK, response.into_response())
        }
    }
}

/// Add a new language from form data and redirect to languages page
pub async fn add_language(
    State(pool): State<PgPool>,
    Form(form): Form<types::AddLanguageForm>,
) -> Response {
    log::debug!("Adding language: {}", form.name);

    match repository::add_language(form.name, form.description, &pool).await {
        Ok(_) => {
            log::info!("Language added successfully, redirecting to /languages");
            Redirect::to("/languages").into_response()
        }
        Err(error) => {
            log::error!("Failed to add language: {}", error);
            // Redirect to error page with the error title
            let error_url = format!("/error?title={}", urlencoding::encode(&error));
            Redirect::to(&error_url).into_response()
        }
    }
}

/// Instantiate and setup routes for the API router. This router will handle endpoints for
/// the internal API, not meant to return HTML views.
pub fn make_api_router() -> Router<PgPool> {
    Router::new()
        .route("/health", get(health))
        .route("/card-types", get(get_card_types))
        .route("/languages/add", post(add_language))
}
