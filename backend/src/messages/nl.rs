/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::messages::Message;

pub fn localize_message(message: &Message) -> String {
    match message {
        Message::EmailVerificationSubject => "BOOM! Accountbevestiging".to_owned(),
        Message::EmailVerificationBody {
            display_name,
            verification_link,
            base_url,
        } => format!(
            "Hallo {display_name},\n\nBedankt voor het registreren van een account bij BOOM!\nVoordat u uw account kunt gebruiken, dient u zich te verifiëren met de volgende link:\n{verification_link}\n\nAls u dit verzoek niet herkent, kunt u deze e-mail gerust negeren.\n\nBOOM!\n{base_url}"
        ),
        Message::CopyOf => "Kopie van".to_owned(),
    }
}
