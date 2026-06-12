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
            duration_seconds,
            base_url,
        } => {
            let minutes = duration_seconds / 60;
            format!(
                "Hallo {display_name},\n\nBedankt voor het registreren van een account bij BOOM!\nVoordat u uw account kunt gebruiken, dient u zich te verifiëren met de volgende link:\n{verification_link}\nDeze link is geldig voor {minutes} minuten.\n\nAls u dit verzoek niet herkent, kunt u deze e-mail gerust negeren.\n\nBOOM!\n{base_url}"
            )
        },
        Message::CopyOf => "Kopie van".to_owned(),
        Message::PasswordResetSubject => "BOOM! Wachtwoord Resetten".to_owned(),
        Message::PasswordResetBody {
            display_name,
            password_reset_link,
            duration_seconds,
            base_url,
        } => {
            let minutes = duration_seconds / 60;
            format!(
                "Beste {display_name},\n\nJe ontvangt deze e-mail omdat er een wachtwoord reset is opgevraagd voor jouw account.\n\nJe kunt de volgende link gebruiken om je wachtwoord opnieuw in te stellen:\n{password_reset_link}\nDeze link is geldig voor {minutes} minuten.\n\nAls je dit verzoek niet herkent, kan je deze e-mail veilig negeren.\n\nBOOM!\n{base_url}"
            )
        },
    }
}
