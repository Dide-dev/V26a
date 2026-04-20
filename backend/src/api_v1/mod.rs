/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

pub mod admin;
pub mod auth;
pub mod cards;
pub mod session;
pub mod trees;
pub mod users;

use crate::Config;
use crate::api_v1::{admin::admin, cards::cards, trees::trees, users::users};
use auth::auth;
use axum::Router;

/// Creates an `Axum` [`Router`] for the v1 API, combining all of its endpoint categories.
pub fn v1_api() -> Router<Config> {
    Router::new()
        .nest("/admin", admin())
        .nest("/auth", auth())
        .nest("/trees", trees())
        .nest("/cards", cards())
        .nest("/users", users())
}
