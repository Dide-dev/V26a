use crate::db_structs::session::Claims;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use chrono::Utc;
use rand::distr::{Alphanumeric, SampleString};
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
/// Returns [`Ok(None)`](Ok) if the token could not be found or has expired.
///
/// Returns [`Claims`] containing the user id and account type otherwise.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_session(db: impl PgExecutor<'_>, token: &str) -> Result<Option<Claims>, ApiError> {
    let row = query!(
        "SELECT s.user_id, u.account_type, s.expires_at AS exp
         FROM sessions s
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
/// `user_id` - The user this session belongs to.
///
/// `session_duration` - The total amount of time the session persists.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn create_session(
    db: impl PgExecutor<'_>,
    user_id: i32,
    session_duration: i64,
) -> Result<String, ApiError> {
    // Create the authorization token. The opaque session token to store.
    let token = {
        let mut rng = rand::rng();
        Alphanumeric.sample_string(&mut rng, 64)
    };
    let expires_at = Utc::now() + chrono::Duration::seconds(session_duration);

    query!(
        "INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3)",
        token,
        user_id,
        expires_at,
    )
    .execute(db)
    .await?;

    Ok(token)
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
    query!("DELETE FROM sessions WHERE token = $1", token,)
        .execute(db)
        .await?;

    Ok(())
}

/// Delete all sessions of a user from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `user_id` - The user this session belongs to.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn delete_all_sessions(db: impl PgExecutor<'_>, user_id: i32) -> Result<(), ApiError> {
    query!("DELETE FROM sessions WHERE user_id = $1", user_id,)
        .execute(db)
        .await?;

    Ok(())
}

/// Delete all sessions of a user from the database except the current session of the user .
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `user_id` - The user this session belongs to.
///
/// `token` - The opaque session token.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn delete_all_sessions_except(
    db: impl PgExecutor<'_>,
    user_id: i32,
    token: &str,
) -> Result<(), ApiError> {
    query!(
        "DELETE FROM sessions WHERE user_id = $1 AND token != $2",
        user_id,
        token
    )
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

        let session = create_session(&db, user_id, 3600).await.unwrap();

        let claims = get_session(&db, &session).await.unwrap().unwrap();

        // Validates if the user is in claims, meaning the session is returned if it is true.
        assert_eq!(claims.sub, user_id);
    }

    #[sqlx::test]
    async fn test_create_session(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

        let session = create_session(&db, user_id, 3600).await.unwrap();

        let row = query!(
            "SELECT token, user_id, expires_at FROM sessions WHERE token = $1",
            session
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

        let session = create_session(&db, user_id, 3600).await.unwrap();

        delete_session(&db, &session).await.unwrap();

        let claims: Option<Claims> = get_session(&db, &session).await.unwrap();

        assert!(claims.is_none());
    }

    #[sqlx::test]
    async fn test_delete_all_sessions(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

        create_session(&db, user_id, 3600).await.unwrap();
        create_session(&db, user_id, 3600).await.unwrap();

        delete_all_sessions(&db, user_id).await.unwrap();

        let claims1 = get_session(&db, "token-1").await.unwrap();
        let claims2 = get_session(&db, "token-2").await.unwrap();

        // Deleting multiple sessions would result in multiple rows being empty.
        assert!(claims1.is_none());
        assert!(claims2.is_none());
    }

    #[sqlx::test]
    async fn test_delete_all_sessions_except(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

        let session1 = create_session(&db, user_id, 3600).await.unwrap();
        let session2 = create_session(&db, user_id, 3600).await.unwrap();

        delete_all_sessions_except(&db, user_id, &session1)
            .await
            .unwrap();

        let claims1 = get_session(&db, &session1).await.unwrap();
        let claims2 = get_session(&db, &session2).await.unwrap();

        // This session is the exception and should not be deleted.
        assert!(claims1.is_some());

        // This session should be deleted.
        assert!(claims2.is_none());
    }
}
