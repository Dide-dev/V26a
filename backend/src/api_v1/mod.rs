/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

pub mod admin;
pub mod auth;
pub mod cards;
pub mod feedback;
pub mod images;
pub mod session;
pub mod trees;
pub mod users;

use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use std::sync::Arc;
use std::time::Duration;
use tower_governor::governor::GovernorConfig;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;

use crate::Config;
use crate::api_v1::{
    admin::admin, cards::cards, feedback::feedback, images::images, trees::trees, users::users,
};
use auth::auth;
use axum::Router;

/// Creates an `Axum` [`Router`] for the v1 API, combining all of its endpoint categories.
pub fn v1_api(config: Config) -> Router<Config> {
    Router::new()
        .nest("/admin", admin(config))
        .nest("/auth", auth())
        .nest("/trees", trees())
        .nest("/cards", cards())
        .nest("/users", users())
        .nest("/images", images())
        .nest("/feedback", feedback())
}

// Compute governor config (per minute).
// Using miliseconds to compute req/min.
pub fn governor_conf_minute(
    refresh_rate: u64,
    burst_size: u32,
) -> Arc<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware<QuantaInstant>>> {
    Arc::new(
        GovernorConfigBuilder::default()
            .period(Duration::from_millis(60_000 / refresh_rate))
            .burst_size(burst_size)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    )
}
