/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![warn(clippy::pedantic)]

mod api_v1;
mod config;
mod db;
mod db_structs;
mod errors;
mod tests;

use crate::{api_v1::v1_api, config::Config};
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use tracing::log::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

#[allow(clippy::needless_for_each)]
mod api_doc {
    use super::{OpenApi, utoipauto};

    // Automatically reads all API docs written with `utoipa` for all API endpoints
    // and converts into OpenAPI documentation.
    #[utoipauto]
    #[derive(OpenApi)]
    #[openapi()]
    pub struct ApiDoc;
}

#[tokio::main]
async fn main() {
    // Loading environment variables from the `.env` file is optional,
    // because for example the CI configures it directly so the `.env` file is not present.
    if let Err(err) = dotenvy::dotenv() {
        assert!(
            err.not_found(),
            "The .env file was found but could not be loaded: {err}"
        );
    }
    let config = Config::load();

    // Set up logging.
    tracing_subscriber::registry()
        .with(config.log_level.clone())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our application with a route.
    let app = Router::new()
        .merge(static_frontend())
        .nest("/api", backend(config.clone()).await)
        // Add documentation routes.
        .merge(
            SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", api_doc::ApiDoc::openapi()),
        );

    // Serve the application.
    let addr = (config.listening_ip.as_str(), config.listening_port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| {
            panic!(
                "Could not create tcp listener on '{}:{}', because: {err}",
                addr.0, addr.1
            )
        });

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
async fn backend(config: Config) -> Router {
    // Set up and migrate database.
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to database.");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("failed to run database migrations.");

    Router::new()
        .nest("/v1", v1_api())
        .layer(Extension(db))
        .with_state(config)
}
