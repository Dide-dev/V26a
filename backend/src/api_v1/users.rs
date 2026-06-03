/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::api_v1::rate_limit_layer;
use crate::db::auth::get_user_credentials_by_id;
use crate::db::users::{
    add_user, delete_user, get_user, get_user_by_email, get_user_by_name, update_password,
    update_user, user_id_exists,
};
use crate::db_structs::session::{Claims, InfoCookieData};
use crate::db_structs::users::{
    AccountType, DeleteOwnedTrees, PasswordChangeRequestBody, RegisterData, UserData, UserGetData,
};
use crate::errors::ApiError;

use crate::db_structs::Validated;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Duration;
use utoipa::ToSchema;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<ArcState> {
    Router::new()
        .route("/", post(add_new_user).layer(rate_limit_layer(5, 20)))
        .route("/me", get(get_own_user_data).put(update_user_data))
        .route("/me/password", put(change_password))
        .route("/{user_id}", delete(delete_user_data))
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum UserResponse {
    /// Return body signaling the user has been successfully added/updated with the contained `user_id`.
    Success { user_id: i32 },
    /// Return body signaling the user could not be added/updated, as the e-mail was already in use.
    EmailAlreadyUsed,
    /// Return body signaling the user could not be added/updated, as the username was already in use.
    UsernameAlreadyUsed,
}

#[utoipa::path(
    post,
    path = "",
    context_path = "/api/v1/users",
    request_body = RegisterData,
    responses(
        (status = 200, description = "Successfully registered user with the returned id.", body = UserResponse, content_type = "application/json"),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 429, description = "Ratelimit exceeded."),
        (status = 500, description = "Internal server error."),
    )
)]
/// Add a user to the database.
///
/// # Arguments
/// `user_data` - The data used to register the user, as [`RegisterData`].
///
/// # Returns
/// [`Success`](UserResponse::Success) with the id assigned to the new user
/// if the email has not yet been used, or [`EmailAlreadyUsed`](UserResponse::EmailAlreadyUsed) otherwise.
/// if the username has not yet been used, or [`UsernameAlreadyUsed`](UserResponse::UsernameAlreadyUsed) otherwise.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_user(
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(user_data): Json<RegisterData>,
) -> Result<Json<UserResponse>, ApiError> {
    user_data.validate_inputs()?;

    let mut transaction = db.begin().await?;

    let found_user_id = get_user_by_email(&mut *transaction, &user_data.email).await?;

    if found_user_id.is_some() {
        return Ok(Json(UserResponse::EmailAlreadyUsed));
    }

    let found_user_id = get_user_by_name(&mut *transaction, &user_data.data.name).await?;

    if found_user_id.is_some() {
        return Ok(Json(UserResponse::UsernameAlreadyUsed));
    }

    let user_id = add_user(&mut transaction, &user_data, state.config.bcrypt_cost).await?;

    transaction.commit().await?;

    Ok(Json(UserResponse::Success { user_id }))
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
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
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
    put,
    path = "/me",
    context_path = "/api/v1/users",
    request_body = UserData,
    responses(
        (status = 200, description = "Successfully updated user with the returned id.", body = UserResponse, content_type = "application/json"),
        (status = 400, description = "User with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Update a user's own data in the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `new_data` - The new data of the user, as [`UserData`].
///
/// # Returns
/// [`Success`](UserResponse::Success) with the id assigned to the new user, if successful,
/// or [`EmailAlreadyUsed`](UserResponse::EmailAlreadyUsed) if the e-mail is already in use,
/// or [`UsernameAlreadyUsed`](UserResponse::UsernameAlreadyUsed) if the username is already in use.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_user_data(
    mut jar: CookieJar,
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(new_data): Json<UserData>,
) -> Result<(CookieJar, Json<UserResponse>), ApiError> {
    new_data.validate_inputs()?;
    let found_user_id = get_user_by_name(&db, &new_data.name).await?;

    if let Some(id) = found_user_id
        && id != claims.sub
    {
        return Ok((jar, Json(UserResponse::UsernameAlreadyUsed)));
    }

    update_user(&db, claims.sub, &new_data).await?;

    let new_user_info_cookie = InfoCookieData {
        name: new_data.name,
        nickname: new_data.nickname,
        sub: claims.sub,
        account_type: claims.account_type,
    };
    let new_user_info = serde_json::to_string(&new_user_info_cookie)?;
    let time = Duration::seconds(claims.exp - Utc::now().timestamp());
    let user_cookie = Cookie::build(("user_info", new_user_info))
        .path("/")
        .secure(true)
        .max_age(time)
        .http_only(false);
    // `add` automatically replaces the cookie values, `remove` sets the cookie to an empty value and the maxAge to 0.
    jar = jar.add(user_cookie);

    Ok((
        jar,
        Json(UserResponse::Success {
            user_id: claims.sub,
        }),
    ))
}

/// Return body, where the `succeeded`-field signals whether the request was successful.
/// - If the flag is `false`, the wrong credentials were passed.
/// - If the flag is `true`, the right credentials were passed and the password change was successful.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordResponse {
    pub succeeded: bool,
}

#[utoipa::path(
    put,
    path = "/me/password",
    context_path = "/api/v1/users",
    request_body = PasswordChangeRequestBody,
    responses(
        (status = 200, description = "Password changing attempt has been finalized.", body = ChangePasswordResponse),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Changes a user's own password in the database.
///
/// # Arguments
/// `password_change_request` - The struct containing the user's old password and the new password,
/// as [`PasswordChangeRequestBody`].
///
/// # Returns
/// A boolean whether the password change was successful, as [`ChangePasswordResponse`].
///
/// # Errors
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn change_password(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(password_change_request): Json<PasswordChangeRequestBody>,
) -> Result<Json<ChangePasswordResponse>, ApiError> {
    password_change_request.validate_inputs()?;

    let credentials = get_user_credentials_by_id(&db, claims.sub)
        .await?
        .ok_or(ApiError::DatabaseException)?;

    let password_valid = bcrypt::verify(
        password_change_request.old_password,
        &credentials.password_hash,
    )
    .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Ok(Json(ChangePasswordResponse { succeeded: false }));
    }

    update_password(
        &db,
        claims.sub,
        &password_change_request.new_password,
        state.config.bcrypt_cost,
    )
    .await?;

    Ok(Json(ChangePasswordResponse { succeeded: true }))
}

#[utoipa::path(
    delete,
    path = "/{user_id}",
    context_path = "/api/v1/users",
    responses(
        (status = 200, description = "User data successfully deleted."),
        (status = 400, description = "User with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("user_id" = i32, Path),
        ("deleteOwnedTrees" = DeleteOwnedTrees, Query),
        ("session" = Claims, Cookie),
    )
)]
/// Delete a user from the database.
///
/// A user can only delete their own account, unless the user is an admin (permission to delete other accounts).
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `user_id` - The user to delete.
///
/// `deleteOwnedTrees` - Delete all the trees of the user.
///
/// # Returns
/// The (cookie) jar that is empty.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// Returns [`MissingUser`](ApiError::MissingUser) if the given user id does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to delete
/// the user.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn delete_user_data(
    mut jar: CookieJar,
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(user_id): Path<i32>,
    Query(delete): Query<DeleteOwnedTrees>,
) -> Result<CookieJar, ApiError> {
    let mut transaction = db.begin().await?;

    if !user_id_exists(&mut *transaction, user_id).await? {
        return Err(ApiError::MissingUser(user_id));
    }

    // A user can only delete their own account unless they are an admin.
    if claims.sub != user_id && claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    delete_user(&mut transaction, user_id, delete.delete_owned_trees).await?;

    if claims.sub == user_id {
        jar = jar
            .remove(Cookie::from("session"))
            .remove(Cookie::from("user_info"));
    }

    transaction.commit().await?;

    Ok(jar)
}
