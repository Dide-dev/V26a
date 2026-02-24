use crate::api_v1::session::{AuthError, Claims, KEYS};
use axum::routing::post;
use axum::{Extension, Json, Router};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{Header, encode};
use serde::Deserialize;
use chrono::Utc;
use crate::Config;

/// Creates an `Axum` [`Router`] for the auth category, combining its endpoints.
pub fn auth() -> Router {
    Router::new().route("/login", post(login))
}

/// The body for a login request that contains the email address and password.
#[derive(Deserialize)]
struct LoginRequestBody {
    email: String,
    password: String,
}

/// An endpoint for logging in. Takes a [`LoginRequestBody`], and on success sets a session cookie
/// with an encoded JWT.
async fn login(jar: CookieJar, config: Extension<Config>, Json(body): Json<LoginRequestBody>) -> Result<CookieJar, AuthError> {
    // Check if the user sent the credentials
    if body.email.is_empty() || body.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // TODO: Check the user credentials
    if body.email != "foo" || body.password != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        sub: 1, // TODO: get user id out of db
        exp: Utc::now().timestamp() + config.session_duration
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Create the authorized token as a cookie
    let cookie = Cookie::build(("session", token))
        .path("/")
        .secure(true)
        .max_age(time::Duration::new(config.session_duration, 0))
        .http_only(true);

    Ok(jar.add(cookie))
}
