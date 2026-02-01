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

/// Add a new card from form data and redirect to languages page
pub async fn add_card(
    State(pool): State<PgPool>,
    Form(form): Form<types::AddCardForm>,
) -> Response {
    log::debug!("Adding card: {} for language {}", form.target, form.language_slug);

    match repository::add_card(form, &pool).await {
        Ok(_) => {
            log::info!("Card added successfully, redirecting to /languages");
            Redirect::to("/languages").into_response()
        }
        Err(error) => {
            log::error!("Failed to add card: {}", error);
            // Redirect to error page with the error title
            let error_url = format!("/error?title={}", urlencoding::encode(&error));
            Redirect::to(&error_url).into_response()
        }
    }
}

/// Create a new deck from form data and redirect to edit page
pub async fn create_deck(
    State(pool): State<PgPool>,
    Form(form): Form<types::AddDeckForm>,
) -> Response {
    log::debug!("Creating deck: {} for language {}", form.name, form.language_slug);

    match repository::create_deck(
        form.language_slug.clone(),
        form.name,
        form.description,
        &pool
    ).await {
        Ok(deck_slug) => {
            log::info!("Deck created successfully, redirecting to edit page");
            // Redirect to edit page for the new deck
            let edit_url = format!("/{}/decks/{}/edit", form.language_slug, deck_slug);
            Redirect::to(&edit_url).into_response()
        }
        Err(error) => {
            log::error!("Failed to create deck: {}", error);
            let error_url = format!("/error?title={}", urlencoding::encode(&error));
            Redirect::to(&error_url).into_response()
        }
    }
}

/// Add a card to a deck and redirect back to edit page with same pagination/filter
pub async fn add_card_to_deck(
    State(pool): State<PgPool>,
    Form(form): Form<types::AddCardToDeckForm>,
) -> Response {
    log::debug!("Adding card {} to deck {}", form.card_id, form.deck_slug);

    match repository::add_card_to_deck(
        form.card_id,
        &form.deck_slug,
        &form.language_slug,
        &pool
    ).await {
        Ok(_) => {
            log::info!("Card added to deck successfully");
            // Build redirect URL preserving pagination and filter
            let mut redirect_url = format!(
                "/{}/decks/{}/edit?page={}",
                form.language_slug,
                form.deck_slug,
                form.page
            );
            if let Some(tf) = form.type_filter {
                redirect_url.push_str(&format!("&type_filter={}", tf));
            }
            Redirect::to(&redirect_url).into_response()
        }
        Err(error) => {
            log::error!("Failed to add card to deck: {}", error);
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
        .route("/cards/add", post(add_card))
        .route("/decks/create", post(create_deck))
        .route("/decks/add-card", post(add_card_to_deck))
}
