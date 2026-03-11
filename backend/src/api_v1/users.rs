use crate::Config;
use crate::db::users::add_user;
use crate::db_structs::users::UserData;

use crate::errors::ApiError;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<Config> {
    Router::new().route("/add-new-user", post(add_new_user))
}

/// Add a user to the database.
///
/// # Returns
/// The id assigned to the new user as [`Json`].
///
/// # Error
/// Returns an [`ApiError::EmailAlreadyUsed`] if the email was already used in the database.
///
/// May return an [`ApiError::DatabaseException`] instead.
async fn add_new_user(
    db: Extension<PgPool>,
    Json(user_data): Json<UserData>,
) -> Result<Json<i32>, ApiError> {
    add_user(&db, user_data).await.map(Json)
}
