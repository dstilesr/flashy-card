mod views;

use super::Args;
use sqlx::postgres;
use axum::Router;
use axum::routing::{get, post};
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

    // Setup Router and Routes
    Router::new()
        .with_state(pool)
        .nest_service("/static", ServeDir::new(args.static_dir))
        .route("/", get(views::home_page))
}
