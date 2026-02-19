use axum::response::{NoContent, IntoResponse, Response, Json, Redirect};
use axum::routing::{get, post};
use axum::{Extension, Router, Form};
use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use askama::Template;
use sqlx::postgres::PgPool;

use super::super::types;
use super::super::repository;
use super::auth::{self, JwtSecret};
use super::super::templates;

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

/// Handle login form submission: validate credentials, set JWT cookie, redirect to home
pub async fn handle_login(
    State(pool): State<PgPool>,
    Extension(jwt_secret): Extension<JwtSecret>,
    jar: CookieJar,
    Form(form): Form<types::LoginForm>,
) -> Response {
    match repository::validate_password(form.username, form.password, &pool).await {
        Ok(user_info) => {
            match auth::create_token(&user_info.username, &user_info.uuid, &jwt_secret.0) {
                Ok(token) => {
                    let cookie = Cookie::build((auth::COOKIE_NAME, token))
                        .path("/")
                        .http_only(true)
                        .same_site(SameSite::Lax)
                        .build();
                    (jar.add(cookie), Redirect::to("/")).into_response()
                }
                Err(e) => {
                    log::error!("Failed to create JWT: {}", e);
                    let html = templates::LoginPage {
                        error: Some("Internal server error".to_string()),
                    }.render().unwrap();
                    (StatusCode::INTERNAL_SERVER_ERROR, axum::response::Html(html)).into_response()
                }
            }
        }
        Err(_) => {
            let html = templates::LoginPage {
                error: Some("Invalid username or password".to_string()),
            }.render().unwrap();
            (StatusCode::UNAUTHORIZED, axum::response::Html(html)).into_response()
        }
    }
}

/// Handle logout: clear the auth cookie and redirect to login
pub async fn handle_logout(jar: CookieJar) -> Response {
    let cookie = Cookie::build((auth::COOKIE_NAME, ""))
        .path("/")
        .build();
    (jar.remove(cookie), Redirect::to("/login")).into_response()
}

/// Handle create user form submission.
/// Always accessible when no users exist; requires auth when users exist.
pub async fn handle_create_user(
    State(pool): State<PgPool>,
    Extension(jwt_secret): Extension<JwtSecret>,
    jar: CookieJar,
    Form(form): Form<types::CreateUserForm>,
) -> Response {
    // Enforce auth when users already exist
    match repository::has_users(&pool).await {
        Ok(true) if !auth::is_authenticated(&jar, &jwt_secret.0) => {
            return Redirect::to("/login").into_response();
        }
        _ => {}
    }

    // Validate passwords match
    if form.password != form.confirm_password {
        let html = templates::CreateUserPage {
            error: Some("Passwords do not match".to_string()),
        }.render().unwrap();
        return (StatusCode::UNPROCESSABLE_ENTITY, axum::response::Html(html)).into_response();
    }

    match repository::create_user(form.username, form.password, &pool).await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(e) => {
            log::error!("Failed to create user: {}", e);
            let html = templates::CreateUserPage {
                error: Some(e),
            }.render().unwrap();
            (StatusCode::UNPROCESSABLE_ENTITY, axum::response::Html(html)).into_response()
        }
    }
}

/// Instantiate and setup routes for the API router. This router will handle endpoints for
/// the internal API, not meant to return HTML views.
pub fn make_api_router() -> Router<PgPool> {
    Router::new()
        .route("/card-types", get(get_card_types))
        .route("/languages/add", post(add_language))
        .route("/cards/add", post(add_card))
        .route("/decks/create", post(create_deck))
        .route("/decks/add-card", post(add_card_to_deck))
}
