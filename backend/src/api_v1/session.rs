use axum::Json;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;
use crate::Config;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone)]
pub struct Keys {
    pub(crate) encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// The claims contained in a JWT (JSON Web Token). Specifies the subject (user id) and the
/// expiration time.
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: i32, // Subject (whom token refers to)
    pub(crate) exp: i64, // Expiration time (as UTC timestamp)
}

impl FromRequestParts<Config> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &Config) -> Result<Self, Self::Rejection> {
        // Extract the token from the cookie jar
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError::MissingToken)?;
        let auth_cookie = jar.get("session").ok_or(AuthError::MissingToken)?.value();

        // Decode the user data, this automatically validates the expiration time
        let token_data = decode::<Claims>(auth_cookie, &state.jwt_keys.decoding, &Validation::default())
            .map_err(|e| {
                if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = e.kind() {
                    AuthError::ExpiredToken
                } else {
                    AuthError::InvalidToken
                }
            })?;

        Ok(token_data.claims)
    }
}

/// An error enum describing the various errors relating to authentication.
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    MissingToken,
    InvalidToken,
    ExpiredToken,
    HashError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::FORBIDDEN, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            AuthError::TokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Session creation error")
            }
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorisation"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Expired authorisation"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authorisation"),
            AuthError::HashError => (StatusCode::INTERNAL_SERVER_ERROR, "Please try again"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
