/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::db::session::get_session;
use crate::db_structs::session::Claims;
use crate::errors::ApiError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use sqlx::PgPool;

async fn get_claims(parts: &mut Parts, _state: &ArcState) -> Result<Claims, ApiError> {
    // Extract the token from the cookie jar.
    let jar = CookieJar::from_headers(&parts.headers);
    let auth_cookie = jar
        .get("session")
        .ok_or(ApiError::MissingAuthentication)?
        .value();

    let db = parts
        .extensions
        .get::<PgPool>()
        .ok_or(ApiError::DatabaseException)?;

    get_session(db, auth_cookie)
        .await?
        .ok_or(ApiError::InvalidSession)
}

impl FromRequestParts<ArcState> for Claims {
    type Rejection = ApiError;

    /// Get [`Claims`] from a user's cookies
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
    /// Returns [`InvalidSession`](ApiError::InvalidSession) if the user's session can not be created.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        get_claims(parts, state).await
    }
}

impl FromRequestParts<ArcState> for Option<Claims> {
    type Rejection = ApiError;

    /// Get optional [`Claims`] from a user's cookies
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
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(get_claims(parts, state).await.ok())
    }
}
