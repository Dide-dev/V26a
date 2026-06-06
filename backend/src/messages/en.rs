/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::messages::Message;

pub fn localize_message(message: &Message) -> String {
    match message {
        Message::EmailVerificationSubject => "BOOM! Account verification".to_owned(),
        Message::EmailVerificationBody(nickname, base_url, verification_link) => format!(
            "Hello {nickname},\n\nThank you for registering an account at BOOM!\nBefore you can use your account, please verify using the following link:\n{verification_link}\n\nIf you do not recognize this request, please ignore this e-mail.\n\nBOOM!\n{base_url}"
        ),
        Message::CopyOf => "Copy of".to_owned(),
    }
}
