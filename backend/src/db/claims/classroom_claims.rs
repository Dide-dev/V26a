/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::session::{ClaimsType, ClassroomClaims};
use crate::errors::ApiError;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use sqlx::{PgConnection, PgExecutor, PgPool, query};

impl ClaimsType for ClassroomClaims {
    fn get_cookie_name() -> &'static str {
        "classroom_sessions"
    }
}

impl ClassroomClaims {
    /// Used in `FromRequestParts` for `ClassroomClaims` to retrieve the `ClassroomClaims`.
    ///
    /// # Arguments
    /// `parts` - a [`&mut Parts`](Parts).
    ///
    /// # Returns
    /// Returns the [`ClassroomClaims`] if there is a non-expired session.
    ///
    /// Returns a [`MissingAuthentication`](ApiError::MissingAuthentication) if no token was found.
    ///
    /// Returns an [`InvalidSession`](ApiError::InvalidSession) if the session has expired.
    ///
    /// # Errors
    /// May also return a [`DatabaseException`](ApiError::DatabaseException).
    pub(crate) async fn get_claims(parts: &mut Parts) -> Result<ClassroomClaims, ApiError> {
        let jar = CookieJar::from_headers(&parts.headers);
        let db = parts
            .extensions
            .get::<PgPool>()
            .ok_or(ApiError::DatabaseException)?;

        // Get the classroom id from the uri
        let classroom_id = get_classroom_id_from_uri(parts.uri.path())?;

        // Extract the token from the cookie jar.
        let token = Self::extract_token(&jar)?;

        ClassroomClaims::get_session(db, &token, classroom_id)
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
    /// `classroom_id` - The id of the classroom retrieved from the api uri.
    ///
    /// # Returns
    /// Returns [`Ok(Some(ClassroomClaims))`](ClassroomClaims) containing the user id and account type.
    ///
    /// Returns [`Ok(None)`](None) if the token could not be found or has expired.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub(crate) async fn get_session(
        db: impl PgExecutor<'_>,
        token: &str,
        classroom_id: i32,
    ) -> Result<Option<ClassroomClaims>, ApiError> {
        if token.is_empty() {
            return Ok(None);
        }

        let row = query!(
            "SELECT s.classroom_id, s.id_within_classroom, u.name
             FROM classroom_tokens t
             JOIN classroom_sessions s ON t.token = s.token
             JOIN classroom_users u ON u.classroom_id = s.classroom_id AND u.id_within_classroom = s.id_within_classroom
             WHERE s.token = $1 AND s.expires_at > NOW() AND s.classroom_id = $2",
            token,
            classroom_id
        )
            .fetch_optional(db)
            .await?;

        let claims = match row {
            Some(row) => Some(ClassroomClaims {
                id: row.classroom_id,
                num: row.id_within_classroom,
                name: row.name,
            }),
            None => None,
        };

        Ok(claims)
    }

    /// Create a session for a classroom user for one classroom and insert it into the database.
    ///
    /// Workings for `ClassroomClaims`: Connects it to an existing token.
    /// Important: will overwrite an existing session for a user if the combination of
    /// token and classroom id already exist.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The existing classroom sessions token this session needs to link to.
    ///
    /// `expires_at` - The [`DateTime<Utc>`](DateTime) when this session expires.
    ///
    /// `(classroom_id, num)` - The classroom id the user wants to join and classroom user number this session belongs to.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub(crate) async fn create_session(
        db: &mut PgConnection,
        token: &str,
        expires_at: DateTime<Utc>,
        (classroom_id, num): &(i32, i32),
    ) -> Result<(), ApiError> {
        query!(
            "INSERT INTO classroom_sessions (token, classroom_id, id_within_classroom, expires_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (token, classroom_id)
             DO UPDATE SET expires_at = excluded.expires_at",
            token,
            classroom_id,
            num,
            expires_at,
        )
        .execute(db)
        .await?;

        Ok(())
    }
}

/// If the api call path contains `classrooms/{id}`, returns `id`.
/// Otherwise, returns an `ApiError`.
fn get_classroom_id_from_uri(uri: &str) -> Result<i32, ApiError> {
    let api_uri_part = "classrooms/";
    // Find the classrooms string in the api call path
    match uri.find(api_uri_part) {
        Some(index) =>
        // Check if the part after the classrooms string is parsable to an i32.
        {
            match uri[(index + api_uri_part.len())..]
                .split('/')
                .collect::<Vec<&str>>()[0]
                .parse::<i32>()
            {
                Ok(num) => Ok(num),
                Err(_) => Err(ApiError::ClassroomIdRetrievalError),
            }
        }
        None => Err(ApiError::ClassroomIdRetrievalError),
    }
}
