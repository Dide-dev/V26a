use axum::{Json, Router};
use axum::routing::{get, post};
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use crate::api_v1::session::{AuthError, Claims, KEYS};

/// Creates an `Axum` [`Router`] for the auth category, combining its endpoints.
pub fn auth() -> Router {
    Router::new()
        .route("/login", post(login))
}

#[derive(Deserialize)]
struct LoginRequestBody {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponseBody {
    access_token: String,
    token_type: String,
}

async fn login(Json(body): Json<LoginRequestBody>) -> Result<Json<LoginResponseBody>, AuthError> {
    // Check if the user sent the credentials
    if body.email.is_empty() || body.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if body.email != "foo" || body.password != "bar" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: 1, // TODO: get user id out of db
        exp: 2_000_000_000, // TODO: think of a TTL
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(LoginResponseBody {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

