use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Display;

/// The Error type to be returned by our application. Converts to various HTTP status codes and
/// error messages to be displayed on a frontend.
#[allow(dead_code)]
pub enum ApiError {
    /// An error to return when an e-mail has already been used and cannot be used again.
    EmailAlreadyUsed,
    /// An error to return when a non-existing `tree_id` is used to access a tree.
    MissingTree(i32),
    /// An exception to throw when a card is attempted to be updated, which does not yet exist.
    MissingCard(i32),
    /// A generic exception to return when the database fails on something unexpected.
    /// Do not use this in place of proper error handling.
    DatabaseException,
    /// An error to return when hashing fails (on creating or verifying a hash).
    HashException,
    /// An error to return when the API if the user does not have the right permissions to
    /// access something.
    WrongCredentials,
    /// An error to return when a user tries to log in with missing credentials.
    MissingCredentials,
    /// An error to be returned by `jsonwebtoken` if the encoding of a token fails.
    TokenCreationException,
    /// An error to be returned by `Axum` when a protected endpoint is accessed without providing
    /// a session.
    MissingAuthentication,
    /// An error to return when a user tries to do something for which they are not properly
    /// authorized.
    NoPermission,
    /// An error to be returned by `Axum` when a user provides a session that can not be decoded.
    InvalidSession,
    /// An error to be returned by `Axum` when a user tries to access a protected endpoint with
    /// an expired session.
    ExpiredSession,
    /// A warning to be given when a request is missing one or more fields, which are required for
    /// the request to process successfully.
    ///
    /// # Arguments
    /// `Vec<String>` - the list of fields, of which some must be defined.
    IncompleteRequest(Vec<String>),
    /// An exception to be given when a request contains unexpected data for a certain field.
    IncorrectRequest(String),
}

// Define how errors should be converted to messages.
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ApiError::EmailAlreadyUsed => {
                write!(f, "A user with this e-mail address already exists.")
            }
            ApiError::MissingTree(id) => write!(f, "The tree with id {id} could not be found."),
            ApiError::MissingCard(id) => write!(f, "The card with id {id} could not be found."),
            ApiError::DatabaseException => write!(f, "An unexpected database error occurred."),
            ApiError::HashException => write!(f, "An unexpected hash error occurred."),
            ApiError::WrongCredentials => write!(f, "Wrong credentials."),
            ApiError::MissingCredentials => write!(f, "Missing credentials."),
            ApiError::TokenCreationException => {
                write!(f, "An unexpected session creation error occurred.")
            }
            ApiError::MissingAuthentication => write!(f, "You need to be logged in to do this."),
            ApiError::NoPermission => write!(f, "You are not authorised to do this."),
            ApiError::ExpiredSession => write!(f, "Your session has expired, please log in again."),
            ApiError::InvalidSession => write!(f, "Invalid authorisation."),
            ApiError::IncompleteRequest(fields) => write!(
                f,
                "The request did not process successfully, please specify one of the '{}' field{}.",
                fields.join("', '"),
                if fields.len() > 1 { "s" } else { "" }
            ),
            ApiError::IncorrectRequest(field) => write!(
                f,
                "The request did not process successfully, please specify the '{field}' field correctly."
            ),
        }
    }
}

impl ApiError {
    /// Convert our [`ApiError`] into a [`StatusCode`] usable by `Axum`.
    fn to_status_code(&self) -> StatusCode {
        match &self {
            ApiError::EmailAlreadyUsed
            | ApiError::MissingTree(_)
            | ApiError::MissingCard(_)
            | ApiError::IncompleteRequest(_)
            | ApiError::IncorrectRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::WrongCredentials | ApiError::NoPermission => StatusCode::FORBIDDEN,
            ApiError::DatabaseException
            | ApiError::HashException
            | ApiError::TokenCreationException => StatusCode::INTERNAL_SERVER_ERROR,
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
