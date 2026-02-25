mod demo;
mod trees;
mod cards;

use crate::api_v1::demo::demo;
use crate::api_v1::trees::trees;
use crate::api_v1::cards::cards;
use axum::Router;

/// Creates an `Axum` [`Router`] for the v1 API, combining all of its endpoint categories.
pub fn v1_api() -> Router {
    Router::new()
        .nest("/demo", demo())
        .nest("/trees", trees())
        .nest("/cards", cards())
}
