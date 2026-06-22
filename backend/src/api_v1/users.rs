/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::users::UserResponse::InvalidVerification;
use crate::db::auth::get_user_credentials_by_id;
use crate::db::users::{
    add_user, delete_user, get_user, get_user_by_email, get_user_by_name, get_user_public_data,
    update_password, update_user, user_id_exists,
};
use crate::errors::ApiError;
use crate::messages::Message::{
    EmailVerificationBody, EmailVerificationSubject, PasswordResetMailBody,
    PasswordResetMailSubject,
};
use crate::ratelimits::{ADD_USER_RATELIMIT, RESET_PASSWORD_RATELIMIT, VERIFY_USER_RATELIMIT};
use crate::structs::session::{InfoCookieData, Session, SessionType};
use crate::structs::users::{
    DeleteOwnedTrees, PasswordChangeRequestBody, PasswordResetBody, PasswordResetRequestData,
    PendingRegisterData, PublicUserData, RegisterData, RemoveExpired, RequestPasswordResetBody,
    UserData, UserGetData,
};
use crate::structs::{Validated, parse_email};
use crate::{ArcState, get_connection};
use axum::extract::{Path, State};
use axum::routing::{get, post, put};
use axum::{Extension, Json, Router};
use axum_extra::extract::{CookieJar, Query, cookie::Cookie};
use chrono::Utc;
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::{Address, Message};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use sqlx::{PgExecutor, PgPool, query};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use time::Duration;
use utoipa::ToSchema;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<ArcState> {
    Router::new()
        .route("/", post(add_new_user).layer(ADD_USER_RATELIMIT.to_layer()))
        .route(
            "/verification",
            post(verify).layer(VERIFY_USER_RATELIMIT.to_layer()),
        )
        .route("/{user_id}/public", get(get_public_user_data))
        .route(
            "/me",
            get(get_own_user_data)
                .put(update_user_data)
                .delete(delete_user_data),
        )
        .route("/me/password", put(change_password))
        .route(
            "/reset-password-requests",
            post(request_password_reset)
                .put(reset_password)
                .layer(RESET_PASSWORD_RATELIMIT.to_layer()),
        )
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all_fields = "camelCase")]
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
    path = "",
    context_path = "/api/v1/users",
    request_body = RegisterData,
    responses(
        (status = 200, description = "Awaiting verification, email already used, or username already used."),
        (status = 400, description = ApiError::EmailParseError.to_string()),
        (status = 429, description = "Ratelimit exceeded."),
        (status = 500, description = "Internal server error."),
    )
)]
/// Register a user, after which the account needs to be verified.
///
/// # Arguments
/// `user_data` - The data used to register the user, as [`RegisterData`].
///
/// # Returns
/// [`Pending`](UserResponse::Pending) if the registration was successful and is now pending verification,
///
/// [`EmailAlreadyUsed`](UserResponse::EmailAlreadyUsed) if the email has already been used, or
///
/// [`UsernameAlreadyUsed`](UserResponse::UsernameAlreadyUsed) if the username has already been used.
///
/// # Errors
/// May return a [`HashException`](ApiError::HashException),
/// [`DatabaseException`](ApiError::DatabaseException),
/// [`EmailParseError`](ApiError::EmailParseError),
/// [`SmtpError`](ApiError::SmtpError) or
/// [`LettreException`](ApiError::LettreException) instead.
async fn add_new_user(
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(user_data): Json<RegisterData>,
) -> Result<Json<UserResponse>, ApiError> {
    user_data.validate_inputs()?;
    let parsed_email: Address = parse_email(&user_data.email)?;

    let verification_code = {
        let mut rng = rand::rng();
        Alphanumeric.sample_string(&mut rng, 64)
    };

    {
        let mut pending_accounts = state.pending_accounts.lock().await;

        // Remove pending accounts in case they have not been pruned yet.
        pending_accounts.remove_expired();

        if check_email_used(&db, &pending_accounts, &user_data.email, None).await? {
            return Ok(Json(UserResponse::EmailAlreadyUsed));
        }

        if check_username_used(&db, &pending_accounts, &user_data.data.name, None).await? {
            return Ok(Json(UserResponse::UsernameAlreadyUsed));
        }

        let expires = Utc::now().timestamp() + state.config.verification_timer;
        let pending_data = PendingRegisterData {
            register_data: user_data.clone(),
            expires,
        };
        pending_accounts.insert(verification_code.clone(), pending_data);

        // Release the lock, then send an email
    }

    let verification_link = format!(
        "{}/verify?code={}",
        state.config.base_url, verification_code
    );
    let display_name = user_data.data.get_display_name();
    let language = user_data.data.language;
    let message = Message::builder()
        .from(Mailbox::new(
            Some("BOOM!".to_owned()),
            format!("noreply@{}", state.config.hostname).parse()?,
        ))
        .to(Mailbox::new(Some(display_name.clone()), parsed_email))
        .header(ContentType::TEXT_PLAIN)
        .subject(EmailVerificationSubject.localized(&language))
        .body(
            EmailVerificationBody {
                display_name,
                verification_link,
                duration_seconds: state.config.verification_timer,
                base_url: state.config.base_url.clone(),
            }
            .localized(&language),
        )?;
    state.config.mailer.send(message).await?;

    Ok(Json(UserResponse::Pending))
}

