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
