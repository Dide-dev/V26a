/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::users::AccountType;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The claims specify the user, expiration time and account type of the request's sender.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    /// The subject (whom the token refers to).
    pub sub: i32,
    /// The expiration time (as UTC timestamp).
    pub exp: i64,
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

/// ONLY USED TO RETRIEVE ALL CLASSROOM SESSIONS, USE `ClassroomClaims`,
/// THAT RETURNS THE CLAIMS OF JUST THE CLASSROOM ID PUT IN THE API-CALL `classrooms/{id}`.
///
/// All classroom claims contained in a JWT (JSON Web Token).
/// Contains lists of classroom ids, user numbers, user names,
/// session expiration timestamps and cookie expiration timestamp.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AllClassroomClaims {
    pub sessions: Vec<ClassroomClaims>,
    pub exp: i64,
}

/// The classroom claims contained in a JWT (JSON Web Token).
/// Specifies the user number, user name and the expiration time.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassroomClaims {
    /// The classroom id
    pub id: i32,
    /// The user number (unique in the classroom).
    pub num: i32,
    /// The user name (not unique).
    pub name: String,
    /// The expiration time.
    pub exp: i64,
}

/// Trait to ensure the function `get_claims` is only used for `db_structs` that implement this trait
/// (so only our cookie `db_structs`).
pub trait ClaimsType: Sized {
    fn get_cookie_name() -> &'static str;
}

impl ClaimsType for Claims {
    fn get_cookie_name() -> &'static str {
        "session"
    }
}

impl ClaimsType for AllClassroomClaims {
    fn get_cookie_name() -> &'static str {
        "classroom_sessions"
    }
}
