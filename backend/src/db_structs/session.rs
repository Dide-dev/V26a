/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::users::AccountType;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A struct containing the [`EncodingKey`] and [`DecodingKey`] used for encoding and decoding the
/// `JsonWebToken`.
#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    /// Creates a [`Keys`] from an array of bytes.
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// The claims contained in a JWT (JSON Web Token). Specifies the subject (user id) and the
/// expiration time.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    /// The subject (whom the token refers to).
    pub sub: i32,
    /// The expiration time (as UTC timestamp).
    pub exp: i64,
    /// the account type
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
