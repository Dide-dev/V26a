/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::Display;

/// The Error type to be returned by our application. Converts to various HTTP status codes and
/// error messages to be displayed on a frontend.
#[derive(Debug, PartialEq, Eq)]
pub enum ApiError {
    //------------------//
    // 400: BAD REQUEST //
    //------------------//
    /// An error to return when an e-mail has already been used and cannot be used again.
    EmailAlreadyUsed,
    /// An error to return when the new tree structure doesn't match the list of cards matched
    /// to the tree in the cards table.
    MismatchedStructure,
    /// An error to return when a card is attempted to be updated, which does not yet exist.
    MissingCard(i32),
    /// An error to return when a non-existing `tree_id` is used to access a tree.
    MissingTree(i32),
    /// An error to return when a tree is added with a default structure containing cards.
    NonEmptyStructure,
    /// An error to be returned when a file is too big to be uploaded.
    FileTooBigException,
    /// An error to return when something goes wrong while parsing multipart data.
    MultipartException,
    /// An error to return when an invalid image type is attempted to be put in the database.
    InvalidImageType,

    //-------------------//
    // 401: UNAUTHORIZED //
    //-------------------//
    /// An error to be returned by `Axum` when a user tries to access a protected endpoint with
    /// an expired session.
    ExpiredSession,
    /// An error to be returned by `Axum` when a user provides a session that can not be decoded.
    InvalidSession,
    /// An error to be returned by `Axum` when a protected endpoint is accessed without providing
    /// a session.
    MissingAuthentication,
    /// An error to return when a user tries to log in with missing credentials.
    MissingCredentials,

    //----------------//
    // 403: FORBIDDEN //
    //----------------//
    /// An error to return when a user tries to do something for which they are not properly
    /// authorized.
    NoPermission,
    /// An error to return when the API if the user does not have the right permissions to
    /// access something.
    WrongCredentials,

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
    /// An error to return when invalid data is found in the database.
    InvalidDataFound,
    /// An exception to return when serde fails on something unexpected.
    SerdeException,
    /// An error to be returned by `jsonwebtoken` if the encoding of a token fails.
    TokenCreationException,
    /// An error to be returned by `create_dir_all` when it is unable to find or create a directory.
    CreateDirException,
    /// An error to be return by `File::create` or `File::write_all` when it is unable to create or write to a file.
    FileWriteException,
    /// An error to be returned by `tokio::fs::rename` when a file cannot be renamed.
    FileRenameException,
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
            ApiError::NonEmptyStructure => write!(
                f,
                "The tree structure contains cards, make sure to initialize a tree without cards."
            ),
            ApiError::FileTooBigException => {
                write!(f, "The file being uploaded is too big.")
            }
            ApiError::MultipartException => {
                write!(
                    f,
                    "Something went wrong parsing multipartdata. Check whether the multipart request is valid and whether the multipart body is not corrupt"
                )
            }
            ApiError::InvalidImageType => {
                write!(f, "An invalid image type was uploaded.")
            }

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
            ApiError::InvalidDataFound => write!(f, "Data was found corrupted."),
            ApiError::SerdeException => {
                write!(f, "An unexpected (de)serialization error occurred.")
            }
            ApiError::TokenCreationException => {
                write!(f, "An unexpected session creation error occurred.")
            }
            ApiError::CreateDirException => {
                write!(f, "An unexpected directory creation error occurred.")
            }
            ApiError::FileWriteException => {
                write!(
                    f,
                    "An unexpected error occurred while writing a file. Check if you have write permisions, the path is valid and whether the disk has space."
                )
            }
            ApiError::FileRenameException => {
                write!(
                    f,
                    "An unexpected error occurred while renaming a file. Check whether the file exists and whether you have permisions."
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
            | ApiError::NonEmptyStructure
            | ApiError::FileTooBigException
            | ApiError::InvalidImageType
            | ApiError::MultipartException => StatusCode::BAD_REQUEST,

            ApiError::MissingCredentials
            | ApiError::MissingAuthentication
            | ApiError::ExpiredSession
            | ApiError::InvalidSession => StatusCode::UNAUTHORIZED,

            ApiError::WrongCredentials | ApiError::NoPermission => StatusCode::FORBIDDEN,

            ApiError::DatabaseException
            | ApiError::DatabaseMemoryCap
            | ApiError::HashException
            | ApiError::InvalidDataFound
            | ApiError::SerdeException
            | ApiError::TokenCreationException
            | ApiError::CreateDirException
            | ApiError::FileWriteException
            | ApiError::FileRenameException => StatusCode::INTERNAL_SERVER_ERROR,
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
