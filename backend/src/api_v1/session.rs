use crate::Config;
use crate::errors::ApiError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

/// A struct containing the [`EncodingKey`] and [`DecodingKey`] used for encoding and decoding the
/// `JsonWebToken`.
#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    /// Creates a [`Keys`] from an array of bytes.
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
    /// The subject (whom the token refers to).
    pub sub: i32,
    /// The expiration time (as UTC timestamp).
    pub exp: i64,
}

impl FromRequestParts<Config> for Claims {
    type Rejection = ApiError;

    /// TODO: Add documentation, specifically on the possible errors that may occur.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Config,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the cookie jar.
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::MissingAuthentication)?;
        let auth_cookie = jar
            .get("session")
            .ok_or(ApiError::MissingAuthentication)?
            .value();

        // Decode the user data, this automatically validates the expiration time.
        let token_data = decode::<Claims>(
            auth_cookie,
            &state.jwt_keys.decoding,
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            // exp gets validated by the library, but we want to map it to our own error type.
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::ExpiredSession,
            // other validation error, usually means that the decoding failed (completely invalid session).
            _ => ApiError::InvalidSession,
        })?;

        Ok(token_data.claims)
    }
}
