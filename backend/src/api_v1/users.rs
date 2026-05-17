/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::api_v1::session::Claims;
use crate::db::auth::get_user_credentials_by_id;
use crate::db::users::{
    add_user, delete_user, get_user, get_user_by_email, get_user_by_name, update_password,
    update_user, user_id_exists,
};
use crate::db_structs::users::{
    AccountType, DeleteOwnedTrees, PasswordChangeRequestBody, PendingRegisterData, RegisterData,
    UserGetData, UserUpdateData,
};
use crate::errors::ApiError;

use crate::api_v1::users::UserResponse::InvalidVerification;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::{Duration, Utc};
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::{Address, Message};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<ArcState> {
    Router::new()
        .route("/add", post(add_new_user))
        .route("/verify", get(verify))
        .route("/me", get(get_own_user_data))
        .route("/update/{user_id}", post(update_user_data))
        .route("/change-password", post(change_password))
        .route("/{user_id}/delete", delete(delete_user_data))
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum UserResponse {
    /// Return body signalling the user has been successfully added/updated with the contained `user_id`.
    Success { user_id: i32 },
    /// Return body signalling the user data has been stored, but is pending verification.
    Pending,
    /// Return body signalling that the user tried to verify with a code that is invalid or expired.
    InvalidVerification,
    /// Return body signalling the user could not be added/updated, as the e-mail was already in use.
    EmailAlreadyUsed,
    /// Return body signalling the user could not be added/updated, as the username was already in use.
    UsernameAlreadyUsed,
}

#[utoipa::path(
    post,
    path = "/add",
    context_path = "/api/v1/users",
    request_body = RegisterData,
    responses(
        (status = 200, description = "Awaiting verification, email already used, or username already used."),
        (status = 400, description = ApiError::EmailParseError.to_string()),
        (status = 500, description = "Internal server error."),
    )
)]
/// Register a user, after which the account needs to be verified.
///
/// # Arguments
/// `user_data` - The data used to register the user, as [`RegisterData`].
///
/// # Returns
/// [`Pending`](UserResponse::Pending) if the registration was successful and is now pending verification.
/// if the email has not yet been used, or [`EmailAlreadyUsed`](UserResponse::EmailAlreadyUsed) otherwise.
/// if the username has not yet been used, or [`UsernameAlreadyUsed`](UserResponse::UsernameAlreadyUsed) otherwise.
///
/// # Errors
/// May return a [`HashException`](ApiError::HashException),
/// [`DatabaseException`](ApiError::DatabaseException),
/// [`EmailParseError`](ApiError::EmailParseError),
/// [`SmtpError`](ApiError::SmtpError) or
/// [`LettreError`](ApiError::LettreError) instead.
async fn add_new_user(
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(user_data): Json<RegisterData>,
) -> Result<Json<UserResponse>, ApiError> {
    let parsed_email: Address = user_data.email.parse()?;

    let verification_code = {
        let mut rng = rand::rng();
        Alphanumeric.sample_string(&mut rng, 64)
    };

    {
        let mut pending_accounts = state.pending_accounts.lock().await;

        let found_user_id = get_user_by_email(&db, &user_data.email).await?;
        let found_pending_email = pending_accounts
            .values()
            .any(|r| r.register_data.email.to_lowercase() == user_data.email.to_lowercase());

        if found_user_id.is_some() || found_pending_email {
            return Ok(Json(UserResponse::EmailAlreadyUsed));
        }

        let found_user_id = get_user_by_name(&db, &user_data.name).await?;
        let found_pending_username = pending_accounts
            .values()
            .any(|r| r.register_data.name.to_lowercase() == user_data.name.to_lowercase());

        if found_user_id.is_some() || found_pending_username {
            return Ok(Json(UserResponse::UsernameAlreadyUsed));
        }

        let expires = Utc::now() + Duration::seconds(state.config.verification_timer);
        let pending_data = PendingRegisterData {
            register_data: user_data.clone(),
            expires: expires.timestamp(),
        };
        pending_accounts.insert(verification_code.clone(), pending_data);

        // Release the lock, then send an email
    }

    let verification_link = format!(
        "{}/api/v1/users/verify?code={}",
        state.config.base_url, verification_code
    );
    let message = Message::builder()
        .from("BOOM! <noreply@localhost>".parse()?)
        .to(Mailbox::new(Some(user_data.nickname.clone()), parsed_email))
        .subject(user_data.language.to_verification_message_title())
        .header(ContentType::TEXT_PLAIN)
        .body(user_data.language.to_verification_message_body(
            &user_data.nickname,
            &verification_link,
            &state.config.base_url,
        ))?;
    state.config.mailer.send(message).await?;

    Ok(Json(UserResponse::Pending))
}

