/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    /// The subject (whom the token refers to).
    pub sub: i32,
    /// The expiration time (as UTC timestamp).
    pub exp: i64,
    /// the account type
    pub account_type: AccountType,
}

/// User cookie info
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InfoCookieData {
    pub name: String,
    pub nickname: String,
    // Copy from `Claims`
    pub sub: i32,
    pub exp: i64,
    pub account_type: AccountType,
}

fn get_claims(parts: &mut Parts, state: &ArcState) -> Result<Claims, ApiError> {
    // Extract the token from the cookie jar.
    let jar = CookieJar::from_headers(&parts.headers);
    let auth_cookie = jar
        .get("session")
        .ok_or(ApiError::MissingAuthentication)?
        .value();

    // Decode the user data, this automatically validates the expiration time.
    let token_data = decode::<Claims>(
        auth_cookie,
        &state.config.jwt_keys.decoding,
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        // exp gets validated by the library, but we want to map it to our own error type.
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::ExpiredSession,
        // Other validation error, usually means that the decoding failed (completely invalid session).
        _ => ApiError::InvalidSession,
    })?;

    Ok(token_data.claims)
}

impl FromRequestParts<ArcState> for Claims {
    type Rejection = ApiError;

    /// Get JWT [`Claims`] from a user's cookies
    ///
    /// # Example
    /// Passing `claims: `[`Claims`] into an axum function will automatically call this function.
    /// ```rs
    /// async fn protected(claims: Claims) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user does not
    /// have a session cookie, or if cookies can not be extracted from the request.
    ///
    /// Returns [`ExpiredSession`](ApiError::ExpiredSession) if the user's session has expired.
    ///
    /// Returns [`InvalidSession`](ApiError::InvalidSession) if the user's session can not be
    /// decrypted, or if another error is given by [`jsonwebtoken`].
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        get_claims(parts, state)
    }
}

impl FromRequestParts<ArcState> for Option<Claims> {
    type Rejection = ApiError;

    /// Get optional JWT [`Claims`] from a user's cookies
    ///
    /// # Example
    /// Passing `optional_claims: `[`Option<Claims>`](Claims) into an axum function will automatically call this function.
    /// ```rs
    /// async fn semi_protected(optional_claims: Option<Claims>) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Returns
    /// Some([`Claims`]) if the user is authenticated.
    ///
    /// None if one of the following is true:
    /// - The user does not have a session cookie,
    /// - The cookies can not be extracted from the request,
    /// - The user's session has expired,
    /// - The user's session can not be decrypted,
    /// - another error is given by [`jsonwebtoken`]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(get_claims(parts, state).ok())
    }
}
