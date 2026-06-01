/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::{Validated, validate_email, validate_password};
use crate::errors::ApiError;
use crate::limits::{
    PASSWORD_LOGIN_MIN_LENGTH, PASSWORD_MAX_LENGTH, USER_ABOUT_ME_MAX_LENGTH, USER_NAME_MAX_LENGTH,
    USER_NAME_MIN_LENGTH, USER_NICKNAME_MAX_LENGTH, USER_NICKNAME_MIN_LENGTH,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use utoipa::ToSchema;

/// Data the user can modify, except email and password.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub name: String,
    pub nickname: String,
    pub about_me: String,
    pub email_visible: bool,
    pub language: UserLanguage,
}

impl Validated for UserData {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.name.chars().count() > USER_NAME_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded("name", USER_NAME_MAX_LENGTH));
        }
        if self.nickname.chars().count() > USER_NICKNAME_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "nickname",
                USER_NICKNAME_MAX_LENGTH,
            ));
        }
        if self.about_me.chars().count() > USER_ABOUT_ME_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "aboutMe",
                USER_ABOUT_ME_MAX_LENGTH,
            ));
        }

        if self.name.chars().count() < USER_NAME_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet("name", USER_NAME_MIN_LENGTH));
        }
        if self.nickname != String::new()
            && self.nickname.chars().count() < USER_NICKNAME_MIN_LENGTH
        {
            return Err(ApiError::InputLengthNotMet(
                "nickname",
                USER_NICKNAME_MIN_LENGTH,
            ));
        }

        for char in self.name.chars() {
            let legal = char.is_ascii_alphanumeric() || char == '_' || char == '-' || char == '.';
            if !legal {
                return Err(ApiError::IllegalName);
            }
        }

        Ok(())
    }
}

/// Data the admin can modify, except email, password, enabled and account type.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserGeneralData {
    pub name: String,
    pub nickname: String,
    pub about_me: String,
}

impl Validated for UserGeneralData {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.name.chars().count() > USER_NAME_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded("name", USER_NAME_MAX_LENGTH));
        }
        if self.nickname.chars().count() > USER_NICKNAME_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "nickname",
                USER_NICKNAME_MAX_LENGTH,
            ));
        }
        if self.about_me.chars().count() > USER_ABOUT_ME_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "aboutMe",
                USER_ABOUT_ME_MAX_LENGTH,
            ));
        }

        if self.name.chars().count() < USER_NAME_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet("name", USER_NAME_MIN_LENGTH));
        }
        if self.nickname != String::new()
            && self.nickname.chars().count() < USER_NICKNAME_MIN_LENGTH
        {
            return Err(ApiError::InputLengthNotMet(
                "nickname",
                USER_NICKNAME_MIN_LENGTH,
            ));
        }

        for char in self.name.chars() {
            let legal = char.is_ascii_alphanumeric() || char == '_' || char == '-' || char == '.';
            if !legal {
                return Err(ApiError::IllegalName);
            }
        }

        Ok(())
    }
}

/// Data needed to add a user to the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub data: UserData,
}

#[cfg(test)]
impl RegisterData {
    pub fn test_values() -> Self {
        use rand::RngExt;

        let mut rng = rand::rng();
        let n: u64 = rng.random();

        RegisterData {
            email: format!("test{n}@localhost"),
            password: "4$$wOrd!".to_owned(),
            data: UserData {
                name: format!("John_Test_{n}"),
                nickname: String::new(),
                about_me: String::new(),
                email_visible: false,
                language: UserLanguage::En,
            },
        }
    }
}

impl Validated for RegisterData {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        self.data.validate_inputs()?;

        validate_email(&self.email)?;

        validate_password(&self.password, "password")?;

        Ok(())
    }
}

/// Data received when getting user data from the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserGetData {
    pub email: String,
    pub data: UserData,
}

/// Data needed for an admin to update a user in the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdateData {
    pub email: String,
    pub data: UserGeneralData,
    pub enabled: bool,
    pub account_type: AccountType,
}

impl Validated for AdminUpdateData {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        self.data.validate_inputs()?;
        validate_email(&self.email)?;

        Ok(())
    }
}

/// Data received when getting all user data from the database.
/// Should only be used by admins
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AllUsersGetData {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub enabled: bool,
}

/// Data received when getting the data of a user from the database.
/// Should only be used by admins
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserGetAllData {
    pub data: UserData,
    pub email: String,
    pub enabled: bool,
    pub account_type: AccountType,
}

/// The data returned when requesting a list of users.
#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserListResponse {
    /// The total amount of users that are visible in the query.
    pub total_users: i32,
    pub users: Vec<AllUsersGetData>,
}

/// The body for a password change request that contains the old password, and the new password.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChangeRequestBody {
    pub old_password: String,
    pub new_password: String,
}

impl Validated for PasswordChangeRequestBody {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        if self.old_password.chars().count() > PASSWORD_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "oldPassword",
                PASSWORD_MAX_LENGTH,
            ));
        }

        if self.old_password.chars().count() < PASSWORD_LOGIN_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet(
                "oldPassword",
                PASSWORD_LOGIN_MIN_LENGTH,
            ));
        }
        validate_password(&self.new_password, "newPassword")?;

        Ok(())
    }
}

/// The body for a password change request that contains the new password.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminPasswordChangeRequestBody {
    pub new_password: String,
}

impl Validated for AdminPasswordChangeRequestBody {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        validate_password(&self.new_password, "newPassword")?;

        Ok(())
    }
}

/// Optional query parameter that makes it possible to delete trees of the user too.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOwnedTrees {
    pub delete_owned_trees: bool,
}

/// The possible values of the `language` field of a user.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
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
    /// Localize 'Copy of' string according to user language.
    pub fn to_copy_of_text(&self) -> &str {
        match &self {
            UserLanguage::Nl => "Kopie van",
            UserLanguage::En => "Copy of",
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
