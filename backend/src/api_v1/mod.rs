/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

pub mod admin;
pub mod auth;
pub mod cards;
pub mod cards_library;
pub mod classrooms;
pub mod feedback;
pub mod images;
pub mod lessons;
pub mod session;
pub mod trees;
pub mod users;

use crate::ArcState;
use crate::api_v1::{
    admin::admin, auth::auth, cards::cards, cards_library::cards_library, classrooms::classrooms,
    feedback::feedback, images::images, trees::trees, users::users,
};
use axum::Router;
use chrono::{DateTime, Utc};
use lessons::lessons;
use rand::distr::{Alphanumeric, SampleString};

/// Creates an `Axum` [`Router`] for the v1 API, combining all of its endpoint categories.
pub fn v1_api(state: ArcState) -> Router<ArcState> {
    Router::new()
        .nest("/admin", admin(state))
        .nest("/auth", auth())
        .nest("/cards", cards())
        .nest("/classrooms", classrooms())
        .nest("/feedback", feedback())
        .nest("/images", images())
        .nest("/lessons", lessons())
        .nest("/library", cards_library())
        .nest("/trees", trees())
        .nest("/users", users())
}

/// Helper function that creates a token.
///
/// # Returns
///
/// Returns a [`String`] containing the token.
///
/// Returns a [`DateTime<Utc>`] containing the expiration time.
pub fn create_token(session_duration: i64) -> (String, DateTime<Utc>) {
    let token = {
        let mut rng = rand::rng();
        Alphanumeric.sample_string(&mut rng, 64)
    };
    let expires_at = Utc::now() + chrono::Duration::seconds(session_duration);
    (token, expires_at)
}
