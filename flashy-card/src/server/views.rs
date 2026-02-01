use serde::Deserialize;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::extract::{Query, State, Path};
use sqlx::PgPool;
use super::super::{templates, repository};


fn default_page() -> i32 {
    1
}

fn default_error_title() -> String {
    "Unable to Complete Request".to_string()
}

/// Pagination parameters
#[derive(Deserialize)]
pub struct Paginate {

    #[serde(default = "default_page")]
    page: i32,
}

/// Error page query parameters
#[derive(Deserialize)]
pub struct ErrorQuery {

    #[serde(default = "default_error_title")]
    pub title: String,
}

/// Query parameters for create deck page (optional preselection)
#[derive(Deserialize)]
pub struct CreateDeckQuery {
    pub language: Option<String>,
}

/// Query parameters for the deck edit page
#[derive(Deserialize)]
pub struct DeckEditQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    pub type_filter: Option<i32>,
}

/// Render the error page into a response with the given status code.
pub fn render_error_page(err_title: String, err_msg: String, status_code: StatusCode) -> Response {
    let error_page = templates::ErrorPage {
        title: err_title,
        description: err_msg,
    };
    let content = error_page.render().unwrap();
    (status_code, Html(content)).into_response()
}

/// Render the home page of the application and return response
pub async fn home_page() -> Response {
    let content = templates::HomePage.render().unwrap();
    Html(content).into_response()
}

/// Render the error page with the given title from query parameters
pub async fn error_page(Query(error_query): Query<ErrorQuery>) -> Response {
    render_error_page(
        error_query.title,
        String::from("Unable to complete request"),
        StatusCode::BAD_REQUEST,
    )
}

