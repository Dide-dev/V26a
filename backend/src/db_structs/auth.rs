use serde::{Deserialize, Serialize};

/// The body for a login request that contains the email address and password.
#[derive(Deserialize, Serialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}
