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
pub mod mail;
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
    if email.split_terminator('@').count() < 2 || email.starts_with('@') {
        return Err(ApiError::IllegalEmailAddress);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_structs::users::{RegisterData, UserData, UserLanguage};
    #[test]
    fn test_validate_password() {
        // The default values are valid.
        let valid_data = RegisterData::test_values();
        let response = valid_data.validate_inputs();
        assert!(response.is_ok());

        // Register with a password that is too short.
        let mut invalid_data = RegisterData {
            email: "valid@email.com".to_owned(),
            password: "no_good".to_owned(),
            data: UserData {
                name: "Jantje_Deeg".to_owned(),
                nickname: "Jantje junior Deeg de tweede".to_owned(),
                about_me: "Ik ben Jantje \"jr\" Deeg de 2e, niet te verwarren met de 3e".to_owned(),
                email_visible: true,
                language: UserLanguage::Nl,
            },
        };
        let response = invalid_data.validate_inputs();
        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            ApiError::InputLengthNotMet("password", PASSWORD_MIN_LENGTH)
        );

        // Register with a password that does not meet the security requirements.
        invalid_data.password = "Grabbelton123".to_owned();
        let response = invalid_data.validate_inputs();
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), ApiError::IllegalPassword);

        // Register with a password that is too long.
        invalid_data.password =
            "Grabbelton123Grabbelton123Grabbelton123Grabbelton123Grabbelton123Grabbelton123"
                .to_owned();
        let response = invalid_data.validate_inputs();
        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            ApiError::InputLengthExceeded("password", PASSWORD_MAX_LENGTH)
        );
    }

    #[test]
    fn test_validate_email() {
        // The default values are valid.
        let valid_data = RegisterData::test_values();
        let response = valid_data.validate_inputs();
        assert!(response.is_ok());

        // Register with an email address that is too long.
        let mut invalid_data = RegisterData {
            email: "Grabbelton123Grabbelton123Grabbelton123Grabbelton123Grabbelton123Grabbelton123Grabbelton123Grabbelton123Grabbelton123@grabbelton.com".to_owned(),
            password: "A$$word123".to_owned(),
            data: UserData {
                name: "Jantje_Deeg".to_owned(),
                nickname: "Jantje junior Deeg de derde".to_owned(),
                about_me: "Ik ben Jantje \"jr\" Deeg de 3e, niet te verwarren met de 2e".to_owned(),
                email_visible: true,
                language: UserLanguage::Nl,
            }
        };
        let response = invalid_data.validate_inputs();
        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            ApiError::InputLengthExceeded("email", EMAIL_MAX_LENGTH)
        );

        // Register with an email address that is too short.
        invalid_data.email = "a@".to_owned();
        let response = invalid_data.validate_inputs();
        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            ApiError::InputLengthNotMet("email", EMAIL_MIN_LENGTH)
        );

        // Register with an email address that is invalid.
        invalid_data.email = "brood@".to_owned();
        let response = invalid_data.validate_inputs();
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), ApiError::IllegalEmailAddress);

        // Register with an email address that is invalid.
        invalid_data.email = "@brood".to_owned();
        let response = invalid_data.validate_inputs();
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), ApiError::IllegalEmailAddress);
    }
}
