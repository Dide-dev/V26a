/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

pub mod auth;
pub mod cards;
pub mod demo;
pub mod session;
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
