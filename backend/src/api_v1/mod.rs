mod demo;

use crate::api_v1::demo::demo;
use axum::Router;

/// Creates an `Axum` [`Router`] for the v1 API, combining all of its endpoint categories.
pub fn v1_api() -> Router {
    Router::new().nest("/demo", demo())
}
