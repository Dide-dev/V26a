/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use std::fmt::Display;
use std::str::FromStr;

/// Data needed to add a user to the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub name: String,
    pub nickname: Option<String>,
    pub about_me: Option<String>,
    pub email_visible: bool,
    pub language: UserLanguage,
}

/// Data received when getting user data from the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGetData {
    pub email: String,
    pub name: String,
    pub nickname: Option<String>,
    pub about_me: Option<String>,
    pub email_visible: bool,
    pub language: UserLanguage,
}

/// Data needed to update a user in the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateData {
    pub id: i32,
    pub name: String,
    pub nickname: Option<String>,
    pub about_me: Option<String>,
    pub email_visible: bool,
    pub language: UserLanguage,
    pub account_type: Option<AccountType>,
}

/// The possible values of the `language` field of a user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
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

/// The possible values of the account type field of a user.
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
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