#[derive(serde::Deserialize, ToSchema)]
pub struct VerificationCode {
    code: String,
}

#[utoipa::path(
    get,
    path = "/verify",
    context_path = "/api/v1/users",
    request_body = RegisterData,
    responses(
        (status = 200, description = "Successfully verified user, or verification expired.", body = UserResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("code" = VerificationCode, Query),
    )
)]
/// Verify a user who is pending registration
///
/// # Arguments
/// `code` - The verification code, given as a [`VerificationCode`] through a query.
///
/// # Returns
/// [`Success`](UserResponse::Success) with the id assigned to the new user.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn verify(
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Query(code): Query<VerificationCode>,
) -> Result<Json<UserResponse>, ApiError> {
    let code = code.code;
    let mut pending_accounts = state.pending_accounts.lock().await;

    let Some(pending_registration) = pending_accounts
        .get(&code)
        // If the registration has expired, consider it not found.
        .filter(|p| Utc::now().timestamp() < p.expires)
    else {
        return Ok(Json(InvalidVerification));
    };

    let mut tx = db.begin().await?;
    let user_id = add_user(
        &mut tx,
        &pending_registration.register_data,
        state.config.bcrypt_cost,
    )
    .await?;

    if pending_accounts.remove(&code).is_none() {
        // Somehow, the program failed to find the verification code in state.
        // Do not commit the transaction and fail.
        tx.rollback().await?;
        Ok(Json(InvalidVerification))
    } else {
        tx.commit().await?;
        Ok(Json(UserResponse::Success { user_id }))
    }
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
    path = "/update/{user_id}",
    context_path = "/api/v1/users",
    request_body = UserUpdateData,
    responses(
        (status = 200, description = "Successfully updated user with the returned id.", body = UserResponse, content_type = "application/json"),
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
    State(state): State<ArcState>,
    Path(user_id): Path<i32>,
    Json(new_data): Json<UserUpdateData>,
) -> Result<Json<UserResponse>, ApiError> {
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

    // This lock should not be released until the user is updated.
    let pending_accounts = state.pending_accounts.lock().await;

    let found_user_id = get_user_by_name(&db, &new_data.name).await?;
    let found_pending_username = pending_accounts
        .values()
        .any(|r| r.register_data.name.to_lowercase() == new_data.name.to_lowercase());

    if found_user_id.is_some_and(|id| id != claims.sub) || found_pending_username {
        return Ok(Json(UserResponse::UsernameAlreadyUsed));
    }

    update_user(&db, user_id, &new_data).await?;

    Ok(Json(UserResponse::Success { user_id }))
}

/// Return body, where the `succeeded`-field signals whether the request was successful.
/// - If the flag is `false`, the wrong credentials were passed.
/// - If the flag is `true`, the right credentials were passed and the password change was successful.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordResponse {
    pub succeeded: bool,
}

#[utoipa::path(
    post,
    path = "/change-password",
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
/// Changes a password in the database.
///
/// # Arguments
/// `password_change_request` - A struct containing the user's old password and the new password.
///
/// # Returns
/// Nothing.
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
    path = "/{user_id}/delete",
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
/// `jar` - The jar with the cookies (`user_info` & session).
///
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
