/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;
use crate::api_v1::session::Claims;
use crate::db::auth::get_user_credentials;
use crate::db_structs::auth::LoginRequestBody;
use crate::errors::ApiError;
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json, Router};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use jsonwebtoken::{Header, encode};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the auth category, combining its endpoints.
pub fn auth() -> Router<Config> {
    Router::new().route("/login", post(login))
}

#[utoipa::path(
    post,
    path = "/login",
    context_path = "/api/v1/auth",
    request_body = LoginRequestBody,
    responses(
        (status = 200, description = "Session and user_info cookies added."),
        (status = 400, description = ApiError::WrongCredentials.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("claims" = Claims, Cookie),
    )
)]
/// Login to an account if the given credentials are valid.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `login_request` - The credentials provided to login, as [`LoginRequestBody`].
///
/// # Returns
/// Sets a `session` cookie with an encoded JWT and a non-HTTP `user_info` cookie.
///
/// # Errors
/// Returns [`WrongCredentials`](ApiError::WrongCredentials) if no user could be found matching the
/// given e-mail, or the given password does not match the correct password.
///
/// May return a [`HashException`](ApiError::HashException), [`TokenCreationException`](ApiError::TokenCreationException)
/// or [`SerdeException`](ApiError::SerdeException) instead.
async fn login(
    jar: CookieJar,
    Extension(db): Extension<PgPool>,
    State(config): State<Config>,
    Json(login_request): Json<LoginRequestBody>,
) -> Result<CookieJar, ApiError> {
    let credentials = get_user_credentials(&db, login_request.email).await?;
    let password_valid = bcrypt::verify(login_request.password, credentials.password_hash.as_str())
        .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Err(ApiError::WrongCredentials);
    }

    let claims = Claims {
        sub: credentials.user_id,
        exp: Utc::now().timestamp() + config.session_duration,
        account_type: credentials.account_type,
    };

    // Create the authorization token.
    let token = encode(&Header::default(), &claims, &config.jwt_keys.encoding)
        .map_err(|_| ApiError::TokenCreationException)?;

    let user_info = serde_json::to_string(&claims).map_err(|_| ApiError::SerdeException)?;

    // Age of a cookie (same for the two cookies)
    let time = time::Duration::new(config.session_duration, 0);

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
        .max_age(time)
        .http_only(true);

    let user_cookie = Cookie::build(("user_info", user_info))
        .path("/")
        .secure(true)
        .max_age(time)
        .http_only(false);

    Ok(jar.add(cookie).add(user_cookie))
}
