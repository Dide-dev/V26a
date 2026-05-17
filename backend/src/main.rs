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
mod state;
mod tests;

use crate::api_v1::rate_limit_layer;
use crate::errors::ApiError;
use crate::{api_v1::v1_api, config::Config, state::ArcState};
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
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

    let db = setup_db(&config.database_url).await;

    // Build our application with a route.
    let app = Router::new()
        .nest_service("/uploads", ServeDir::new(&config.upload_dir))
        .nest("/api", backend(config.clone(), db))
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

/// Sets up and migrates the database.
async fn setup_db(database_url: &str) -> sqlx::PgPool {
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .expect("failed to connect to database.");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("failed to run database migrations.");

    db
}

/// Creates an `Axum` [`Router`] for the backend API, combining all the API versions.
/// Also sets up the global configuration and the database.
fn backend(config: Config, db: sqlx::PgPool) -> Router {
    // Map status code 429 to api-error.
    fn map_response_error(res: Response) -> Response {
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            return ApiError::RateLimitExceeded.into_response();
        }
        res
    }

    let state = ArcState::new(config);

    // Extract ratelimiter config (per minute) &  create governor layer (minute)
    let governor_layer = ServiceBuilder::new()
        .map_response(map_response_error)
        .layer(rate_limit_layer(
            state.config.rate_limit_per_minute,
            state.config.rate_limit_burst_size,
        ));

    Router::new()
        .nest("/v1", v1_api(state.clone()))
        .layer(governor_layer)
        .layer(Extension(db))
        .layer(DefaultBodyLimit::max(state.config.max_upload_size)) // If this limit is for some reason removed, a check for max file size must be added to the `add_new_image` function on the images/add endpoint.
        .with_state(state)
}
