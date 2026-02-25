/// Data needed to add a user to the database
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub email: String,
    pub password: String,
    pub name: String,
    pub nickname: Option<String>,
    pub about_me: Option<String>,
    pub email_visible: bool,
    pub language: String,
}
