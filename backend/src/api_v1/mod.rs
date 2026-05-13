/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

pub mod admin;
pub mod auth;
pub mod cards;
pub mod classrooms;
pub mod feedback;
pub mod images;
pub mod session;
pub mod trees;
pub mod users;

use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::Request;
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_governor::GovernorError;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::KeyExtractor;

use crate::ArcState;
use crate::api_v1::{
    admin::admin, cards::cards, classrooms::classrooms, feedback::feedback, images::images,
    trees::trees, users::users,
};
use auth::auth;
use axum::Router;

#[derive(Clone)]
pub struct IpKeyExtractor;

impl KeyExtractor for IpKeyExtractor {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        // Try XFF header to identify ip adress.
        if let Some(res) = req
            .headers()
            .get("X-Forwarded-For")
            .and_then(|header: &axum::http::HeaderValue| header.to_str().ok())
        {
            return Ok(res.to_string());
        }

        // Try socket address (TCP connection), if the XFF fails.
        if let Some(ConnectInfo(addr)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
            return Ok(addr.ip().to_string());
        }

        // Mockup for tests, where no TCP connection exists and reject if no IP found.
        if cfg!(test) {
            return Ok("127.0.0.1".to_string());
        }
        Err(GovernorError::UnableToExtractKey)
    }
}

/// Creates an `Axum` [`Router`] for the v1 API, combining all of its endpoint categories.
pub fn v1_api(state: ArcState) -> Router<ArcState> {
    Router::new()
        .nest("/admin", admin(state))
        .nest("/auth", auth())
        .nest("/cards", cards())
        .nest("/classrooms", classrooms())
        .nest("/feedback", feedback())
        .nest("/images", images())
        .nest("/trees", trees())
        .nest("/users", users())
}

// Compute governor config (per minute).
// Using miliseconds to compute req/min.
pub fn governor_conf_minute(
    refresh_rate: u64,
    burst_size: u32,
) -> GovernorLayer<IpKeyExtractor, NoOpMiddleware<QuantaInstant>, Body> {
    let config = Arc::new(
        GovernorConfigBuilder::default()
            .period(Duration::from_millis(60_000 / refresh_rate))
            .burst_size(burst_size)
            .key_extractor(IpKeyExtractor)
            .finish()
            .unwrap(),
    );
    GovernorLayer::new(config)
}
