use crate::Config;
use crate::db::users::add_user;
use crate::db_structs::users::UserData;

use crate::db::get_connection;
use crate::errors::ApiError;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<Config> {
    Router::new().route("/add", post(add_new_user))
}

/// Add a user to the database.
///
/// # Returns
/// The id assigned to the new user as [`Json`].
///
/// # Error
/// Returns [`EmailAlreadyUsed`](ApiError::EmailAlreadyUsed) if the email was already used in the database.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_user(
    Extension(db): Extension<PgPool>,
    Json(user_data): Json<UserData>,
) -> Result<Json<i32>, ApiError> {
    add_user(&mut *get_connection(db).await?, user_data)
        .await
        .map(Json)
}
