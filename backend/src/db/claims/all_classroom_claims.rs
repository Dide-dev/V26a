/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::session::{AllClassroomClaims, ClaimsType, ClassroomClaims};
use crate::errors::ApiError;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use sqlx::{PgConnection, PgExecutor, PgPool, query};

impl ClaimsType for AllClassroomClaims {
    type PathOptions = ();

    type ManageSessionInfo = Vec<ClassroomClaims>;

    fn get_cookie_name() -> &'static str {
        "classroom_sessions"
    }

    /// Used in `FromRequestParts` for `AllClassroomClaims` to retrieve the `AllClassroomClaims`.
    ///
    /// # Arguments
    /// `parts` - a [`&mut Parts`](Parts).
    ///
    /// # Returns
    /// Returns the [`AllClassroomClaims`] if there is a non-expired session.
    ///
    /// Returns a [`MissingAuthentication`](ApiError::MissingAuthentication) if no token was found.
    ///
    /// Returns an [`InvalidSession`](ApiError::InvalidSession) if the session has expired.
    ///
    /// # Errors
    /// May also return a [`DatabaseException`](ApiError::DatabaseException).
    async fn get_claims(parts: &mut Parts) -> Result<AllClassroomClaims, ApiError> {
        let jar = CookieJar::from_headers(&parts.headers);
        let db = parts
            .extensions
            .get::<PgPool>()
            .ok_or(ApiError::DatabaseException)?;

        // Extract the token from the cookie jar.
        let token = AllClassroomClaims::extract_token(&jar)?;

        AllClassroomClaims::get_session(db, &token, ())
            .await?
            .ok_or(ApiError::InvalidSession)
    }

    /// Fetch the claims associated with a classroom sessions token.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The opaque session token from the session cookie.
    ///
    /// `()` - A type used when extra information is needed, not necessary for `AllClassroomClaims`.
    ///
    /// # Returns
    /// Returns [`Ok(Some(AllClassroomClaims))`](AllClassroomClaims) containing the user id and account type.
    ///
    /// Returns [`Ok(None)`](None) if the token could not be found or has expired.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    async fn get_session(
        db: impl PgExecutor<'_>,
        token: &str,
        (): Self::PathOptions,
    ) -> Result<Option<AllClassroomClaims>, ApiError> {
        let rows = query!(
            "SELECT s.classroom_id, s.id_within_classroom, u.name
             FROM classroom_tokens t
             JOIN classroom_sessions s ON t.token = s.token
             JOIN classroom_users u ON u.classroom_id = s.classroom_id AND u.id_within_classroom = s.id_within_classroom
             WHERE t.token = $1 AND s.expires_at > NOW()",
            token
        )
        .fetch_all(db)
        .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        Ok(Some(AllClassroomClaims {
            claims: rows
                .iter()
                .map(|row| ClassroomClaims {
                    id: row.classroom_id,
                    num: row.id_within_classroom,
                    name: row.name.clone(),
                })
                .collect::<Vec<ClassroomClaims>>(),
        }))
    }

    /// Create a session for a classroom user for one classroom and insert it into the database.
    ///
    /// Workings for `AllClassroomClaims`: Takes a new token and inserts it into the token database.
    /// Then creates a session for every `ClassroomClaims`.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The existing classroom sessions token this session needs to link to.
    ///
    /// `expires_at` - The [`DateTime<Utc>`](DateTime) when this session expires.
    ///
    /// `all_claims` - The collection of classroom sessions to create.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    async fn create_session(
        db: &mut PgConnection,
        token: &str,
        expires_at: DateTime<Utc>,
        all_claims: &Self::ManageSessionInfo,
    ) -> Result<(), ApiError> {
        query!("INSERT INTO classroom_tokens (token) VALUES ($1)", token,)
            .execute(&mut *db)
            .await?;

        for claims in all_claims {
            ClassroomClaims::create_session(&mut *db, token, expires_at, &(claims.id, claims.num))
                .await?;
        }

        Ok(())
    }

    /// Delete all classroom session connected to a token from the database.
    ///
    /// Workings for `AllClassroomClaims`: Deletes the token, thereby deleting all sessions connected to it.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The token of the session to delete.
    ///
    /// `_all_claims` - The collection of all claims, not used in this function for `AllClassroomClaims` as the database does this for us.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    async fn delete_session(
        db: impl PgExecutor<'_>,
        token: &str,
        _all_claims: &Self::ManageSessionInfo,
    ) -> Result<(), ApiError> {
        query!("DELETE FROM classroom_tokens WHERE token = $1", token)
            .execute(db)
            .await?;

        Ok(())
    }

    /// Delete all tokens and sessions for a collection of classroom ids and user numbers from the database.
    ///
    /// Workings for `AllClassroomClaims`: For every tuple of classroom ids and user numbers,
    /// gets all tokens connected to it and invalidates those tokens.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `all_claims` - The collection of all claims of which all tokens connected to it will be deleted.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    async fn delete_all_sessions(
        db: impl PgExecutor<'_>,
        all_claims: &Self::ManageSessionInfo,
    ) -> Result<(), ApiError> {
        let ids: Vec<i32> = all_claims.iter().map(|claims| claims.id).collect();
        let nums: Vec<i32> = all_claims.iter().map(|claims| claims.id).collect();

        query!(
            "DELETE FROM classroom_tokens WHERE token IN (
                    SELECT token FROM classroom_sessions
                    WHERE (classroom_id, id_within_classroom) IN (
                        SELECT * FROM unnest($1::int[], $2::int[])))",
            &ids as &[i32],
            &nums as &[i32],
        )
        .execute(db)
        .await?;

        Ok(())
    }

    /// Delete all sessions connected to a collection of classroom id and user number except for one.
    ///
    /// Workings for `AllClassroomClaims`: For a collection of tuples of classroom ids and user numbers,
    /// deletes all tokens connected to those tuples except for the token provided.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The token of the session that should remain.
    ///
    /// `(classroom_id, num)` - The user whose sessions will be deleted.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    async fn delete_all_sessions_except(
        db: impl PgExecutor<'_>,
        token: &str,
        all_claims: &Self::ManageSessionInfo,
    ) -> Result<(), ApiError> {
        let ids: Vec<i32> = all_claims.iter().map(|claims| claims.id).collect();
        let nums: Vec<i32> = all_claims.iter().map(|claims| claims.id).collect();

        query!(
            "DELETE FROM classroom_tokens WHERE token != $1 AND token IN (
                    SELECT token FROM classroom_sessions
                    WHERE (classroom_id, id_within_classroom) IN (
                        SELECT * FROM unnest($2::int[], $3::int[])))",
            token,
            &ids as &[i32],
            &nums as &[i32],
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
