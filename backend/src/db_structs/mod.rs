/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::errors::ApiError;
use crate::limits::{EMAIL_MAX_LENGTH, EMAIL_MIN_LENGTH, PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH};

pub mod auth;
pub mod cards;
pub mod classrooms;
pub mod feedback;
pub mod pagination;
pub mod session;
pub mod trees;
pub mod users;

/// A trait that can be implemented on structs that have inputs that need to be validated.
pub trait Validated {
    /// Validates the inputs for a struct, returning [`Ok`] if valid, or an [`ApiError`] if not.
    fn validate_inputs(&self) -> Result<(), ApiError>;
}

/// Validates the given password. This function is not meant for legacy passwords and should not be used in login validation.
///
/// # Returns
///  [`Ok`] if the password meets the requirements.
///
/// # Errors
/// Returns an [`InputLengthExceeded`](ApiError::InputLengthExceeded) if the password exceeds the maximum password length.
///
/// Returns an [`InputLengthNotMet`](ApiError::InputLengthNotMet) if the password does not meet the minimum password length.
///
/// Returns an [`IllegalPassword`](ApiError::IllegalPassword) if the password does not meet the minimum security requirements.
pub fn validate_password(password: &str, field_name: &'static str) -> Result<(), ApiError> {
    if password.chars().count() > PASSWORD_MAX_LENGTH {
        return Err(ApiError::InputLengthExceeded(
            field_name,
            PASSWORD_MAX_LENGTH,
        ));
    }
    if password.chars().count() < PASSWORD_MIN_LENGTH {
        return Err(ApiError::InputLengthNotMet(field_name, PASSWORD_MIN_LENGTH));
    }
    let mut lower = false;
    let mut upper = false;
    let mut special = false;
    let mut number = false;
    for char in password.chars() {
        if char.is_lowercase() {
            lower = true;
        }
        if char.is_uppercase() {
            upper = true;
        }
        if char.is_ascii_punctuation() {
            special = true;
        }
        if char.is_ascii_digit() {
            number = true;
        }

        if lower && upper && special && number {
            break;
        }
    }

    if !(lower && upper && special && number) {
        return Err(ApiError::IllegalPassword);
    }

    Ok(())
}

/// Validates the given email address.
///
/// # Returns
///  [`Ok`] if the email address meets the requirements.
///
/// # Errors
/// Returns an [`InputLengthExceeded`](ApiError::InputLengthExceeded) if the password exceeds the maximum password length.
///
/// Returns an [`InputLengthNotMet`](ApiError::InputLengthNotMet) if the password does not meet the minimum password length.
///
/// Returns an [`IllegalEmailAddress`](ApiError::IllegalEmailAddress) if the email address does not have at least 1 '@'-symbol with at least 1 character in front and behind it.
pub fn validate_email(email: &str) -> Result<(), ApiError> {
    if email.chars().count() > EMAIL_MAX_LENGTH {
        return Err(ApiError::InputLengthExceeded("email", EMAIL_MAX_LENGTH));
    }
    if email.chars().count() < EMAIL_MIN_LENGTH {
        return Err(ApiError::InputLengthNotMet("email", EMAIL_MIN_LENGTH));
    }
    if email.split_terminator('@').count() < 2 {
        return Err(ApiError::IllegalEmailAddress);
    }

    Ok(())
}
