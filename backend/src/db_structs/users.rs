/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use utoipa::ToSchema;

/// Data needed to add a user to the database.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub name: String,
    pub nickname: String,
    pub about_me: String,
    pub email_visible: bool,
    pub language: UserLanguage,
}

/// User data to store before an account is verified.
/// Stores their registration data and when the verification link should expire.
#[derive(Debug)]
pub struct PendingRegisterData {
    pub register_data: RegisterData,
    pub expires: i64,
}

/// Data received when getting user data from the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserGetData {
    pub email: String,
    pub name: String,
    pub nickname: String,
    pub about_me: String,
    pub email_visible: bool,
    pub language: UserLanguage,
}

/// Data needed to update a user in the database.
/// `account_type` and `enabled` are only updated if a `Some` value is passed.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateData {
    pub name: String,
    pub nickname: String,
    pub about_me: String,
    pub email_visible: bool,
    pub language: UserLanguage,
    /// This field may only be updated by admins.
    pub account_type: Option<AccountType>,
    /// This field may only be updated by admins.
    pub enabled: Option<bool>,
}

/// Data received when getting all user data from the database.
/// Should only be used by admins
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserGetAllData {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub enabled: bool,
}

/// The data returned when requesting a list of users.
#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserListResponse {
    /// The total amount of users that are visible in the query.
    pub total_users: i32,
    pub users: Vec<UserGetAllData>,
}

/// The body for a password change request that contains the old password, and the new password.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChangeRequestBody {
    pub old_password: String,
    pub new_password: String,
}

/// Optional query parameter that makes it possible to delete trees of the user too.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOwnedTrees {
    pub delete_owned_trees: bool,
}

/// The possible values of the `language` field of a user.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum UserLanguage {
    Nl,
    En,
}

impl Display for UserLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            UserLanguage::Nl => write!(f, "nl"),
            UserLanguage::En => write!(f, "en"),
        }
    }
}

impl FromStr for UserLanguage {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "nl" => Ok(UserLanguage::Nl),
            "en" => Ok(UserLanguage::En),
            _ => Err(ApiError::InvalidDataFound),
        }
    }
}

impl UserLanguage {
    pub fn to_verification_message_title(&self) -> &str {
        match &self {
            UserLanguage::Nl => "BOOM! Accountbevestiging",
            UserLanguage::En => "BOOM! Account verification",
        }
    }

    pub fn to_verification_message_body(
        &self,
        nickname: &str,
        verification_link: &str,
        base_url: &str,
    ) -> String {
        match &self {
            UserLanguage::Nl => {
                format!(
                    "Hallo {nickname},\n\nBedankt voor het registreren aan BOOM!\nVoordat u uw account kunt gebruiken, dient u zich te verifiëren met de volgende link:\n{verification_link}\n\nAls u dit verzoek niet herkent, kunt u deze e-mail gerust negeren.\n\nBOOM!\n{base_url}"
                )
            }
            UserLanguage::En => {
                format!(
                    "Hello {nickname},\n\nThank you for signing up to BOOM!\nBefore you can use your account, please verify using the following link:\n{verification_link}\n\nIf you do not recognize this request, please ignore this e-mail.\n\nBOOM!\n{base_url}"
                )
            }
        }
    }
}

/// The possible values of the account type field of a user.
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Admin,
    Author,
}

impl Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AccountType::Admin => write!(f, "admin"),
            AccountType::Author => write!(f, "author"),
        }
    }
}

impl FromStr for AccountType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "admin" => Ok(AccountType::Admin),
            "author" => Ok(AccountType::Author),
            _ => Err(ApiError::InvalidDataFound),
        }
    }
}
