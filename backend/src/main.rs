#![warn(clippy::pedantic)]

mod api_v1;
mod db;
mod db_structs;

use crate::api_v1::v1_api;
use axum::{Extension, Router};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing::log::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file missing."); // TODO: this .expect() needs to be replaced when deployed
    // set up logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    // build our application with a route
    let app = Router::new()
        .merge(static_frontend())
        .nest("/api", backend(db));

    // run it
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
///
/// # Arguments
/// `db` - An `SQLx` [`PgPool`] that represents the connection to a Postgres database,
/// to be passed as an extension to the Routers.
fn backend(db: PgPool) -> Router {
    Router::new().nest("/v1", v1_api()).layer(Extension(db))
}
