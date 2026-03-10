use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Display;

/// The Error type to be returned by our application. Converts to various HTTP status codes and
/// error messages to be displayed on a frontend.
#[allow(dead_code)]
pub enum ApiError {
    /// An exception to throw when an e-mail has already been used and can not be used again.
    EmailAlreadyUsed,
    /// An exception to throw when a non-existing `tree_id` is used to access a tree.
    MissingTree(i32),
    /// Generic exception when the database fails on something unexpected. Do not use this in place
    /// of proper error handling.
    DatabaseException,
    /// An exception to throw when hashing fails (on creating or verifying a hash).
    HashException,
    /// An exception to be given by the API if the user does not have the right permissions to
    /// access something.
    WrongCredentials,
    /// An exception to be given when a user tries to log in with missing credentials.
    MissingCredentials,
    /// An exception to be given by `jsonwebtoken` if the encoding of a token fails.
    TokenCreation,
    /// An exception to be given by `Axum` when a protected endpoint is accessed without providing
    /// a session.
    MissingAuthentication,
    /// An exception to be given when a user tries to do something for which they are not properly
    /// authorized.
    MissingAuthorisation,
    /// An exception to be given by `Axum` when a user provides a session that can not be decoded.
    InvalidSession,
    /// An exception to be given by `Axum` when a user tries to access a protected endpoint with
    /// an expired session.
    ExpiredSession,
}

// Define how errors should be converted to messages.
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ApiError::EmailAlreadyUsed => {
                write!(f, "A user with this e-mail address already exists.")
            }
            ApiError::MissingTree(id) => write!(f, "The tree with id {id} could not be found."),
            ApiError::DatabaseException => write!(f, "An unexpected database error occurred."),
            ApiError::HashException => write!(f, "An unexpected hash error occurred."),
            ApiError::WrongCredentials => write!(f, "Wrong credentials"),
            ApiError::MissingCredentials => write!(f, "Missing credentials"),
            ApiError::TokenCreation => write!(f, "Session creation error"),
            ApiError::MissingAuthentication => write!(f, "You need to login first."),
            ApiError::MissingAuthorisation => write!(f, "You are not authorised to do this."),
            ApiError::ExpiredSession => write!(f, "Your session has expired, please log in again."),
            ApiError::InvalidSession => write!(f, "Invalid authorisation"),
        }
    }
}

impl ApiError {
    /// Convert our [`ApiError`] into a [`StatusCode`] usable by `Axum`.
    fn to_status_code(&self) -> StatusCode {
        match &self {
            ApiError::EmailAlreadyUsed | ApiError::MissingTree(_) => StatusCode::BAD_REQUEST,
            ApiError::WrongCredentials | ApiError::MissingAuthorisation => StatusCode::FORBIDDEN,
            ApiError::DatabaseException | ApiError::HashException | ApiError::TokenCreation => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::MissingCredentials
            | ApiError::MissingAuthentication
            | ApiError::ExpiredSession
            | ApiError::InvalidSession => StatusCode::UNAUTHORIZED,
        }
    }
}

impl IntoResponse for ApiError {
    /// Convert our [`ApiError`] into an `Axum` [`Response`].
    fn into_response(self) -> Response {
        let message = self.to_string();
        let status = self.to_status_code();
        (status, message).into_response()
    }
}
