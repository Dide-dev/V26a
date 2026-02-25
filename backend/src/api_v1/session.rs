use crate::Config;
use crate::errors::ApiError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

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
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Config,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the cookie jar
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::MissingAuthorisation)?;
        let auth_cookie = jar
            .get("session")
            .ok_or(ApiError::MissingAuthorisation)?
            .value();

        // Decode the user data, this automatically validates the expiration time
        let token_data = decode::<Claims>(
            auth_cookie,
            &state.jwt_keys.decoding,
            &Validation::default(),
        )
        .map_err(|e| {
            if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = e.kind() {
                ApiError::ExpiredSession
            } else {
                ApiError::InvalidSession
            }
        })?;

        Ok(token_data.claims)
    }
}
