use serde::Deserialize;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::extract::{Query, State};
use sqlx::PgPool;
use super::super::{templates, repository};


fn default_page() -> i32 {
    1
}

/// Pagination parameters
#[derive(Deserialize)]
pub struct Paginate {

    #[serde(default = "default_page")]
    page: i32,
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
