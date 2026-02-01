mod views;
mod api;

use super::Args;
use sqlx::postgres;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;



/// Create a new router to serve requests
pub async fn create_router(args: Args) -> Router {

    // Initialize the DB connection pool
    let conn_string = std::env::var("DATABASE_URL")
        .expect("Found no DATABASE_URL in environment");
    let conn_opts = postgres::PgPoolOptions::new()
        .max_connections(args.max_db_connections);
    let pool = conn_opts.connect(&conn_string).await
        .expect("Can't connect to database");

    let api_route = api::make_api_router();

    // Setup Router and Routes
    Router::new()
        .nest_service("/static", ServeDir::new(args.static_dir))
        .nest("/api", api_route)
        .route("/", get(views::home_page))
        .route("/languages", get(views::render_languages_page))
        .route("/decks", get(views::render_all_decks_page))
        .route("/decks/new", get(views::render_create_deck_page))
        .route("/decks/{deck_slug}/cards", get(views::render_deck_cards_page))
        .route("/{language_slug}/decks", get(views::render_language_decks_page))
        .route("/{language_slug}/decks/{deck_slug}/edit", get(views::render_edit_deck_page))
        .route("/{language_slug}/cards", get(views::render_language_cards_page))
        .route("/{language_slug}/add-card", get(views::render_add_card_page))
        .route("/error", get(views::error_page))
        .with_state(pool)
}
