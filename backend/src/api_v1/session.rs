/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::db_structs::session::{AllClassroomClaims, Claims, ClassroomClaims};
use crate::errors::ApiError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

impl FromRequestParts<ArcState> for Claims {
    type Rejection = ApiError;

    /// Get [`Claims`] from a user's cookies.
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
    /// Returns [`InvalidSession`](ApiError::InvalidSession) if the user's session can not be
    /// retrieved from the database. It may have expired.
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Claims::get_claims(parts).await
    }
}

impl FromRequestParts<ArcState> for Option<Claims> {
    type Rejection = ApiError;

    /// Get optional [`Claims`] from a user's cookies.
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
    /// - The user does not have a session cookie
    /// - The cookies can not be extracted from the request
    /// - The user's session has expired
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Claims::from_request_parts(parts, state).await.ok())
    }
}

impl FromRequestParts<ArcState> for ClassroomClaims {
    type Rejection = ApiError;

    /// Get [`ClassroomClaims`] from a user's cookies.
    ///
    /// # Example
    /// Passing `claims: `[`ClassroomClaims`] into an axum function will automatically call this function.
    /// ```rs
    /// async fn protected(claims: ClassroomClaims) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user does not
    /// have a session cookie, or if cookies can not be extracted from the request.
    ///
    /// Returns [`InvalidSession`](ApiError::InvalidSession) if the user's session can not be
    /// retrieved from the database. It may have expired.
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        ClassroomClaims::get_claims(parts).await
    }
}

impl FromRequestParts<ArcState> for Option<ClassroomClaims> {
    type Rejection = ApiError;

    /// Get optional [`ClassroomClaims`] from a user's cookies.
    ///
    /// # Example
    /// Passing `optional_claims: `[`Option<ClassroomClaims>`](ClassroomClaims) into an axum function will automatically call this function.
    /// ```rs
    /// async fn semi_protected(optional_claims: Option<ClassroomClaims>) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Returns
    /// Some([`ClassroomClaims`]) if the user is authenticated.
    ///
    /// None if one of the following is true:
    /// - The user does not have a session cookie
    /// - The cookies can not be extracted from the request
    /// - The user's session has expired
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(ClassroomClaims::from_request_parts(parts, state).await.ok())
    }
}

impl FromRequestParts<ArcState> for AllClassroomClaims {
    type Rejection = ApiError;

    /// Get [`AllClassroomClaims`] from a user's cookies.
    ///
    /// # Example
    /// Passing `claims: `[`AllClassroomClaims`] into an axum function will automatically call this function.
    /// ```rs
    /// async fn protected(claims: AllClassroomClaims) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user does not
    /// have a session cookie, or if cookies can not be extracted from the request.
    ///
    /// Returns [`InvalidSession`](ApiError::InvalidSession) if the user's session can not be
    /// retrieved from the database. It may have expired.
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        AllClassroomClaims::get_claims(parts).await
    }
}

impl FromRequestParts<ArcState> for Option<AllClassroomClaims> {
    type Rejection = ApiError;

    /// Get optional [`AllClassroomClaims`] from a user's cookies.
    ///
    /// # Example
    /// Passing `optional_claims: `[`Option<AllClassroomClaims>`](AllClassroomClaims) into an axum function will automatically call this function.
    /// ```rs
    /// async fn semi_protected(optional_claims: Option<AllClassroomClaims>) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Returns
    /// Some([`AllClassroomClaims`]) if the user is authenticated.
    ///
    /// None if one of the following is true:
    /// - The user does not have a session cookie
    /// - The cookies can not be extracted from the request
    /// - The user's session has expired
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(AllClassroomClaims::from_request_parts(parts, state)
            .await
            .ok())
    }
}
