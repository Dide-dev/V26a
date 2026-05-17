use crate::api_v1::session::Claims;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use chrono::{DateTime, Utc};
use sqlx::{PgExecutor, query};
use std::str::FromStr;

/// Fetch the claims associated with a session token.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `token` - The opaque session token from the session cookie.
///
/// # Returns
/// Returns `Ok(None)` if the token could not be found or has expired.
///
/// Returns [`Claims`] containing the user id and account type otherwise.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_session(db: impl PgExecutor<'_>, token: &str) -> Result<Option<Claims>, ApiError> {
    let row = query!(
        "SELECT s.user_id, u.account_type
         FROM session s
         JOIN users u ON u.id = s.user_id
         WHERE s.token = $1 AND s.expires_at > NOW()",
        token
    )
    .fetch_optional(db)
    .await;

    let Ok(Some(row)) = row else {
        return Ok(None);
    };

    Ok(Some(Claims {
        sub: row.user_id,
        account_type: AccountType::from_str(&row.account_type)?,
    }))
}

/// Insert a new session into the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `token` - The opaque session token to store.
///
/// `user_id` - The user this session belongs to.
///
/// `expires_at` - When the session expires.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn create_session(
    db: impl PgExecutor<'_>,
    token: &str,
    user_id: i32,
    expires_at: DateTime<Utc>,
) -> Result<(), ApiError> {
    query!(
        "INSERT INTO session (token, user_id, expires_at) VALUES ($1, $2, $3)",
        token,
        user_id,
        expires_at,
    )
    .execute(db)
    .await?;

    Ok(())
}