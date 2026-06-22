/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use crate::structs::users::AccountType;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utoipa::ToSchema;

/// The session specifies the user and account type of the request's sender.
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// The `user_id` belonging to the session.
    pub user_id: i32,
    /// The account type
    pub account_type: AccountType,
}

/// User cookie info
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InfoCookieData {
    pub name: String,
    pub nickname: String,
    pub user_id: i32,
    pub account_type: AccountType,
}

/// The classroom session for one classroom.
/// Specifies the user number and username.
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomSession {
    /// The classroom id
    pub id: i32,
    /// The user number (unique in the classroom).
    pub num: i32,
    /// The user name (not unique).
    pub name: String,
}

/// The classroom sessions for all classrooms the user has logged in to.
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllClassroomSessions {
    /// The collection of [`ClassroomSession`]s for a user.
    pub sessions: Vec<ClassroomSession>,
}

/// Trait to ensure these function are only used for structs that implement this trait
/// (so only our cookie structs).
pub trait SessionType: Sized + Debug {
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
}
