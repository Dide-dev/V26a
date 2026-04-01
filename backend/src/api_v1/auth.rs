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

/// An endpoint for logging in. Takes a [`LoginRequestBody`], and on success sets a session cookie
/// with an encoded JWT.
async fn login(
    jar: CookieJar,
    Extension(db): Extension<PgPool>,
    State(config): State<Config>,
    Json(body): Json<LoginRequestBody>,
) -> Result<CookieJar, ApiError> {
    let (user_id, password_hash) = get_user_id_and_password(&db, body.email).await?;
    let password_valid = bcrypt::verify(body.password, password_hash.as_str())
        .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Err(ApiError::WrongCredentials);
    }

    // Get account type
    let account_type = get_user_account_type(&db, user_id).await?;

    // Create data fields for second cookie
    let user_info = serde_json::json!({
        "userId": user_id,
        "accountType": account_type
    });


    let claims = Claims {
        sub: user_id,
        exp: Utc::now().timestamp() + config.session_duration,
        account_type,
    };

    // Create the authorization token.
    let token = encode(&Header::default(), &claims, &config.jwt_keys.encoding)
        .map_err(|_| ApiError::TokenCreationException)?;

    // Set the authorization token as a cookie.
    let cookie = Cookie::build(("session", token))
        .path("/")
        .secure(true)
        .max_age(time::Duration::new(config.session_duration, 0))
        .http_only(true);

    // not http cookie
    let user_cookie = Cookie::build(("user_info", user_info.to_string()))
        .path("/")
        .secure(false)
        .http_only(false);

    Ok(jar.add(cookie).add(user_cookie))
}
