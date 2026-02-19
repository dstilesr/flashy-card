mod views;
mod api;
mod auth;

use super::Args;
use sqlx::postgres;
use axum::{Extension, Router};
use axum::routing::{get, post};
use axum::middleware;
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

    super::repository::migrate(&pool).await;

    let jwt_secret = auth::JwtSecret(auth::get_jwt_secret());

    // Protected routes: require a valid JWT cookie
    let protected = Router::new()
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
        .nest("/api", api::make_api_router())
        .layer(middleware::from_fn_with_state(pool.clone(), auth::require_auth));

    // Public routes: no auth required (individual handlers may enforce conditional auth)
    let public = Router::new()
        .route("/login", get(views::login_page))
        .route("/create-user", get(views::create_user_page))
        .route("/api/login", post(api::handle_login))
        .route("/api/logout", get(api::handle_logout))
        .route("/api/create-user", post(api::handle_create_user))
        .route("/api/health", get(api::health))
        .nest_service("/static", ServeDir::new(args.static_dir));

    public
        .merge(protected)
        .layer(Extension(jwt_secret))
        .with_state(pool)
}
