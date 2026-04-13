/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::db_structs::auth::UserCredentials;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use sqlx::{PgExecutor, query};
use std::str::FromStr;

/// Fetch a user's password from the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `email` - The user's email address.
///
/// # Returns
/// A struct containing the user id, password hash and account type.
///
/// # Errors
/// Returns [`WrongCredentials`](ApiError::WrongCredentials) if no user could be found matching the given e-mail.
pub async fn get_user_credentials(
    db: impl PgExecutor<'_>,
    email: String,
) -> Result<UserCredentials, ApiError> {
    let credentials = query!(
        "SELECT id, password_hash, account_type FROM users WHERE email = $1",
        email
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::WrongCredentials)?;

    Ok(UserCredentials {
        user_id: credentials.id,
        password_hash: credentials.password_hash,
        account_type: AccountType::from_str(&credentials.account_type)?,
    })
}
