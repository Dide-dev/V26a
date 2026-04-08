/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;
use crate::api_v1::session::Claims;
use crate::db::auth::get_user_id_and_password;
use crate::db::users::get_user_account_type;
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json, Router};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use jsonwebtoken::{Header, encode};
use serde::Serialize;
use sqlx::PgPool;
use std::str::FromStr;

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
    let Some((user_id, password_hash)) = get_user_id_and_password(&db, body.email).await? else {
        return Ok((jar, Json(LoginResponse { succeeded: false })));
    };

    let password_valid = bcrypt::verify(body.password, password_hash.as_str())
        .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Ok((jar, Json(LoginResponse { succeeded: false })));
    }

    // Get account type
    let account_type = get_user_account_type(&db, user_id).await?;

    let claims = Claims {
        sub: user_id,
        exp: Utc::now().timestamp() + config.session_duration,
        account_type: AccountType::from_str(&account_type)?,
    };

    // Create the authorization token.
    let token = encode(&Header::default(), &claims, &config.jwt_keys.encoding)
        .map_err(|_| ApiError::TokenCreationException)?;

    // Age of a cookie (same for the two cookies)
    let time = time::Duration::new(config.session_duration, 0);

    // Set the authorization token as a cookie.
    let cookie = Cookie::build(("session", token))
        .path("/")
        .secure(true)
        .max_age(time)
        .http_only(true);

    // Create data fields for second cookie
    let user_info = serde_json::json!({
        "userId": user_id,
        "accountType": AccountType::from_str(&account_type)?
    });

    // not http cookie
    let user_cookie = Cookie::build(("user_info", user_info.to_string()))
        .path("/")
        .secure(false)
        .max_age(time)
        .http_only(false);

    Ok((
        jar.add(cookie).add(user_cookie),
        Json(LoginResponse { succeeded: true }),
    ))
}
