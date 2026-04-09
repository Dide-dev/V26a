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
use serde::Serialize;
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the auth category, combining its endpoints.
pub fn auth() -> Router<Config> {
    Router::new().route("/login", post(login))
}

#[derive(Serialize)]
struct LoginResponse {
    succeeded: bool,
}

/// An endpoint for logging in. Takes a [`LoginRequestBody`], and on success sets a session cookie
/// with an encoded JWT.
///
/// # Returns
/// A [`LoginResponse`] whether the combination of email and password is correct.
///
/// # Error
/// Returns a [`HashException`](ApiError::HashException) if the password hashing fails.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn login(
    jar: CookieJar,
    Extension(db): Extension<PgPool>,
    State(config): State<Config>,
    Json(body): Json<LoginRequestBody>,
) -> Result<(CookieJar, Json<LoginResponse>), ApiError> {
    let Some(credentials) = get_user_credentials(&db, body.email).await? else {
        return Ok((jar, Json(LoginResponse { succeeded: false })));
    };

    let password_valid = bcrypt::verify(body.password, credentials.password_hash.as_str())
        .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Ok((jar, Json(LoginResponse { succeeded: false })));
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

    Ok((
        jar.add(cookie).add(user_cookie),
        Json(LoginResponse { succeeded: true }),
    ))
}
