use crate::db::users::add_user;
use crate::errors::ApiError;
use crate::users::UserData;

use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router {
    Router::new().route("/add-new-user", post(add_new_user))
}

/// Add a user to the database.
///
/// # Returns
/// Returns the index assigned to the new user.
///
/// # Error
/// Returns an [`ApiError::EmailAlreadyUsed`] if the email was already used in the database
async fn add_new_user(
    db: Extension<PgPool>,
    Json(user_data): Json<UserData>,
) -> Result<Json<i32>, ApiError> {
    add_user(&db, user_data).await.map(Json)
}
