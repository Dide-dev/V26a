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
    /// An error to return when a name contains characters which fall outside of the accepted standard.
    IllegalName,
    /// An error to return when an input provided is too long for the database.
    InputLengthExceeded(&'static str, usize),
    /// An error to return when an invalid image type is attempted to be put in the database.
    InvalidImageType,
    /// An error to return when an invalid isbn is attempted to be put in the database.
    InvalidIsbn,
    /// An error to return when the user requests a page that doesn't exist.
    InvalidPage,
    /// An error to return when the user requests a page number that exceeds the defined maximum.
    MaxPageNumberExceeded(i32),
    /// An error to return when the user requests a page size that exceeds the defined maximum.
    MaxPageSizeExceeded(i32),
    /// An error to return when the new tree structure doesn't match the list of cards matched
    /// to the tree in the cards table.
    MismatchedStructure,
    /// An error to return when a card is attempted to be updated, which does not yet exist.
    MissingCard(i32),
    /// An error to return when a non-existing `language` is used when creating or updating a tree.
    MissingLanguage(String),
    /// An error to return when a non-existing `subject_id` is used when creating or updating a tree.
    MissingSubject(i32),
    /// An error to return when a non-existing `tree_id` is used to access a tree.
    MissingTree(i32),
    /// An error to return when a non-existing `user_id` is used to access a user.
    MissingUser(i32),
    /// An error to return when `add_new_image` has been called with an empty body.
    NoImageBodyProvidedException,
    /// An error to return when a tree is added with a default structure containing cards.
    NonEmptyStructure,

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

    //----------------//
    // 403: FORBIDDEN //
    //----------------//
    /// An error to return when a user tries to do something for which they are not properly
    /// authorized.
    NoPermission,

    //----------------------------//
    // 500: INTERNAL SERVER ERROR //
    //----------------------------//
    /// An error to be returned by `create_dir_all` when it is unable to find or create a directory.
    CreateDirException,
    /// A generic exception to return when the database fails on something unexpected.
    /// Do not use this in place of proper error handling.
    DatabaseException,
    /// An error to return when a request is denied because a limit on the database memory
    /// disallows it.
    DatabaseMemoryCap,
    /// An error to be return by `File::create` or `File::write_all` when it is unable to create or write to a file.
    FileWriteException,
    /// An error to return when hashing fails (on creating or verifying a hash).
    HashException,
    /// An error to return when invalid data is found in the database.
    InvalidDataFound,
    /// An error to be returned by `add_image` if adding the image to the database fails.
    ImageAddFail,
    /// An exception to return when serde fails on something unexpected.
    SerdeException,
    /// An error to be returned by `jsonwebtoken` if the encoding of a token fails.
    TokenCreationException,
}

impl From<sqlx::Error> for ApiError {
    /// Convert `SQLx` errors to our own [`ApiError`], so that ? can be used on `SQLx` results.
    fn from(error: sqlx::Error) -> Self {
        tracing::error!("sqlx error: {}", error);
        ApiError::DatabaseException
    }
}

impl From<serde_json::Error> for ApiError {
    /// Convert `serde` errors to our own [`ApiError`], so that ? can be used on `serde` results.
    fn from(error: serde_json::Error) -> Self {
        tracing::error!("serde error: {}", error);
        ApiError::SerdeException
    }
}