/// Struct for a verification code for the /users/verify endpoint.
#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
pub struct VerificationCode {
    pub code: String,
}

#[utoipa::path(
    post,
    path = "/verify",
    context_path = "/api/v1/users",
    request_body = VerificationCode,
    responses(
        (status = 200, description = "Successfully verified user, or verification expired.", body = UserResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error."),
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
    Json(VerificationCode { code }): Json<VerificationCode>,
) -> Result<Json<UserResponse>, ApiError> {
    let mut pending_accounts = state.pending_accounts.lock().await;

    let entry = match pending_accounts.entry(code) {
        Entry::Occupied(occupied_entry) => {
            if occupied_entry.get().expires < Utc::now().timestamp() {
                return Ok(Json(InvalidVerification));
            }
            occupied_entry
        }
        Entry::Vacant(_) => {
            return Ok(Json(InvalidVerification));
        }
    };

    let user_id = add_user(
        get_connection!(db),
        &entry.get().register_data,
        state.config.bcrypt_cost,
    )
    .await?;

    entry.remove();

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
        ("session" = Session, Cookie),
    )
)]
/// Get a user's own data from the database.
///
/// # Arguments
/// `session` - The session cookie of the user.
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
    session: Session,
    Extension(db): Extension<PgPool>,
) -> Result<Json<UserGetData>, ApiError> {
    get_user(&db, session.user_id).await.map(Json)
}

