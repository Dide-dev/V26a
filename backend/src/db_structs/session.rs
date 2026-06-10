/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgExecutor};
use std::fmt::Debug;
use utoipa::ToSchema;

/// The claims specify the user, expiration time and account type of the request's sender.
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    /// The subject (whom the token refers to).
    pub sub: i32,
    /// The account type
    pub account_type: AccountType,
}

/// User cookie info
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InfoCookieData {
    pub name: String,
    pub nickname: String,
    pub sub: i32,
    pub account_type: AccountType,
}

/// The classroom claims contained in a JWT (JSON Web Token).
/// Specifies the user number, user name and the expiration time.
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomClaims {
    /// The classroom id
    pub id: i32,
    /// The user number (unique in the classroom).
    pub num: i32,
    /// The user name (not unique).
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllClassroomClaims {
    pub claims: Vec<ClassroomClaims>,
}

/// Trait to ensure these function are only used for structs that implement this trait
/// (so only our cookie structs).
pub trait ClaimsType: Sized + Debug {
    /// The type of options that are retrieved from the api uri.
    type PathOptions;

    /// A type that defines the information necessary to execute the create and delete sessions functions.
    type ManageSessionInfo;

    /// Get the cookie name connected to the cookie struct
    fn get_cookie_name() -> &'static str;

    /// Get the value of the session cookie from the [`CookieJar`]
    ///
    /// # Arguments
    /// `jar` - The [`CookieJar`] returned from a request.
    ///
    /// # Returns
    /// The value of a session cookie as a [`String`].
    ///
    /// # Errors
    /// May return a [`MissingAuthentication`](ApiError::MissingAuthentication) instead.
    fn extract_token(jar: &CookieJar) -> Result<String, ApiError> {
        Ok(jar
            .get(Self::get_cookie_name())
            .ok_or(ApiError::MissingAuthentication)?
            .value()
            .to_string())
    }

    /// Function that returns the claims from the [`Parts`].
    async fn get_claims(parts: &mut Parts) -> Result<Self, ApiError>;

    /// Function that retrieves a session from the database in some way.
    async fn get_session(
        db: impl PgExecutor<'_>,
        token: &str,
        options: Self::PathOptions,
    ) -> Result<Option<Self>, ApiError>;

    /// Function that creates a new session in some way.
    async fn create_session(
        db: &mut PgConnection,
        token: &str,
        expires_at: DateTime<Utc>,
        info: &Self::ManageSessionInfo,
    ) -> Result<(), ApiError>;

    /// Function that deletes a session in some way.
    async fn delete_session(
        db: impl PgExecutor<'_>,
        token: &str,
        info: &Self::ManageSessionInfo,
    ) -> Result<(), ApiError>;

    /// Function that deletes all sessions in some way.
    async fn delete_all_sessions(
        db: impl PgExecutor<'_>,
        info: &Self::ManageSessionInfo,
    ) -> Result<(), ApiError>;

    /// Function that deletes all sessions except one in some way.
    async fn delete_all_sessions_except(
        db: impl PgExecutor<'_>,
        token: &str,
        info: &Self::ManageSessionInfo,
    ) -> Result<(), ApiError>;
}
