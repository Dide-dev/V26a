/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::Validated;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::limits::{EMAIL_MAX_LENGTH, PASSWORD_MAX_LENGTH, EMAIL_MIN_LENGTH, PASSWORD_LOGIN_MIN_LENGTH};
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
        if self.email.chars().count() > EMAIL_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded("email", EMAIL_MAX_LENGTH));
        }
        if self.password.chars().count() > PASSWORD_MAX_LENGTH {
            return Err(ApiError::InputLengthExceeded(
                "password",
                PASSWORD_MAX_LENGTH,
            ));
        }

        if self.email.chars().count() < EMAIL_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet("email", EMAIL_MAX_LENGTH));
        }
        if self.password.chars().count() < PASSWORD_LOGIN_MIN_LENGTH {
            return Err(ApiError::InputLengthNotMet("password", PASSWORD_LOGIN_MIN_LENGTH));
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
