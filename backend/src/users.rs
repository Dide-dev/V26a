#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserData {
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub nickname: String,
    pub about_me: String,
    pub email_visible: bool,
    pub language: String,
}
