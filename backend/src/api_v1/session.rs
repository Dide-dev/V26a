/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::errors::ApiError;
use crate::structs::session::{AllClassroomSessions, ClassroomSession, Session};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

impl FromRequestParts<ArcState> for Session {
    type Rejection = ApiError;

    /// Get [`Session`] from a user's cookies.
    ///
    /// # Example
    /// Passing `session: `[`Session`] into an axum function will automatically call this function.
    /// ```rs
    /// async fn protected(session: Session) -> Result<(), ApiError> {
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
    ///
    /// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
    /// invalid or corrupted.
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Session::get_session(parts).await
    }
}

impl FromRequestParts<ArcState> for Option<Session> {
    type Rejection = ApiError;

    /// Get optional [`Session`] from a user's cookies.
    ///
    /// # Example
    /// Passing `optional_session: `[`Option<Session>`](Session) into an axum function will automatically call this function.
    /// ```rs
    /// async fn semi_protected(optional_session: Option<Session>) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Returns
    /// Some([`Session`]) if the user is authenticated.
    ///
    /// None if one of the following is true:
    /// - The user does not have a session cookie
    /// - The cookies can not be extracted from the request
    /// - The user's session has expired
    /// - The user's data in the database is invalid or corrupted
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Session::from_request_parts(parts, state).await.ok())
    }
}

impl FromRequestParts<ArcState> for ClassroomSession {
    type Rejection = ApiError;

    /// Get [`ClassroomSession`] from a user's cookies.
    ///
    /// # Example
    /// Passing `session: `[`ClassroomSession`] into an axum function will automatically call this function.
    /// ```rs
    /// async fn protected(session: ClassroomSession) -> Result<(), ApiError> {
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
        ClassroomSession::get_session(parts).await
    }
}

impl FromRequestParts<ArcState> for Option<ClassroomSession> {
    type Rejection = ApiError;

    /// Get optional [`ClassroomSession`] from a user's cookies.
    ///
    /// # Example
    /// Passing `optional_session: `[`Option<ClassroomSession>`](ClassroomSession) into an axum function will automatically call this function.
    /// ```rs
    /// async fn semi_protected(optional_session: Option<ClassroomSession>) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Returns
    /// Some([`ClassroomSession`]) if the user is authenticated.
    ///
    /// None if one of the following is true:
    /// - The user does not have a session cookie
    /// - The cookies can not be extracted from the request
    /// - The user's session has expired
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(ClassroomSession::from_request_parts(parts, state)
            .await
            .ok())
    }
}

impl FromRequestParts<ArcState> for AllClassroomSessions {
    type Rejection = ApiError;

    /// Get [`AllClassroomSessions`] from a user's cookies.
    ///
    /// # Example
    /// Passing `session: `[`AllClassroomSessions`] into an axum function will automatically call this function.
    /// ```rs
    /// async fn protected(session: ClassroomSessions) -> Result<(), ApiError> {
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
        AllClassroomSessions::get_session(parts).await
    }
}

impl FromRequestParts<ArcState> for Option<AllClassroomSessions> {
    type Rejection = ApiError;

    /// Get optional [`AllClassroomSessions`] from a user's cookies.
    ///
    /// # Example
    /// Passing `optional_session: `[`Option<ClassroomSessions>`](AllClassroomSessions) into an axum function will automatically call this function.
    /// ```rs
    /// async fn semi_protected(optional_session: Option<ClassroomSessions>) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Returns
    /// Some([`AllClassroomSessions`]) if the user is authenticated.
    ///
    /// None if one of the following is true:
    /// - The user does not have a session cookie
    /// - The cookies can not be extracted from the request
    /// - The user's session has expired
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(AllClassroomSessions::from_request_parts(parts, state)
            .await
            .ok())
    }
}
