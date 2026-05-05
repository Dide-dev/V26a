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

use crate::errors::ApiError;
use crate::{api_v1::v1_api, config::Config};
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::GlobalKeyExtractor;
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

    // Build our application with a route.
    let app = Router::new()
        .nest_service("/uploads", ServeDir::new(&config.upload_dir))
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

/// Creates an `Axum` [`Router`] for the backend API, combining all the API versions.
/// Also sets up the global configuration and the database.
async fn backend(config: Config) -> Router {
    // Map status code 429 to api-error.
    fn map_response_error(res: Response) -> Response {
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            return ApiError::RateLimitExceeded.into_response();
        }
        res
    }

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

    // Extract governor config.
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(config.rate_limit_per_second)
            .burst_size(config.rate_limit_burst_size)
            .key_extractor(GlobalKeyExtractor)
            .finish()
            .unwrap(),
    );

    // Create governor layer.
    let governor_layer = ServiceBuilder::new()
        .map_response(map_response_error)
        .layer(GovernorLayer::new(governor_conf.clone()));

    // Cleanup to prevent high memory usage.
    let governor_limiter = governor_conf.limiter().clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            governor_limiter.retain_recent();
        }
    });

    Router::new()
        .nest("/v1", v1_api(config.clone()))
        .layer(governor_layer)
        .layer(Extension(db))
        .layer(DefaultBodyLimit::max(config.max_upload_size)) // If this limit is for some reason removed, a check for max file size must be added to the `add_new_image` function on the images/add endpoint.
        .with_state(config)
}
