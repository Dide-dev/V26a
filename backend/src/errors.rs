use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Display;

/// The Error type to be returned by our application. Converts to various HTTP status codes and
/// error messages to be displayed on a frontend.
pub enum ApiError {
    //------------------//
    // 400: BAD REQUEST //
    //------------------//
    /// An error to return when an e-mail has already been used and cannot be used again.
    EmailAlreadyUsed,
    /// An error to return when the new tree structure doesn't match the list of cards matched
    /// to the tree in the cards table.
    MismatchedStructure,
    /// An error to return when a non-existing `tree_id` is used to access a tree.
    MissingTree(i32),
    /// An error to return when a card is attempted to be updated, which does not yet exist.
    MissingCard(i32),
    /// An error to return when the user requests a page that doesn't exist.
    InvalidPage,
    /// An error to return when the user requests a page size that exceeds the defined maximum.
    MaxPageSizeExceeded(i32),

    //-------------------//
    // 401: UNAUTHORIZED //
    //-------------------//
    /// An error to be returned by `Axum` when a user provides a session that can not be decoded.
    InvalidSession,
    /// An error to be returned by `Axum` when a user tries to access a protected endpoint with
    /// an expired session.
    ExpiredSession,
    /// An error to return when a user tries to log in with missing credentials.
    MissingCredentials,
    /// An error to be returned by `Axum` when a protected endpoint is accessed without providing
    /// a session.
    MissingAuthentication,

    //----------------//
    // 403: FORBIDDEN //
    //----------------//
    /// An error to return when the API if the user does not have the right permissions to
    /// access something.
    WrongCredentials,
    /// An error to return when a user tries to do something for which they are not properly
    /// authorized.
    NoPermission,

    //----------------------------//
    // 500: INTERNAL SERVER ERROR //
    //----------------------------//
    /// A generic exception to return when the database fails on something unexpected.
    /// Do not use this in place of proper error handling.
    DatabaseException,
    /// An error to return when a request is denied because a limit on the database memory
    /// disallows it.
    DatabaseMemoryCap,
    /// An error to return when hashing fails (on creating or verifying a hash).
    HashException,
    /// An exception to return when serde fails on something unexpected.
    SerdeException,
    /// An error to be returned by `jsonwebtoken` if the encoding of a token fails.
    TokenCreationException,
}

// Define how errors should be converted to messages.
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ApiError::EmailAlreadyUsed => {
                write!(f, "A user with this e-mail address already exists.")
            }
            ApiError::MismatchedStructure => write!(
                f,
                "The provided structure does not match the cards found in the database. Make sure to update the database with any new or removed cards first."
            ),
            ApiError::MissingTree(id) => write!(f, "The tree with id {id} could not be found."),
            ApiError::MissingCard(id) => write!(f, "The card with id {id} could not be found."),

            ApiError::InvalidSession => write!(f, "Invalid authorisation."),
            ApiError::ExpiredSession => write!(f, "Your session has expired, please log in again."),
            ApiError::MissingCredentials => write!(f, "Missing credentials."),
            ApiError::MissingAuthentication => write!(f, "You need to be logged in to do this."),

            ApiError::WrongCredentials => write!(f, "Wrong credentials."),
            ApiError::NoPermission => write!(f, "You are not authorised to do this."),

            ApiError::DatabaseException => write!(f, "An unexpected database error occurred."),
            ApiError::DatabaseMemoryCap => write!(
                f,
                "Request was denied, because a database memory cap was exceeded."
            ),
            ApiError::HashException => write!(f, "An unexpected hash error occurred."),
            ApiError::SerdeException => {
                write!(f, "An unexpected (de)serialization error occurred.")
            }
            ApiError::TokenCreationException => {
                write!(f, "An unexpected session creation error occurred.")
            }
            ApiError::InvalidPage => {
                write!(f, "This page does not exist.")
            }
            ApiError::MaxPageSizeExceeded(max_size) => {
                write!(
                    f,
                    "You may not request more than {max_size} items per page."
                )
            }
        }
    }
}

impl ApiError {
    /// Convert our [`ApiError`] into a [`StatusCode`] usable by `Axum`.
    fn to_status_code(&self) -> StatusCode {
        match &self {
            ApiError::EmailAlreadyUsed
            | ApiError::MismatchedStructure
            | ApiError::MissingTree(_)
            | ApiError::MissingCard(_)
            | ApiError::InvalidPage
            | ApiError::MaxPageSizeExceeded(_) => StatusCode::BAD_REQUEST,

            ApiError::MissingCredentials
            | ApiError::MissingAuthentication
            | ApiError::ExpiredSession
            | ApiError::InvalidSession => StatusCode::UNAUTHORIZED,

            ApiError::WrongCredentials | ApiError::NoPermission => StatusCode::FORBIDDEN,

            ApiError::DatabaseException
            | ApiError::DatabaseMemoryCap
            | ApiError::HashException
            | ApiError::SerdeException
            | ApiError::TokenCreationException => StatusCode::INTERNAL_SERVER_ERROR,
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
