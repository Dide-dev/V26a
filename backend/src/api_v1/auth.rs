/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::db::auth::get_user_credentials_by_email;
use crate::db::session::{create_session, delete_session};
use crate::db::users::get_user;
use crate::db_structs::Validated;
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::session::{Claims, InfoCookieData};
use crate::errors::ApiError;
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json, Router};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Duration;
use utoipa::ToSchema;

/// Creates an `Axum` [`Router`] for the auth category, combining its endpoints.
pub fn auth() -> Router<ArcState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

/// Return body, where the `succeeded`-field signals whether the request was successful.
/// - If the flag is `false`, the wrong credentials were passed.
/// - If the flag is `true`, the login was successful, and both a `session` and `user_info` cookie have been created.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub succeeded: bool,
}

#[utoipa::path(
    post,
    path = "/login",
    context_path = "/api/v1/auth",
    request_body = LoginRequestBody,
    responses(
        (status = 200, description = "Session and user_info cookies added.", body = LoginResponse),
        (status = 500, description = "Internal server error."),
    ),
)]
/// Login to an account if the given credentials are valid. It takes a [`LoginRequestBody`], and on success sets a session cookie.
///
/// # Arguments
/// `login_request` - The credentials provided to login, as [`LoginRequestBody`].
///
/// # Returns
/// A [`LoginResponse`] which flags whether the combination of email and password is correct.
/// If correct, it sets a `session` cookie with an opaque token and a non-HTTP `user_info` cookie.
///
/// # Errors
/// May return a [`HashException`](ApiError::HashException), [`DatabaseException`](ApiError::DatabaseException), [`TokenCreationException`](ApiError::TokenCreationException)
/// or [`SerdeException`](ApiError::SerdeException) instead.
async fn login(
    jar: CookieJar,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(login_request): Json<LoginRequestBody>,
) -> Result<(CookieJar, Json<LoginResponse>), ApiError> {
    let config = &state.config;
    login_request.validate_inputs()?;

    let Some(credentials) = get_user_credentials_by_email(&db, login_request.email).await? else {
        return Ok((jar, Json(LoginResponse { succeeded: false })));
    };

    let password_valid = bcrypt::verify(login_request.password, credentials.password_hash.as_str())
        .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Ok((jar, Json(LoginResponse { succeeded: false })));
    }

    let user_data = get_user(&db, credentials.user_id).await?;

    let user_info_cookie = InfoCookieData {
        name: user_data.data.name,
        nickname: user_data.data.nickname,
        sub: credentials.user_id,
        account_type: credentials.account_type,
    };

    // Create the authorization token.
    let token = {
        let mut rng = rand::rng();
        Alphanumeric.sample_string(&mut rng, 64)
    };

    // The age of a cookie (same for the two cookies)
    let expires_at: i64 = config.session_duration;

    create_session(
        &db,
        &token,
        credentials.user_id,
        Utc::now() + chrono::Duration::seconds(expires_at),
    )
    .await?;

    let user_info = serde_json::to_string(&user_info_cookie)?;

    // The properties of the cookies are important to get right.
    // `Secure`: This prevents the cookie from being sent in plaintext over HTTP, it will only be sent in requests over HTTPS.
    //           There is no reason to not set this property, now that every website is using HTTPS.
    // `HttpOnly`: This prevents javascript from accessing the cookie.
    // `Path`: The cookie is only sent along requests when the request path starts with this string.
    //         The cookie is also only available to the JavaScript when the webpage url starts with this string.
    // `MaxAge`: After this much time has passed, the browser will automatically delete the cookie.
    // See also https://developer.mozilla.org/en-US/docs/Web/Security/Practical_implementation_guides/Cookies

    let cookie = Cookie::build(("session", token))
        .path("/api/")
        .secure(true)
        .max_age(Duration::seconds(expires_at))
        .http_only(true);

    let user_cookie = Cookie::build(("user_info", user_info))
        .path("/")
        .secure(true)
        .max_age(Duration::seconds(expires_at))
        .http_only(false);

    Ok((
        jar.add(cookie).add(user_cookie),
        Json(LoginResponse { succeeded: true }),
    ))
}

#[utoipa::path(
    post,
    path = "/logout",
    context_path = "/api/v1/auth",
    responses(
        (status = 200, description = "User data successfully logged out."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Logout a user from the database.
/// A user can only logout their own account, if the user itself exist.
///
/// # Arguments
/// `jar` - The jar with the cookies (`user_info` & session).
///
/// `user_id` - The user to logout.
///
/// # Returns
/// The (cookie) jar that is empty.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to delete the session.
async fn logout(
    mut jar: CookieJar,
    Extension(db): Extension<PgPool>,
) -> Result<CookieJar, ApiError> {
    let token = jar
        .get("session")
        .ok_or(ApiError::NoPermission)?
        .value()
        .to_string();

    // Unvalidate the session.
    delete_session(&db, &token).await?;

    // Unset the cookie, "remove" sets the time of the cookie in the past.
    jar = jar
        .remove(Cookie::from("session"))
        .remove(Cookie::from("user_info"));

    Ok(jar)
}
