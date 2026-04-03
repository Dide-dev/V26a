/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::session::Claims;
use crate::db::users::{add_user, get_user, update_user};
use crate::db_structs::users::{AccountType, RegisterData, UserGetData, UserUpdateData};
use crate::errors::ApiError;
use crate::{Config, get_connection};

use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_user))
        .route("/me", get(get_own_user_data))
        .route("/update", post(update_user_data))
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
    Json(user_data): Json<RegisterData>,
) -> Result<Json<i32>, ApiError> {
    add_user(get_connection!(db), &user_data).await.map(Json)
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
    get_user(get_connection!(db), claims.sub).await.map(Json)
}

/// Update a user's data in the database.
///
/// # Error
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the given user's data.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_user_data(
    Extension(db): Extension<PgPool>,
    claims: Claims,
    Json(new_data): Json<UserUpdateData>,
) -> Result<(), ApiError> {
    if claims.sub != new_data.id && claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    // The account type can only be changed by an admin.
    if new_data.account_type.is_some() && claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    update_user(get_connection!(db), &new_data).await
}
