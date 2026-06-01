/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::users::AccountType;
use crate::db_structs::{Validated, validate_email};
use crate::errors::ApiError;
use crate::limits::{PASSWORD_LOGIN_MIN_LENGTH, PASSWORD_MAX_LENGTH};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The body for a login request that contains the email address and password.
#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}

impl Validated for LoginRequestBody {
    fn validate_inputs(&self) -> Result<(), ApiError> {
        validate_email(&self.email)?;

        if self.password.chars().count() > PASSWORD_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "password",
                PASSWORD_MAX_LENGTH,
            ));
        }

        if self.password.chars().count() < PASSWORD_LOGIN_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet(
                "password",
                PASSWORD_LOGIN_MIN_LENGTH,
            ));
        }

        Ok(())
    }
}

/// User information to check password and permissions.
#[derive(Deserialize, Serialize, Debug)]
pub struct UserCredentials {
    pub user_id: i32,
    pub password_hash: String,
    pub account_type: AccountType,
}