// Define how errors should be converted to messages.
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            //------------------//
            // 400: BAD REQUEST //
            //------------------//
            ApiError::IllegalName => write!(f, "Name contains illegal characters."),
            ApiError::InputLengthExceeded(offending_string, max_length) => {
                write!(
                    f,
                    "Input in field \"{offending_string}\" exceeds the maximum length of {max_length} characters."
                )
            }
            ApiError::InvalidImageType => {
                write!(f, "An invalid image type was uploaded.")
            }
            ApiError::InvalidIsbn => {
                write!(f, "An invalid isbn was uploaded.")
            }
            ApiError::InvalidPage => {
                write!(f, "This page does not exist.")
            }
            ApiError::MaxPageNumberExceeded(max_page) => {
                write!(f, "You may not request a page higher than {max_page}.")
            }
            ApiError::MaxPageSizeExceeded(max_size) => {
                write!(
                    f,
                    "You may not request more than {max_size} items per page."
                )
            }
            ApiError::MismatchedStructure => write!(
                f,
                "The provided structure does not match the cards found in the database. Make sure to update the database with any new or removed cards first."
            ),
            ApiError::MissingCard(id) => write!(f, "The card with id {id} could not be found."),
            ApiError::MissingLanguage(lang) => write!(f, "The language {lang} does not exist."),
            ApiError::MissingSubject(id) => write!(f, "The subject with id {id} does not exist."),
            ApiError::MissingTree(id) => write!(f, "The tree with id {id} could not be found."),
            ApiError::MissingUser(id) => write!(f, "The user with id {id} could not be found."),
            ApiError::NoImageBodyProvidedException => {
                write!(f, "No image body has been provided.")
            }
            ApiError::NonEmptyStructure => write!(
                f,
                "The tree structure contains cards, make sure to initialize a tree without cards."
            ),

            //-------------------//
            // 401: UNAUTHORIZED //
            //-------------------//
            ApiError::ExpiredSession => write!(f, "Your session has expired, please log in again."),
            ApiError::InvalidSession => write!(f, "Invalid authorisation."),
            ApiError::MissingAuthentication => write!(f, "You need to be logged in to do this."),

            //----------------//
            // 403: FORBIDDEN //
            //----------------//
            ApiError::NoPermission => write!(f, "You are not authorised to do this."),

            //----------------------------//
            // 500: INTERNAL SERVER ERROR //
            //----------------------------//
            ApiError::CreateDirException => {
                write!(f, "An unexpected directory creation error occurred.")
            }
            ApiError::DatabaseException => write!(f, "An unexpected database error occurred."),
            ApiError::DatabaseMemoryCap => write!(
                f,
                "Request was denied, because a database memory cap was exceeded."
            ),
            ApiError::FileWriteException => {
                write!(
                    f,
                    "An unexpected error occurred while writing a file. Check if you have write permisions, the path is valid and whether the disk has space."
                )
            }
            ApiError::HashException => write!(f, "An unexpected hash error occurred."),
            ApiError::ImageAddFail => {
                write!(
                    f,
                    "Something went wrong while adding the image to the database"
                )
            }
            ApiError::InvalidDataFound => write!(f, "Data was found corrupted."),
            ApiError::SerdeException => {
                write!(f, "An unexpected (de)serialization error occurred.")
            }
            ApiError::TokenCreationException => {
                write!(f, "An unexpected session creation error occurred.")
            }
        }
    }
}

impl ApiError {
    /// Convert our [`ApiError`] into a [`StatusCode`] usable by `Axum`.
    fn to_status_code(&self) -> StatusCode {
        match &self {
            ApiError::IllegalName
            | ApiError::InputLengthExceeded(_, _)
            | ApiError::InvalidImageType
            | ApiError::InvalidIsbn
            | ApiError::InvalidPage
            | ApiError::MaxPageNumberExceeded(_)
            | ApiError::MaxPageSizeExceeded(_)
            | ApiError::MismatchedStructure
            | ApiError::MissingCard(_)
            | ApiError::MissingLanguage(_)
            | ApiError::MissingSubject(_)
            | ApiError::MissingTree(_)
            | ApiError::MissingUser(_)
            | ApiError::NoImageBodyProvidedException
            | ApiError::NonEmptyStructure => StatusCode::BAD_REQUEST,

            ApiError::ExpiredSession
            | ApiError::InvalidSession
            | ApiError::MissingAuthentication => StatusCode::UNAUTHORIZED,

            ApiError::NoPermission => StatusCode::FORBIDDEN,

            ApiError::CreateDirException
            | ApiError::DatabaseException
            | ApiError::DatabaseMemoryCap
            | ApiError::FileWriteException
            | ApiError::HashException
            | ApiError::ImageAddFail
            | ApiError::InvalidDataFound
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
