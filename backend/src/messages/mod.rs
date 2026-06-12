/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::users::UserLanguage;

mod en;
mod nl;

pub enum Message {
    /// The subject of an e-mail sent to verify an account upon registration.
    EmailVerificationSubject,
    /// The body of an e-mail sent to verify an account upon registration.
    ///
    /// # Parameters
    /// `display_name` - The name that the receiver will be greeted as.
    ///
    /// `verification_link` - The link that the receiver can use to verify.
    ///
    /// `base_url` - The base URL of the website, to show in the footer.
    EmailVerificationBody {
        display_name: String,
        verification_link: String,
        duration_seconds: i64,
        base_url: String,
    },
    /// The subject of an e-mail sent to reset a user's password.
    PasswordResetSubject,
    /// The body of an e-mail sent to reset a user's password.
    ///
    /// # Parameters
    /// `display_name` - The name that the receiver will be greeted as.
    ///
    /// `password_reset_link` - The link that the receiver can use to reset their password.
    ///
    /// `base_url` - The base URL of the website, to show in the footer.
    PasswordResetBody {
        display_name: String,
        password_reset_link: String,
        duration_seconds: i64,
        base_url: String,
    },
    /// A text to be used as prefix in a copy of something.
    CopyOf,
}

impl Message {
    pub fn localized(&self, language: &UserLanguage) -> String {
        match language {
            UserLanguage::Nl => nl::localize_message(self),
            UserLanguage::En => en::localize_message(self),
        }
    }
}
