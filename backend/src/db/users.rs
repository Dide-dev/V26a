extern crate bcrypt;

use crate::db_structs::users::UserData;
use crate::errors::ApiError;
use bcrypt::{DEFAULT_COST, hash};

use sqlx::{PgConnection, PgExecutor, query};

/// Add a new user to the database, returning its assigned id.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `user_data` - The data assigned to the new user.
///
/// # Returns
/// The id assigned to the new user.
///
/// # Error
/// Returns [`EmailAlreadyUsed`](ApiError::EmailAlreadyUsed) if the email was already used in the database.
///
/// May return a [`HashException`](ApiError::HashException) instead.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_user(db: &mut PgConnection, user_data: UserData) -> Result<i32, ApiError> {
    // Check if email already exists. This check is not strictly necessary, as a `UNIQUE`
    // constraint is placed on the `email` field.
    // In case of two concurrent requests with same e-mail, a `ApiError::DatabaseException`
    // will be returned instead of an `ApiError::EmailAlreadyUsed`.
    let exists = user_exists(&mut *db, &user_data.email).await?;

    if exists {
        return Err(ApiError::EmailAlreadyUsed);
    }

    // Hash the password using bcrypt. Anything beyond 72 bytes will be truncated.
    // Put a limit on password length.
    let hashed_password =
        hash(user_data.password, DEFAULT_COST).map_err(|_| ApiError::HashException)?;

    // Insert user into database.
    let user_id = query!(
        "INSERT INTO users (email, password_hash, name, nickname, about_me, email_visible, language)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id",
        user_data.email,
        hashed_password,
        user_data.name,
        user_data.nickname,
        user_data.about_me,
        user_data.email_visible,
        user_data.language.to_string(),
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .id;

    Ok(user_id)
}

/// Checks if a user with a given e-mail exists in the given database.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `email` - A reference to the email of the user being checked.
///
/// # Returns
/// `true` if the user exists or `false` if they do not.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn user_exists<'a, E: PgExecutor<'a>>(db: E, email: &String) -> Result<bool, ApiError> {
    query!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)",
        email,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .exists
    .ok_or(ApiError::DatabaseException)
}
