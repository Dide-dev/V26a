use serde::Deserialize;

/// The body for a login request that contains the email address and password.
#[derive(Deserialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}