/// Render the languages list page
pub async fn render_languages_page(State(pool): State<PgPool>, Query(pagination): Query<Paginate>) -> Response {
    if pagination.page <= 0 {
        return render_error_page(
            String::from("Invalid Page Number"),
            String::from("Page must be greater than 0."),
            StatusCode::BAD_REQUEST,
        );
    }
    match repository::get_languages(pagination.page, &pool).await {
        Err(e) => render_error_page(
            String::from("Error Getting Languages"),
            format!("{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        Ok((languages, has_next)) => {
            let template = templates::LanguagesPage{languages, has_next, page: pagination.page};
            Html(template.render().unwrap()).into_response()
        }
    }
}

/// Render the decks list page for all languages
pub async fn render_all_decks_page(State(pool): State<PgPool>, Query(pagination): Query<Paginate>) -> Response {
    if pagination.page <= 0 {
        return render_error_page(
            String::from("Invalid Page Number"),
            String::from("Page must be greater than 0."),
            StatusCode::BAD_REQUEST,
        );
    }
    match repository::list_decks(None, pagination.page, &pool).await {
        Err(e) => render_error_page(
            String::from("Error Getting Decks"),
            format!("{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        Ok((decks, has_next)) => {
            let template = templates::DecksPage {
                base_url: String::from("/decks"),
                decks,
                page: pagination.page,
                has_next,
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

/// Render the decks list page for a specific language
pub async fn render_language_decks_page(
    State(pool): State<PgPool>,
    Path(language_slug): Path<String>,
    Query(pagination): Query<Paginate>,
) -> Response {
    if pagination.page <= 0 {
        return render_error_page(
            String::from("Invalid Page Number"),
            String::from("Page must be greater than 0."),
            StatusCode::BAD_REQUEST,
        );
    }
    match repository::list_decks(Some(language_slug.clone()), pagination.page, &pool).await {
        Err(e) => render_error_page(
            String::from("Error Getting Decks"),
            format!("{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        Ok((decks, has_next)) => {
            let template = templates::DecksPage {
                base_url: format!("/{}/decks", language_slug),
                decks,
                page: pagination.page,
                has_next,
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

/// Render the add card form page for a specific language
pub async fn render_add_card_page(
    State(pool): State<PgPool>,
    Path(language_slug): Path<String>,
) -> Response {
    // Fetch the language name from the slug
    let language_name = match repository::get_language_name(&language_slug, &pool).await {
        Ok(name) => name,
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            return render_error_page(
                String::from("Error Getting Language"),
                e,
                status,
            );
        }
    };

    // Fetch card types
    match repository::list_card_types(&pool).await {
        Err(e) => render_error_page(
            String::from("Error Getting Card Types"),
            format!("{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        Ok(card_types) => {
            let template = templates::AddCardPage {
                language_slug,
                language_name,
                card_types,
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

/// Render the create deck form page
pub async fn render_create_deck_page(
    State(pool): State<PgPool>,
    Query(query): Query<CreateDeckQuery>,
) -> Response {
    // Fetch all languages for the dropdown
    // For simplicity, fetch from page 1; if there are many languages, this could be improved
    match repository::get_languages(1, &pool).await {
        Err(e) => render_error_page(
            String::from("Error Getting Languages"),
            format!("{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        Ok((languages, _)) => {
            let template = templates::CreateDeckPage {
                languages,
                preselected_language: query.language,
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

/// Render the deck edit page showing available cards to add
pub async fn render_edit_deck_page(
    State(pool): State<PgPool>,
    Path((language_slug, deck_slug)): Path<(String, String)>,
    Query(query): Query<DeckEditQuery>,
) -> Response {
    if query.page <= 0 {
        return render_error_page(
            String::from("Invalid Page Number"),
            String::from("Page must be greater than 0."),
            StatusCode::BAD_REQUEST,
        );
    }

    // 1. Get deck info
    let deck = match repository::get_deck_info(&language_slug, &deck_slug, &pool).await {
        Ok(d) => d,
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            return render_error_page(
                String::from("Error Getting Deck"),
                e,
                status,
            );
        }
    };

    // 2. Get card types for filter dropdown
    let card_types = match repository::list_card_types(&pool).await {
        Ok(ct) => ct,
        Err(e) => {
            return render_error_page(
                String::from("Error Getting Card Types"),
                e,
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }
    };

    // 3. Get available cards (not in deck)
    match repository::list_available_cards_for_deck(
        deck.id,
        &language_slug,
        query.type_filter,
        query.page,
        &pool
    ).await {
        Err(e) => render_error_page(
            String::from("Error Getting Cards"),
            format!("{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        Ok((cards, has_next)) => {
            // Build base_url for pagination
            let base_url = if let Some(tf) = query.type_filter {
                format!("/{}/decks/{}/edit?type_filter={}", language_slug, deck_slug, tf)
            } else {
                format!("/{}/decks/{}/edit", language_slug, deck_slug)
            };

            let template = templates::EditDeckPage {
                deck,
                cards,
                card_types,
                page: query.page,
                has_next,
                type_filter: query.type_filter,
                base_url,
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

/// Render the cards page for a specific language
pub async fn render_language_cards_page(
    State(pool): State<PgPool>,
    Path(language_slug): Path<String>,
    Query(pagination): Query<Paginate>,
) -> Response {
    if pagination.page <= 0 {
        return render_error_page(
            String::from("Invalid Page Number"),
            String::from("Page must be greater than 0."),
            StatusCode::BAD_REQUEST,
        );
    }

    // Fetch language name
    let language_name = match repository::get_language_name(&language_slug, &pool).await {
        Ok(name) => name,
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            return render_error_page(
                String::from("Error Getting Language"),
                e,
                status,
            );
        }
    };

    // Fetch cards for the language
    match repository::list_cards(language_slug.clone(), pagination.page, &pool).await {
        Err(e) => render_error_page(
            String::from("Error Getting Cards"),
            format!("{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        Ok((cards, has_next)) => {
            let template = templates::CardsPage {
                cards,
                page: pagination.page,
                has_next,
                base_url: format!("/{}/cards", language_slug),
                title: format!("Cards in {}", language_name),
                subtitle: format!("All flashcards for {}", language_name),
                back_url: format!("/{}/decks", language_slug),
                back_label: String::from("Back to Decks"),
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

/// Render the cards page for a specific deck
pub async fn render_deck_cards_page(
    State(pool): State<PgPool>,
    Path(deck_slug): Path<String>,
    Query(pagination): Query<Paginate>,
) -> Response {
    if pagination.page <= 0 {
        return render_error_page(
            String::from("Invalid Page Number"),
            String::from("Page must be greater than 0."),
            StatusCode::BAD_REQUEST,
        );
    }

    // Fetch deck info
    let deck = match repository::get_deck_by_slug(&deck_slug, &pool).await {
        Ok(d) => d,
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            return render_error_page(
                String::from("Error Getting Deck"),
                e,
                status,
            );
        }
    };

    // Fetch cards for the deck
    match repository::list_cards_for_deck(&deck_slug, pagination.page, &pool).await {
        Err(e) => render_error_page(
            String::from("Error Getting Cards"),
            format!("{}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        Ok((cards, has_next)) => {
            let template = templates::CardsPage {
                cards,
                page: pagination.page,
                has_next,
                base_url: format!("/decks/{}/cards", deck_slug),
                title: format!("Cards in {}", deck.name),
                subtitle: format!("{} - {}", deck.language_name, deck.name),
                back_url: format!("/{}/decks", deck.language_slug),
                back_label: String::from("Back to Decks"),
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}
