mod auth;
mod demo;
pub mod session;
mod trees;

use crate::Config;
use crate::api_v1::demo::demo;
use crate::api_v1::trees::trees;
use auth::auth;
use axum::Router;

/// Creates an `Axum` [`Router`] for the v1 API, combining all of its endpoint categories.
pub fn v1_api() -> Router<Config> {
    Router::new()
        .nest("/demo", demo())
        .nest("/auth", auth())
        .nest("/trees", trees())
}
