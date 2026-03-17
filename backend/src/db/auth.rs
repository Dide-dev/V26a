use crate::errors::ApiError;
use sqlx::{PgExecutor, query};

/// Fetch a user's password from the given database.
///
/// # Arguments
/// `db` - a reference to the database connection to use for the query.
///
/// `email` - the user's email address.
///
/// # Returns
/// A tuple containing the user id and the password hash.
///
/// # Error
/// May return an [`ApiError::WrongCredentials`] instead. TODO: clarify when this error occurs.
pub async fn get_user_id_and_password<'a, E: PgExecutor<'a>>(
    db: E,
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