#[utoipa::path(
    get,
    path = "/{user_id}/public",
    context_path = "/api/v1/users",
    responses(
        (status = 200, description = "Successfully retrieved public data of user.", body = PublicUserData),
        (status = 400, description = "User with given id cannot be found."),
        (status = 500, description = "Internal server error."),
    )
)]
/// Get a user's public data from the database.
///
/// # Returns
/// The user's data as [`PublicUserData`].
///
/// # Errors
/// Returns [`MissingUser`](ApiError::MissingUser) if no user of the given id could be found.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_public_user_data(
    Path(user_id): Path<i32>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<PublicUserData>, ApiError> {
    if let Some(user_data) = get_user_public_data(&db, user_id).await? {
        Ok(Json(user_data))
    } else {
        Err(ApiError::MissingUser(user_id))
    }
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
        ("session" = Session, Cookie),
    )
)]
/// Update a user's own data in the database.
///
/// # Arguments
/// `session` - The session cookie of the user.
///
/// `new_data` - The new data of the user, as [`UserData`].
///
/// # Returns
/// [`Success`](UserResponse::Success) with the id assigned to the new user, if successful,
/// or [`UsernameAlreadyUsed`](UserResponse::UsernameAlreadyUsed) if the username is already in use.
/// Also updates the cookies.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_user_data(
    jar: CookieJar,
    session: Session,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(new_data): Json<UserData>,
) -> Result<(CookieJar, Json<UserResponse>), ApiError> {
    new_data.validate_inputs()?;

    // This lock should not be released until the user is updated.
    let mut pending_accounts = state.pending_accounts.lock().await;

    // Remove pending accounts in case they have not been pruned yet.
    pending_accounts.remove_expired();

    if check_username_used(
        &db,
        &pending_accounts,
        &new_data.name,
        Some(session.user_id),
    )
    .await?
    {
        return Ok((jar, Json(UserResponse::UsernameAlreadyUsed)));
    }

    update_user(&db, session.user_id, &new_data).await?;

    let new_user_info_cookie = InfoCookieData {
        name: new_data.name,
        nickname: new_data.nickname,
        user_id: session.user_id,
        account_type: session.account_type,
    };
    let new_user_info = serde_json::to_string(&new_user_info_cookie)?;

    let expires_at = query!(
        "SELECT expires_at FROM sessions WHERE token = $1",
        Session::extract_token(&jar)?
    )
    .fetch_one(&db)
    .await?
    .expires_at;

    let user_cookie = Cookie::build(("user_info", new_user_info))
        .path("/")
        .secure(true)
        .max_age(Duration::seconds((expires_at - Utc::now()).num_seconds()))
        .http_only(false);
    // `add` automatically replaces the cookie values, `remove` sets the cookie to an empty value and the maxAge to 0.
    let jar = jar.add(user_cookie);

    Ok((
        jar,
        Json(UserResponse::Success {
            user_id: session.user_id,
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
        ("session" = Session, Cookie),
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
/// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
///
/// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
///
/// Returns [`IllegalPassword`](ApiError::IllegalPassword) if the new password does not meet the minimum security requirements.
///
/// May return a [`HashException`](ApiError::HashException), [`InvalidDataFound`](ApiError::InvalidDataFound) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn change_password(
    session: Session,
    jar: CookieJar,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(password_change_request): Json<PasswordChangeRequestBody>,
) -> Result<Json<ChangePasswordResponse>, ApiError> {
    password_change_request.validate_inputs()?;

    let credentials = get_user_credentials_by_id(&db, session.user_id)
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
        session.user_id,
        &password_change_request.new_password,
        state.config.bcrypt_cost,
    )
    .await?;

    let token = Session::extract_token(&jar)?;

    // All other sessions of the account are invalidated if password is changed.
    Session::delete_all_sessions_except(&db, &token, &session.user_id).await?;

    Ok(Json(ChangePasswordResponse { succeeded: true }))
}

#[utoipa::path(
    post,
    path = "/reset-password-requests",
    context_path = "/api/v1/users",
    request_body = RequestPasswordResetBody,
    responses(
        (status = 200, description = "Password change request fulfilled."),
        (status = 400, description = ApiError::EmailParseError.to_string()),
        (status = 429, description = "Ratelimit exceeded."),
        (status = 500, description = "Internal server error."),
    )
)]
/// Sends a change password e-mail to the specified e-mail, if an account with that email exists.
///
/// # Arguments
/// `password_change_request` - The struct containing the e-mail address to which a change password
/// e-mail should be sent as [`RequestPasswordResetBody`].
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException),
/// [`EmailParseError`](ApiError::EmailParseError),
/// [`SmtpError`](ApiError::SmtpError) or
/// [`LettreException`](ApiError::LettreException) instead.
async fn request_password_reset(
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(password_reset_request): Json<RequestPasswordResetBody>,
) -> Result<(), ApiError> {
    password_reset_request.validate_inputs()?;
    let email = password_reset_request.email;
    let parsed_email = parse_email(&email)?;

    let Some(user_id) = get_user_by_email(&db, &email).await? else {
        // Account with e-mail doesn't exist, do nothing.
        return Ok(());
    };
    let user_data = get_user(&db, user_id).await?.data;

    let password_reset_code = {
        let mut rng = rand::rng();
        Alphanumeric.sample_string(&mut rng, 64)
    };

    {
        let mut reset_password_requests = state.reset_password_requests.lock().await;

        // Exit if a password reset has already been requested.
        if reset_password_requests
            .values()
            .any(|r| r.user_id == user_id && Utc::now().timestamp() < r.expires)
        {
            return Ok(());
        }

        let expires = Utc::now().timestamp() + state.config.reset_password_timer;
        let password_reset_request_data = PasswordResetRequestData { user_id, expires };
        reset_password_requests.insert(password_reset_code.clone(), password_reset_request_data);

        // Release the lock, then send an email
    }

    let password_reset_link = format!(
        "{}/reset-password?code={}",
        state.config.base_url, password_reset_code
    );
    let display_name = user_data.get_display_name();
    let language = user_data.language;
    let message = Message::builder()
        .from(Mailbox::new(
            Some("BOOM!".to_owned()),
            format!("noreply@{}", state.config.hostname).parse()?,
        ))
        .to(Mailbox::new(Some(display_name.clone()), parsed_email))
        .header(ContentType::TEXT_PLAIN)
        .subject(PasswordResetMailSubject.localized(&language))
        .body(
            PasswordResetMailBody {
                display_name,
                password_reset_link,
                duration_seconds: state.config.reset_password_timer,
                base_url: state.config.base_url.clone(),
            }
            .localized(&language),
        )?;
    state.config.mailer.send(message).await?;

    Ok(())
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all_fields = "camelCase")]
pub enum PasswordResetResponse {
    /// Return body signalling the password has been successfully updated..
    Success,
    /// Return body signalling that the user tried to reset a password with a code that is invalid or expired.
    InvalidCode,
}

