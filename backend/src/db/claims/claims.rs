/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::session::{Claims, ClaimsType};
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use sqlx::{PgConnection, PgExecutor, PgPool, query};
use std::str::FromStr;

impl ClaimsType for Claims {
    fn get_cookie_name() -> &'static str {
        "session"
    }
}

impl Claims {
    /// Used in `FromRequestParts` for `Claims` to retrieve the `Claims`.
    ///
    /// # Arguments
    /// `parts` - a [`&mut Parts`](Parts).
    ///
    /// # Returns
    /// Returns the [`Claims`] if there is a non-expired session.
    ///
    /// Returns a [`MissingAuthentication`](ApiError::MissingAuthentication) if no token was found.
    ///
    /// Returns an [`InvalidSession`](ApiError::InvalidSession) if the session has expired.
    ///
    /// # Errors
    /// May also return a [`DatabaseException`](ApiError::DatabaseException).
    pub async fn get_claims(parts: &mut Parts) -> Result<Claims, ApiError> {
        let jar = CookieJar::from_headers(&parts.headers);
        let db = parts
            .extensions
            .get::<PgPool>()
            .ok_or(ApiError::DatabaseException)?;

        // Extract the token from the cookie jar.
        let token = Claims::extract_token(&jar)?;

        Claims::get_session(db, &token)
            .await?
            .ok_or(ApiError::InvalidSession)
    }

    /// Fetch the claims associated with a session token.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The opaque session token from the session cookie.
    ///
    /// # Returns
    /// Returns [`Ok(Some(Claims))`](Claims) containing the user id and account type.
    ///
    /// Returns [`Ok(None)`](None) if the token could not be found or has expired.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn get_session(
        db: impl PgExecutor<'_>,
        token: &str,
    ) -> Result<Option<Claims>, ApiError> {
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
            Some(row) => Some(Claims {
                sub: row.user_id,
                account_type: AccountType::from_str(&row.account_type)?,
            }),
            None => None,
        };

        Ok(claims)
    }

    /// Create a new session for a user and insert it into the database.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - A new session token.
    ///
    /// `expires_at` - The [`DateTime<Utc>`](DateTime) when this session expires.
    ///
    /// `user_id` - The user this session belongs to.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn create_session(
        db: &mut PgConnection,
        token: &str,
        expires_at: DateTime<Utc>,
        user_id: &i32,
    ) -> Result<(), ApiError> {
        query!(
            "INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3)",
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
    /// `token` - The token of the session to delete.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn delete_session(db: impl PgExecutor<'_>, token: &str) -> Result<(), ApiError> {
        query!("DELETE FROM sessions WHERE token = $1", token)
            .execute(db)
            .await?;

        Ok(())
    }

    /// Delete all sessions of a user from the database.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `user_id` - The user whose sessions will be deleted.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn delete_all_sessions(
        db: impl PgExecutor<'_>,
        user_id: &i32,
    ) -> Result<(), ApiError> {
        query!("DELETE FROM sessions WHERE user_id = $1", user_id)
            .execute(db)
            .await?;

        Ok(())
    }

    /// Delete all sessions of a user from the database except one.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The token of the session that should remain.
    ///
    /// `user_id` - The user whose sessions will be deleted.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn delete_all_sessions_except(
        db: impl PgExecutor<'_>,
        token: &str,
        user_id: &i32,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_v1::create_token;
    use crate::db::test::initialize_basic_user;
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_get_session(db: PgPool) {
        let claims = initialize_basic_user(&db).await;

        let (token, expires_at) = create_token(3600);
        Claims::create_session(get_connection_unwrap!(db), &token, expires_at, &claims.sub)
            .await
            .unwrap();

        let retrieved_claims = Claims::get_session(&db, &token).await.unwrap().unwrap();

        // Validates if the user is in claims, meaning the session is returned if it is true.
        assert_eq!(retrieved_claims, claims);
    }

    #[sqlx::test]
    async fn test_create_session(db: PgPool) {
        let claims = initialize_basic_user(&db).await;

        let (token, expires_at) = create_token(3600);
        Claims::create_session(get_connection_unwrap!(db), &token, expires_at, &claims.sub)
            .await
            .unwrap();

        let row = query!(
            "SELECT token, user_id, expires_at FROM sessions WHERE token = $1",
            token
        )
        .fetch_one(&db)
        .await
        .unwrap();

        // Checks if the corresponding token and userid are present in the rows of the session table.
        assert_eq!(row.user_id, claims.sub);
    }

    #[sqlx::test]
    async fn test_delete_session(db: PgPool) {
        let claims = initialize_basic_user(&db).await;

        let (token, expires_at) = create_token(3600);
        Claims::create_session(get_connection_unwrap!(db), &token, expires_at, &claims.sub)
            .await
            .unwrap();

        Claims::delete_session(&db, &token).await.unwrap();

        let claims: Option<Claims> = Claims::get_session(&db, &token).await.unwrap();

        assert!(claims.is_none());
    }

    #[sqlx::test]
    async fn test_delete_all_sessions_user(db: PgPool) {
        let claims = initialize_basic_user(&db).await;

        let (token, expires_at) = create_token(3600);
        Claims::create_session(get_connection_unwrap!(db), &token, expires_at, &claims.sub)
            .await
            .unwrap();
        let (token, expires_at) = create_token(3600);
        Claims::create_session(get_connection_unwrap!(db), &token, expires_at, &claims.sub)
            .await
            .unwrap();

        Claims::delete_all_sessions(&db, &claims.sub).await.unwrap();

        let claims1 = Claims::get_session(&db, "token-1").await.unwrap();
        let claims2 = Claims::get_session(&db, "token-2").await.unwrap();

        // Deleting multiple claims would result in multiple rows being empty.
        assert!(claims1.is_none());
        assert!(claims2.is_none());
    }

    #[sqlx::test]
    async fn test_delete_all_sessions_except(db: PgPool) {
        let claims = initialize_basic_user(&db).await;

        let (token1, expires_at) = create_token(3600);
        Claims::create_session(get_connection_unwrap!(db), &token1, expires_at, &claims.sub)
            .await
            .unwrap();
        let (token2, expires_at) = create_token(3600);
        Claims::create_session(get_connection_unwrap!(db), &token2, expires_at, &claims.sub)
            .await
            .unwrap();

        Claims::delete_all_sessions_except(&db, &token1, &claims.sub)
            .await
            .unwrap();

        let claims1 = Claims::get_session(&db, &token1).await.unwrap();
        let claims2 = Claims::get_session(&db, &token2).await.unwrap();

        // This session is the exception and should not be deleted.
        assert!(claims1.is_some());

        // This session should be deleted.
        assert!(claims2.is_none());
    }
}
