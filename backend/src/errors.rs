use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Display;

// This currently serves as an example, remove the allow statement when we have an error type that contains a string
#[allow(dead_code)]
pub enum ApiError {
    EmailAlreadyUsedException,
    DatabaseException,
    HashException,
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    MissingAuthorisation,
    InvalidSession,
    ExpiredSession,
    Text(String),
}

// Define how errors should be displayed
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ApiError::EmailAlreadyUsedException => {
                write!(f, "A user with this e-mail address already exists.")
            }
            ApiError::DatabaseException => write!(f, "An unexpected database error occurred."),
            ApiError::HashException => write!(f, "An unexpected hash error occurred."),
            ApiError::WrongCredentials => write!(f, "Wrong credentials"),
            ApiError::MissingCredentials => write!(f, "Missing credentials"),
            ApiError::TokenCreation => write!(f, "Session creation error"),
            ApiError::MissingAuthorisation => write!(f, "Missing authorisation"),
            ApiError::ExpiredSession => write!(f, "Expired authorisation"),
            ApiError::InvalidSession => write!(f, "Invalid authorisation"),
            ApiError::Text(s) => write!(f, "Error: {s}"),
        }
    }
}

impl ApiError {
    /// Convert our [`ApiError`] into a [`StatusCode`] usable by `Axum`.
    fn to_status_code(&self) -> StatusCode {
        match &self {
            ApiError::EmailAlreadyUsedException => StatusCode::BAD_REQUEST,
            ApiError::WrongCredentials => StatusCode::FORBIDDEN,
            ApiError::DatabaseException
            | ApiError::HashException
            | ApiError::TokenCreation
            | ApiError::Text(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MissingCredentials
            | ApiError::MissingAuthorisation
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
