/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;
use crate::db::users::{add_user, user_exists};
use crate::db_structs::users::UserData;

use crate::errors::ApiError;
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::Serialize;
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<Config> {
    Router::new().route("/add", post(add_new_user))
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AddUserResponse {
    Success { user_id: i32 },
    EmailAlreadyUsed,
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
) -> Result<Json<AddUserResponse>, ApiError> {
    let mut tx = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    // Check if email already exists. This check is not strictly necessary, as a `UNIQUE`
    // constraint is placed on the `email` field.
    let exists = user_exists(&mut *tx, &user_data.email).await?;

    if exists {
        return Ok(Json(AddUserResponse::EmailAlreadyUsed));
    }

    let user_id = add_user(&mut tx, &user_data).await?;

    tx.commit().await.map_err(|_| ApiError::DatabaseException)?;

    Ok(Json(AddUserResponse::Success { user_id }))
}
