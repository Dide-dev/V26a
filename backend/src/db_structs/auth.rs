/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use serde::{Deserialize, Serialize};

/// The body for a login request that contains the email address and password.
#[derive(Deserialize, Serialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}
