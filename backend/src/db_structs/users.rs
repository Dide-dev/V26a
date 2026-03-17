use std::fmt::Display;

/// Data needed to add a user to the database.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub email: String,
    pub password: String,
    pub name: String,
    pub nickname: Option<String>,
    pub about_me: Option<String>,
    pub email_visible: bool,
    pub language: UserLanguage,
}

/// The possible values of the `language` field of a user.
#[derive(serde::Deserialize, serde::Serialize)]
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
