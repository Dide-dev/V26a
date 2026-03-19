pub mod auth;
pub mod cards;
pub mod demo;
pub mod session;
pub mod structs;
pub mod trees;
pub mod users;

use crate::Config;
use crate::api_v1::{cards::cards, demo::demo, trees::trees, users::users};
use auth::auth;
use axum::Router;

/// Creates an `Axum` [`Router`] for the v1 API, combining all of its endpoint categories.
pub fn v1_api() -> Router<Config> {
    Router::new()
        .nest("/auth", auth())
        .nest("/demo", demo())
        .nest("/trees", trees())
        .nest("/cards", cards())
        .nest("/users", users())
}
