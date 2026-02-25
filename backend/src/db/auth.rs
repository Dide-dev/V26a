use crate::errors::ApiError;
use sqlx::{PgPool, query};

/// Fetch a user's password from the given database, returning the password hash if successful,
/// or an error if not.
///
/// # Arguments
/// `db` - a reference to the database connection to use for the query.
///
/// `email` - the user's email address.
pub async fn get_user_id_and_password(
    db: &PgPool,
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
