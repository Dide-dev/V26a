/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;
use crate::api_v1::session::Claims;
use crate::db::users::{add_user, get_user, update_user, user_email_exists, user_id_exists};
use crate::db_structs::users::{AccountType, RegisterData, UserGetData, UserUpdateData};
use crate::errors::ApiError;

use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_user))
        .route("/me", get(get_own_user_data))
        .route("/update/{user_id}", post(update_user_data))
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AddUserResponse {
    /// Return body signaling the user has been successfully added with the contained `user_id`.
    Success { user_id: i32 },
    /// Return body signaling the user could not be added, as the e-mail was already in use.
    EmailAlreadyUsed,
}

/// Add a user to the database.
///
/// # Returns
/// [`Success`](AddUserResponse::Success) with the id assigned to the new user
/// if the email has not yet been used, or [`EmailAlreadyUsed`](AddUserResponse::EmailAlreadyUsed) otherwise.
///
/// # Error
/// May return a [`HashException`](ApiError::HashException) instead.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_user(
    Extension(db): Extension<PgPool>,
    Json(user_data): Json<RegisterData>,
) -> Result<Json<AddUserResponse>, ApiError> {
    let mut tx = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    // Check if email already exists. This check is not strictly necessary, as a `UNIQUE`
    // constraint is placed on the `email` field.
    let exists = user_email_exists(&mut *tx, &user_data.email).await?;

    if exists {
        return Ok(Json(AddUserResponse::EmailAlreadyUsed));
    }

    let user_id = add_user(&mut tx, &user_data).await?;

    tx.commit().await.map_err(|_| ApiError::DatabaseException)?;

    Ok(Json(AddUserResponse::Success { user_id }))
}

/// Get a user's own data from the database.
///
/// # Returns
/// The user's data as [`Json`].
///
/// # Error
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_own_user_data(
    Extension(db): Extension<PgPool>,
    claims: Claims,
) -> Result<Json<UserGetData>, ApiError> {
    get_user(&db, claims.sub).await.map(Json)
}

/// Update a user's data in the database.
///
/// # Error
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the given user's data.
///
/// Returns [`MissingUser`](ApiError::MissingUser) if the given user id does not exist.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_user_data(
    Extension(db): Extension<PgPool>,
    claims: Claims,
    Path(user_id): Path<i32>,
    Json(new_data): Json<UserUpdateData>,
) -> Result<(), ApiError> {
    // A user can only update another user's data if they are an administrator.
    if claims.sub != user_id && claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    // The account type and enabled flag can only be changed by an admin.
    if (new_data.account_type.is_some() || new_data.enabled.is_some())
        && claims.account_type != AccountType::Admin
    {
        return Err(ApiError::NoPermission);
    }

    if !user_id_exists(&db, user_id).await? {
        return Err(ApiError::MissingUser(user_id));
    }

    update_user(&db, user_id, &new_data).await
}
