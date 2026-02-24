use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Display;

pub enum ApiError {
    UserAlreadyExistsError,
    DatabaseException,
    Text(String),
}

// Define how errors should be displayed
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ApiError::UserAlreadyExistsError => {
                write!(f, "A user with this e-mail address already exists.")
            }
            ApiError::DatabaseException => write!(f, "An unexpected database error occurred"),
            ApiError::Text(s) => write!(f, "Error: {s}"),
        }
    }
}

impl ApiError {
    fn to_status_code(&self) -> StatusCode {
        match &self {
            ApiError::UserAlreadyExistsError => StatusCode::BAD_REQUEST,
            ApiError::DatabaseException | ApiError::Text(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        let status = self.to_status_code();
        (status, message).into_response()
    }
}
