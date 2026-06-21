/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::messages::Message;

pub fn localize_message(message: &Message) -> String {
    match message {
        Message::EmailVerificationSubject => "BOOM! Account verification".to_owned(),
        Message::EmailVerificationBody {
            display_name,
            verification_link,
            duration_seconds,
            base_url,
        } => {
            let minutes = duration_seconds / 60;
            format!(
                "Hello {display_name},\n\nThank you for registering an account at BOOM!\nBefore you can use your account, please verify using the following link:\n{verification_link}\nThis link will be valid for {minutes} minutes.\n\nIf you do not recognize this request, please ignore this e-mail.\n\nBOOM!\n{base_url}"
            )
        }
        Message::CopyOf => "Copy of".to_owned(),
        Message::PasswordResetMailSubject => "BOOM! Password reset".to_owned(),
        Message::PasswordResetMailBody {
            display_name,
            password_reset_link,
            duration_seconds,
            base_url,
        } => {
            let minutes = duration_seconds / 60;
            format!(
                "Hello {display_name},\n\nYou are receiving this e-mail because a password reset has been requested for your account.\n\nPlease use the following link to reset your password:\n{password_reset_link}\nThis link will be valid for {minutes} minutes.\n\nIf you do not recognize this request, you can safely ignore this e-mail.\n\nBOOM!\n{base_url}"
            )
        }
    }
}
