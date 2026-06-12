/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::{Validated, parse_email, validate_password};
use crate::errors::ApiError;
use crate::limits::{
    PASSWORD_LOGIN_MIN_LENGTH, USER_ABOUT_ME_MAX_LENGTH, USER_NAME_MAX_LENGTH,
    USER_NAME_MIN_LENGTH, USER_NICKNAME_MAX_LENGTH, USER_NICKNAME_MIN_LENGTH,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use tracing::log::info;
use utoipa::ToSchema;

/// Data the user can modify, except email and password.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub name: String,
    pub nickname: String,
    pub about_me: String,
    pub email_visible: bool,
    pub language: UserLanguage,
}

impl Display for UserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            get_display_name(self.name.clone(), self.nickname.clone())
        )
    }
}

impl Validated for UserData {
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns [`IllegalName`](ApiError::IllegalName) when the username contains an illegal character.
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

fn get_display_name(name: String, nickname: String) -> String {
    if nickname.trim().is_empty() {
        name
    } else {
        nickname
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
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns [`IllegalName`](ApiError::IllegalName) when the username contains an illegal character.
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
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
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
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns [`IllegalName`](ApiError::IllegalName) when the username contains an illegal character.
    ///
    /// Returns an [`EmailParseError`](ApiError::EmailParseError) if the email address could not be parsed by Lettre.
    ///
    /// Returns [`IllegalPassword`](ApiError::IllegalPassword) if the password does not meet the minimum security requirements.
    fn validate_inputs(&self) -> Result<(), ApiError> {
        self.data.validate_inputs()?;

        parse_email(&self.email)?;

        validate_password(&self.password, "password")?;

        Ok(())
    }
}

/// User data to store before an account is verified.
/// Stores their registration data and when the verification link should expire.
#[derive(Debug)]
pub struct PendingRegisterData {
    pub register_data: RegisterData,
    pub expires: i64,
}

/// A trait that can be implemented to remove expired data.
pub trait Clean {
    fn remove_expired(&mut self);
}

impl Clean for HashMap<String, PendingRegisterData> {
    /// Removes pending accounts for which the expiration time has passed.
    ///
    /// This function mutates the [`HashMap`] in place.
    fn remove_expired(&mut self) {
        let before_len = self.len();

        self.retain(|_, p| p.expires > Utc::now().timestamp());

        // Subtraction with overflow is impossible, as there is a lock on the Map,
        // and entries are only removed here.
        let diff = before_len - self.len();
        if diff != 0 {
            info!("Removed {diff} pending accounts from cache.");
        }
    }
}

/// Data belonging to a password change request (forgot password).
/// Stores the user ID and when the change password link should expire.
#[derive(Debug)]
pub struct PasswordResetRequestData {
    pub user_id: i32,
    pub expires: i64,
}

impl Clean for HashMap<String, PasswordResetRequestData> {
    /// Removes password change requests for which the expiration time has passed.
    ///
    /// This function mutates the [`HashMap`] in place.
    fn remove_expired(&mut self) {
        let before_len = self.len();

        self.retain(|_, p| p.expires > Utc::now().timestamp());

        // Subtraction with overflow is impossible, as there is a lock on the Map,
        // and entries are only removed here.
        let diff = before_len - self.len();
        if diff != 0 {
            info!("Removed {diff} password reset requests from cache.");
        }
    }
}

/// Data received when getting own user data from the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserGetData {
    pub email: String,
    pub data: UserData,
}

/// Data received when getting public user data from the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserData {
    pub name: String,
    pub nickname: String,
    pub about_me: String,
    pub language: UserLanguage,

    /// If email is not visible, this will be set to None.
    pub email: Option<String>,
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

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminGetUserData {
    pub created_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub updateable_data: AdminUpdateData,
}

impl Validated for AdminUpdateData {
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns [`IllegalName`](ApiError::IllegalName) when the username contains an illegal character.
    ///
    /// Returns an [`EmailParseError`](ApiError::EmailParseError) if the email address could not be parsed by Lettre.
    fn validate_inputs(&self) -> Result<(), ApiError> {
        self.data.validate_inputs()?;
        parse_email(&self.email)?;

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

/// The request body for a password reset request that contains the email to which the password change link should be sent to.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestPasswordResetRequestBody {
    pub email: String,
}

impl Validated for RequestPasswordResetRequestBody {
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns an [`EmailParseError`](ApiError::EmailParseError) if the email address could not be parsed by Lettre.
    fn validate_inputs(&self) -> Result<(), ApiError> {
        parse_email(&self.email)?;

        Ok(())
    }
}

/// The request body for a password reset that contains the password reset code and the new password.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetRequestBody {
    pub code: String,
    pub new_password: String,
}

impl Validated for PasswordResetRequestBody {
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns [`IllegalPassword`](ApiError::IllegalPassword) if the password does not meet the minimum security requirements.
    fn validate_inputs(&self) -> Result<(), ApiError> {
        validate_password(&self.new_password, "newPassword")?;

        Ok(())
    }
}

/// The body for a password change request that contains the old password, and the new password.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChangeRequestBody {
    pub old_password: String,
    pub new_password: String,
}

impl Validated for PasswordChangeRequestBody {
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns [`IllegalPassword`](ApiError::IllegalPassword) if the password does not meet the minimum security requirements.
    fn validate_inputs(&self) -> Result<(), ApiError> {
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
    /// # Errors
    /// Returns [`InputLengthExceeded`](ApiError::InputLengthExceeded) when one of the inputs exceeds a maximum.
    ///
    /// Returns [`InputLengthNotMet`](ApiError::InputLengthNotMet) when one of the inputs does not meet a minimum.
    ///
    /// Returns an [`IllegalPassword`](ApiError::IllegalPassword) if the password does not meet the minimum security requirements.
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

#[cfg(test)]
mod tests {
    use crate::db_structs::users::{Clean, PendingRegisterData, RegisterData};
    use chrono::Utc;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_remove_expired() {
        let mut pending_accounts = HashMap::<String, PendingRegisterData>::new();
        let register_data = RegisterData::test_values();

        pending_accounts.insert(
            "a".to_owned(),
            PendingRegisterData {
                register_data: register_data.clone(),
                expires: 0,
            },
        );
        pending_accounts.insert(
            "b".to_owned(),
            PendingRegisterData {
                register_data: register_data.clone(),
                expires: Utc::now().timestamp() + 60,
            },
        );

        pending_accounts.remove_expired();

        assert!(!pending_accounts.contains_key("a"));
        assert!(pending_accounts.contains_key("b"));
    }

    #[test]
    fn test_get_display_name() {
        assert_eq!(
            super::get_display_name(String::new(), "Peter".to_owned()),
            "Peter"
        );
        assert_eq!(
            super::get_display_name("Lorax".to_owned(), " ".to_owned()),
            "Lorax"
        );
        assert_eq!(
            super::get_display_name("Lorax".to_owned(), "Santa_Claus".to_owned()),
            "Santa_Claus"
        );
    }
}