#[utoipa::path(
    put,
    path = "/reset-password-requests",
    context_path = "/api/v1/users",
    request_body = PasswordResetBody,
    responses(
        (status = 200, description = "Password changing attempt has been finalized.", body = PasswordResetResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error.")
    )
)]
/// Reset a user's password, with a password-reset code tied to a user and the new password.
///
/// # Arguments
/// `password_reset_body` - The struct containing the password-reset code and the new password,
/// as [`PasswordResetMailBody`].
///
/// # Returns
/// [`Success`](PasswordResetResponse::Success) if successful,
/// or [`InvalidCode`](PasswordResetResponse::InvalidCode) if the code is invalid or expired.
///
/// # Errors
/// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
///
/// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
///
/// Returns [`IllegalPassword`](ApiError::IllegalPassword) if the new password does not meet the minimum security requirements.
///
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn reset_password(
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(password_reset_body): Json<PasswordResetBody>,
) -> Result<Json<PasswordResetResponse>, ApiError> {
    password_reset_body.validate_inputs()?;
    let new_password = password_reset_body.new_password;

    let mut reset_password_requests = state.reset_password_requests.lock().await;

    let entry = match reset_password_requests.entry(password_reset_body.code) {
        Entry::Occupied(occupied_entry) => {
            if occupied_entry.get().expires < Utc::now().timestamp() {
                return Ok(Json(PasswordResetResponse::InvalidCode));
            }
            occupied_entry
        }
        Entry::Vacant(_) => {
            return Ok(Json(PasswordResetResponse::InvalidCode));
        }
    };

    let user_id = entry.get().user_id;

    update_password(&db, user_id, &new_password, state.config.bcrypt_cost).await?;

    entry.remove();

    // All other sessions of the account are invalidated if password is changed.
    Session::delete_all_sessions(&db, &user_id).await?;

    Ok(Json(PasswordResetResponse::Success))
}

