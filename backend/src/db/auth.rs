use crate::errors::ApiError;
use sqlx::{PgExecutor, query};

/// Fetch a user's password from the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `email` - The user's email address.
///
/// # Returns
/// A tuple containing the user id and the password hash.
///
/// # Error
/// Returns [`WrongCredentials`](ApiError::WrongCredentials) if no user could be found matching the given e-mail.
pub async fn get_user_id_and_password(
    db: impl PgExecutor<'_>,
    email: String,
) -> Result<(i32, String), ApiError> {
    let record = query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::WrongCredentials)?;

    let user_id = record.id;
    let password_hash = record.password_hash;

    Ok((user_id, password_hash))
}
