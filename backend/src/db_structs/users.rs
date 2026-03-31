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
pub struct UserData {
    pub email: String,
    pub name: String,
    pub nickname: Option<String>,
    pub about_me: Option<String>,
    pub email_visible: bool,
    pub language: UserLanguage,
}

/// The possible values of the `language` field of a user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UserLanguage {
    Nl,
    En,
}

impl UserLanguage {
    pub fn try_parse(s: &str) -> Option<Self> {
        match s {
            "nl" => Some(Self::Nl),
            "en" => Some(Self::En),
            _ => None,
        }
    }
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