#[utoipa::path(
    delete,
    path = "/me",
    context_path = "/api/v1/users",
    responses(
        (status = 200, description = "User data successfully deleted."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("deleteOwnedTrees" = DeleteOwnedTrees, Query),
        ("session" = Session, Cookie),
    )
)]
/// Delete the logged-in user from the database.
///
/// # Arguments
/// `session` - The session cookie of the user.
///
/// `deleteOwnedTrees` - Delete all the trees of the user.
///
/// # Returns
/// The cookie jar with the session cookies removed.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// Returns [`MissingUser`](ApiError::MissingUser) if the given user id does not exist.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn delete_user_data(
    jar: CookieJar,
    session: Session,
    Extension(db): Extension<PgPool>,
    Query(delete): Query<DeleteOwnedTrees>,
) -> Result<CookieJar, ApiError> {
    let mut transaction = db.begin().await?;

    if !user_id_exists(&mut *transaction, session.user_id).await? {
        return Err(ApiError::MissingUser(session.user_id));
    }

    delete_user(&mut transaction, session.user_id, delete.delete_owned_trees).await?;

    // Invalidate all the sessions based on userid.
    Session::delete_all_sessions(&mut *transaction, &session.user_id).await?;

    transaction.commit().await?;

    // Unset the cookies by changing the date to a time in the past.
    Ok(jar
        .remove(Cookie::build("session").path("/api/"))
        .remove(Cookie::build("user_info").path("/")))
}

/// Check if a username has already been used in the application.
///
/// It is considered taken if either an existing account or an account pending verification has the username.
///
/// # Parameters
/// `db` - Either a [`&PgPool`](PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `pending_accounts` - The [`HashMap`] from the [`State`](crate::state::State), containing the accounts
/// that are pending verification.
///
/// `check_username` - The username, for which it should be checked if it can be used or not.
///
/// `allowed_user_id` - An optional user id, who should be ignored if they have the username being checked.
///
/// # Returns
/// A [`bool`], which is true if the username has already been taken, or false if it has not.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn check_username_used(
    db: impl PgExecutor<'_>,
    pending_accounts: &HashMap<String, PendingRegisterData>,
    check_username: &str,
    allowed_user_id: Option<i32>,
) -> Result<bool, ApiError> {
    let found_user_id = get_user_by_name(db, check_username).await?;
    let used_by_other = found_user_id.is_some_and(|id| match allowed_user_id {
        // Did not specify an allowed user id, so the username is a duplicate.
        None => true,
        // If there is an allowed user, the username has been used if the allowed user is not the found user.
        Some(other_id) => id != other_id,
    });
    let used_by_pending = pending_accounts
        .values()
        .any(|r| r.register_data.data.name.to_lowercase() == check_username.to_lowercase());

    Ok(used_by_other || used_by_pending)
}

/// Check if an e-mail has already been used in the application.
///
/// It is considered taken if either an existing account or an account pending verification has the e-mail.
///
/// # Parameters
/// `db` - Either a [`&PgPool`](PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `pending_accounts` - The [`HashMap`] from the [`State`](crate::state::State), containing the accounts
/// that are pending verification.
///
/// `check_email` - The e-mail, for which it should be checked if it can be used or not.
///
/// `allowed_user_id` - An optional user id, who should be ignored if they have the username being checked.
///
/// # Returns
/// A [`bool`], which is true if the e-mail has already been taken, or false if it has not.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn check_email_used(
    db: impl PgExecutor<'_>,
    pending_accounts: &HashMap<String, PendingRegisterData>,
    check_email: &str,
    allowed_user_id: Option<i32>,
) -> Result<bool, ApiError> {
    let found_user_id = get_user_by_email(db, check_email).await?;
    let used_by_other = found_user_id.is_some_and(|id| match allowed_user_id {
        // Did not specify an allowed user id, so the email is a duplicate.
        None => true,
        // If there is an allowed user, the email has been used if the allowed user is not the found user.
        Some(other_id) => id != other_id,
    });
    let used_by_pending = pending_accounts
        .values()
        .any(|r| r.register_data.email.to_lowercase() == check_email.to_lowercase());

    Ok(used_by_other || used_by_pending)
}
