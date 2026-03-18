use crate::Config;
use crate::api_v1::session::Claims;
use crate::db::auth::get_user_id_and_password;
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
    // Check if the user sent the credentials
    if body.email.is_empty() || body.password.is_empty() {
        return Err(ApiError::MissingCredentials);
    }

    let (user_id, password_hash) = get_user_id_and_password(&db, body.email).await?;
    let password_valid = bcrypt::verify(body.password, password_hash.as_str())
        .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Err(ApiError::WrongCredentials);
    }

    let claims = Claims {
        sub: user_id,
        exp: Utc::now().timestamp() + config.session_duration,
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

    Ok(jar.add(cookie))
}
