mod views;

use super::Args;
use sqlx::postgres;
use axum::Router;
use axum::routing::{get, post};
use tower_http::services::ServeDir;

/// Application Server
pub struct Server {
    pool: postgres::PgPool,
    static_dir: String,
}

impl Server {

    /// Initialize a new server. This will create the router and start the connection pool,
    /// and return the initialized server.
    pub async fn new(args: Args) -> Self {
        let conn_string = std::env::var("DATABASE_URL")
            .expect("Found no DATABASE_URL in environment");


        let conn_opts = postgres::PgPoolOptions::new()
            .max_connections(args.max_db_connections);
        let pool = conn_opts.connect(&conn_string).await
            .expect("Can't connect to database");

        Self { pool, static_dir: args.static_dir }
    }

    /// Setup the router to handle requests
    fn setup_router(static_files_path: &str) -> Router{
        Router::new()
            .nest_service("/static", ServeDir::new(static_files_path))
            .route("/", get(views::home_page))
    }

    /// Start the server with the given listener
    pub async fn serve(&self, listener: tokio::net::TcpListener) {
        log::info!("Starting server...");
        let router = Self::setup_router(&self.static_dir);
        axum::serve(listener, router).await.unwrap()
    }
}
