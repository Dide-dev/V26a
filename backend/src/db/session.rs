use crate::db_structs::session::Claims;
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
        "SELECT s.user_id, u.account_type, s.expires_at AS exp
         FROM session s
         JOIN users u ON u.id = s.user_id
         WHERE s.token = $1 AND s.expires_at > NOW()",
        token
    )
    .fetch_optional(db)
    .await?;

    let claims = match row {
        Some(r) => Some(Claims {
            sub: r.user_id,
            exp: r.exp.timestamp(),
            account_type: AccountType::from_str(&r.account_type)?,
        }),
        None => None,
    };

    Ok(claims)
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

/// Delete a session from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `token` - The opaque session token to delete.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn delete_session(db: impl PgExecutor<'_>, token: &str) -> Result<(), ApiError> {
    query!("DELETE FROM session WHERE token = $1", token,)
        .execute(db)
        .await?;

    Ok(())
}

/// Delete all sessions of an user from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `user_id` - The user this session belongs to.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn delete_all_sessions(db: impl PgExecutor<'_>, user_id: i32) -> Result<(), ApiError> {
    query!("DELETE FROM session WHERE user_id = $1", user_id,)
        .execute(db)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test::initialize_basic_user;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_get_session(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;
        let expires_at = Utc::now() + chrono::Duration::hours(1);
        let token = "donut-token";

        create_session(&db, token, user_id, expires_at)
            .await
            .unwrap();

        let claims = get_session(&db, token).await.unwrap().unwrap();

        // Validates if the user is in claims, meaning the session is returned if it is true.
        assert_eq!(claims.sub, user_id);
    }

    #[sqlx::test]
    async fn test_create_session(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;
        let expires_at = Utc::now() + chrono::Duration::hours(1);
        let token = "test-token";

        create_session(&db, token, user_id, expires_at)
            .await
            .unwrap();

        let row = query!(
            "SELECT token, user_id, expires_at FROM session WHERE token = $1",
            token
        )
        .fetch_one(&db)
        .await
        .unwrap();

        // Checks if the corresponding token and userid are present in the rows of the session table.
        assert_eq!(row.user_id, user_id);
    }

    #[sqlx::test]
    async fn test_delete_session(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;
        let expires_at = Utc::now() + chrono::Duration::hours(1);
        let token = "test-token";

        create_session(&db, token, user_id, expires_at)
            .await
            .unwrap();
        delete_session(&db, token).await.unwrap();

        let row = get_session(&db, token).await.unwrap();

        // Deleting the session is equal to deleting a row of session table. So no row should be present.
        assert!(row.is_none());
    }

    #[sqlx::test]
    async fn test_delete_all_sessions(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;
        let expires_at = Utc::now() + chrono::Duration::hours(1);

        create_session(&db, "token-1", user_id, expires_at)
            .await
            .unwrap();
        create_session(&db, "token-2", user_id, expires_at)
            .await
            .unwrap();

        delete_all_sessions(&db, user_id).await.unwrap();

        let rows1 = get_session(&db, "token-1").await.unwrap();
        let rows2 = get_session(&db, "token-2").await.unwrap();

        // Deleting multiple sessions would result in multiple rows being empty.
        assert!(rows1.is_none());
        assert!(rows2.is_none());
    }
}
