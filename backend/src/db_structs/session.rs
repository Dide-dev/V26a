/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
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
