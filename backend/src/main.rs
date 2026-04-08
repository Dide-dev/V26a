/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![warn(clippy::pedantic)]

mod api_v1;
mod db;
mod db_structs;
mod errors;
mod tests;

use crate::api_v1::{session::Keys, v1_api};
use axum::extract::DefaultBodyLimit;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::services::ServeDir;
use tracing::log::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct Config {
    session_duration: i64,
    jwt_keys: Keys,
    upload_dir: String,
}

impl Config {
    /// Loads the global server configuration from the environment.
    ///
    /// # Panics
    /// This may panic if any of the server parameters have an invalid format.
    #[must_use]
    pub fn load() -> Self {
        Self {
            session_duration: env::var("SESSION_EXPIRATION_SECONDS")
                // Defaults to a week.
                .unwrap_or("604800".to_string())
                .parse()
                .expect("Failed to parse SESSION_EXPIRATION_SECONDS"),
            jwt_keys: Keys::new(
                env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set")
                    .as_bytes(),
            ),
            upload_dir: env::var("UPLOAD_DIR").unwrap_or("./uploads".to_string()),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file missing."); // TODO: this .expect() needs to be replaced when deployed
    // Set up logging.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our application with a route.
    let app = Router::new()
        .nest_service("/uploads", ServeDir::new("./uploads"))
        .merge(static_frontend())
        .nest("/api", backend().await);

    // Serve the application.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Creates an `Axum` [`Router`] that serves the static frontend from the `frontend` folder.
fn static_frontend() -> Router {
    use tower_http::services::ServeDir;
    let static_frontend_dir = ServeDir::new("../frontend/build");

    Router::new().fallback_service(static_frontend_dir)
}

/// Creates an `Axum` [`Router`] for the backend API, combining all the API versions.
/// Also sets up the global configuration and the database.
async fn backend() -> Router {
    // Set up global configurations.
    let config = Config::load();

    // Set up and migrate database.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("failed to connect to database.");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("failed to run database migrations.");

    Router::new()
        .nest("/v1", v1_api())
        .layer(Extension(db))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .with_state(config)
}
