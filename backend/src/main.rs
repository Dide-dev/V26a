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

use crate::api_v1::auth::auth;
use crate::api_v1::feedback::feedback;
use crate::api_v1::images::images;
use crate::api_v1::users::add_new_user;
use crate::errors::ApiError;
use crate::{api_v1::v1_api, config::Config};
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::post;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;
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

    // Extract governor config (per second).
    let governor_conf_second = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(config.rate_limit_per_second)
            .burst_size(config.rate_limit_burst_size_second)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    // Extract governor config (per minute).
    // Divided by 60 to get the per-second refill rate.
    // Values below 60 req/min will clamp to 1 req/sec
    let governor_conf_minute = Arc::new(
        GovernorConfigBuilder::default()
            .per_second((config.rate_limit_per_minute / 60).max(1))
            .burst_size(config.rate_limit_burst_size_minute)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    // Extract strict gover config per second
    let governor_conf_second_strict = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(config.rate_limit_per_second_strict)
            .burst_size(config.rate_limit_burst_size_second_strict)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    let governor_conf_minute_strict = Arc::new(
        GovernorConfigBuilder::default()
            .per_second((config.rate_limit_per_minute_strict / 60).max(1))
            .burst_size(config.rate_limit_burst_size_minute_strict)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    // Create governor layer (second)
    let governor_layer_second = ServiceBuilder::new()
        .map_response(map_response_error)
        .layer(GovernorLayer::new(governor_conf_second.clone()));

    // Create governor layer (minute)
    let governor_layer_minute = ServiceBuilder::new()
        .map_response(map_response_error)
        .layer(GovernorLayer::new(governor_conf_minute.clone()));

    // Create strict governor layers
    // These layers are applied on the following endpoints: login, feedback, add images, add user.
    let governor_layer_second_strict = ServiceBuilder::new()
        .map_response(map_response_error)
        .layer(GovernorLayer::new(governor_conf_second_strict.clone()));

    let governor_layer_minute_strict = ServiceBuilder::new()
        .map_response(map_response_error)
        .layer(GovernorLayer::new(governor_conf_minute_strict.clone()));

    // Clone layers

    // Apply the strict layers directly on specific endpoints/routes.
    let strict_endpoints = Router::new()
        .route("/api/v1/users/add", post(add_new_user))
        .nest("/api/v1/auth", auth())
        .nest("/api/v1/feedback", feedback())
        .nest("/api/v1/images", images())
        .layer(governor_layer_second_strict)
        .layer(governor_layer_minute_strict);

    // Cleanup to prevent high memory usage.
    for limiter in [
        governor_conf_second.limiter().clone(),
        governor_conf_minute.limiter().clone(),
        governor_conf_second_strict.limiter().clone(),
        governor_conf_minute_strict.limiter().clone(),
    ] {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                limiter.retain_recent();
            }
        });
    }

    Router::new()
        .merge(strict_endpoints)
        .nest(
            "/v1",
            v1_api(config.clone())
                .layer(governor_layer_second)
                .layer(governor_layer_minute),
        )
        .layer(Extension(db))
        .layer(DefaultBodyLimit::max(config.max_upload_size)) // If this limit is for some reason removed, a check for max file size must be added to the `add_new_image` function on the images/add endpoint.
        .with_state(config)
}
