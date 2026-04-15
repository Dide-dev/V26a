/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;
use crate::api_v1::session::Claims;
use crate::db::auth::get_user_credentials;
use crate::db::users::{add_user, get_user, update_password, update_user, user_id_exists};
use crate::db_structs::users::{
    AccountType, PasswordChangeRequestBody, RegisterData, UserGetData, UserUpdateData,
};
use crate::errors::ApiError;

use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_user))
        .route("/me", get(get_own_user_data))
        .route("/update/{user_id}", post(update_user_data))
        .route("/change-password", post(change_password))
}

#[utoipa::path(
    post,
    path = "/add-new-user",
    context_path = "/api/v1/users",
    request_body = RegisterData,
    responses(
        (status = 200, description = "Successfully registered user with the returned id.", body = AddUserResponse, content_type = "application/json"),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    )
)]
/// Add a user to the database.
///
/// # Arguments
/// `user_data` - The data used to register the user, as [`RegisterData`].
///
/// # Returns
/// [`Success`](AddUserResponse::Success) with the id assigned to the new user
/// if the email has not yet been used, or [`EmailAlreadyUsed`](AddUserResponse::EmailAlreadyUsed) otherwise.
///
/// # Errors
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_user(
    Extension(db): Extension<PgPool>,
    Json(user_data): Json<RegisterData>,
) -> Result<Json<AddUserResponse>, ApiError> {
    let mut transaction = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    let exists = user_email_exists(&mut *transaction, &user_data.email).await?;

    if exists {
        return Ok(Json(AddUserResponse::EmailAlreadyUsed));
    }

    let user_id = add_user(&mut transaction, &user_data).await?;

    transaction
        .commit()
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    Ok(Json(AddUserResponse::Success { user_id }))
}

#[utoipa::path(
    get,
    path = "/me",
    context_path = "/api/v1/users",
    responses(
        (status = 200, description = "Successfully retrieved own data.", body = UserGetData),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Get a user's own data from the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// # Returns
/// The user's data as [`UserGetData`].
///
/// # Errors
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_own_user_data(
    claims: Claims,
    Extension(db): Extension<PgPool>,
) -> Result<Json<UserGetData>, ApiError> {
    get_user(&db, claims.sub).await.map(Json)
}

#[utoipa::path(
    post,
    path = "/change-password",
    context_path = "/api/v1/users",
    request_body = UserUpdateData,
    responses(
        (status = 200, description = "Password successfully changes."),
        (status = 400, description = "User with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
        ("user_id" = i32, Path),
    )
)]
/// Update a user's data in the database.
///
/// A normal user can only update their own data, only an admin can change other user's data.
/// Certain fields can only be updated by admins in general, namely `account_type` and `enabled`.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `user_id` - The user for whom to update their data.
///
/// `new_data` - The new data of the user, as [`UserUpdateData`]. The `account_type` and `enabled`
/// fields are only updated if passed, and if the user has permission to do so.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingUser`](ApiError::MissingUser) if the given user id does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the given user's data, or to update one of the specified fields.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_user_data(
    claims: Claims,
    Extension(db): Extension<PgPool>,
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

#[utoipa::path(
    post,
    path = "/update/{user_id}",
    context_path = "/api/v1/users",
    request_body = PasswordChangeRequestBody,
    responses(
        (status = 200, description = "User data successfully updated."),
        (status = 400, description = "User with given id cannot be found."),
        (status = 400, description = ApiError::WrongCredentials.to_string()),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status= 500, description = ApiError::HashException.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Changes a password in the database.
///
/// # Arguments
/// `old_password` - The user's old password.
///
/// `new_password` - The new password.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`WrongCredentials`](ApiError::WrongCredentials) if the given password does not match the correct password.
///
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn change_password(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(password_change_request): Json<PasswordChangeRequestBody>,
) -> Result<(), ApiError> {
    let credentials = get_user_credentials(&db, password_change_request.email).await?;
    let password_valid = bcrypt::verify(
        password_change_request.old_password,
        credentials.password_hash.as_str(),
    )
    .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Err(ApiError::WrongCredentials);
    }

    update_password(
        get_connection!(db),
        claims.sub,
        password_change_request.new_password,
    ).await?;

    Ok(())
}
