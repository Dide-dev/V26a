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
/// Returns Ok(None) if the email could not be found.
///
/// Returns a struct containing the user id, password hash and account type otherwise
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user_credentials_by_email(
    db: impl PgExecutor<'_>,
    email: String,
) -> Result<Option<UserCredentials>, ApiError> {
    let credentials = query!(
        "SELECT id, password_hash, account_type FROM users WHERE email = $1",
        email.to_lowercase()
    )
    .fetch_optional(db)
    .await?;

    let Some(credentials) = credentials else {
        return Ok(None);
    };

    Ok(Some(UserCredentials {
        user_id: credentials.id,
        password_hash: credentials.password_hash,
        account_type: AccountType::from_str(&credentials.account_type)?,
    }))
}

/// Fetch a user's password from the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `id` - The user's id.
///
/// # Returns
/// Returns Ok(None) if the id could not be found.
///
/// Returns a struct containing the user id, password hash and account type otherwise.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user_credentials_by_id(
    db: impl PgExecutor<'_>,
    id: i32,
) -> Result<Option<UserCredentials>, ApiError> {
    let credentials = query!(
        "SELECT id, password_hash, account_type FROM users WHERE id = $1",
        id
    )
    .fetch_optional(db)
    .await?;

    let Some(credentials) = credentials else {
        return Ok(None);
    };

    Ok(Some(UserCredentials {
        user_id: credentials.id,
        password_hash: credentials.password_hash,
        account_type: AccountType::from_str(&credentials.account_type)?,
    }))
}
