/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use crate::structs::session::{AllClassroomSessions, ClassroomSession, SessionType};
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use sqlx::{PgConnection, PgExecutor, PgPool, query};

impl SessionType for AllClassroomSessions {
    fn get_cookie_name() -> &'static str {
        "classroom_sessions"
    }
}

impl AllClassroomSessions {
    /// Used in `FromRequestParts` for [`AllClassroomSessions`] to retrieve the [`AllClassroomSessions`].
    ///
    /// # Arguments
    /// `parts` - a [`&mut Parts`](Parts).
    ///
    /// # Returns
    /// Returns the [`AllClassroomSessions`] if there is a non-expired session.
    ///
    /// Returns a [`MissingAuthentication`](ApiError::MissingAuthentication) if no token was found.
    ///
    /// Returns an [`InvalidSession`](ApiError::InvalidSession) if the session has expired.
    ///
    /// # Errors
    /// May also return a [`DatabaseException`](ApiError::DatabaseException).
    pub async fn get_session(parts: &mut Parts) -> Result<AllClassroomSessions, ApiError> {
        let jar = CookieJar::from_headers(&parts.headers);
        let db = parts
            .extensions
            .get::<PgPool>()
            .ok_or(ApiError::DatabaseException)?;

        // Extract the token from the cookie jar.
        let token = AllClassroomSessions::extract_token(&jar)?;

        AllClassroomSessions::retrieve_session(db, &token)
            .await?
            .ok_or(ApiError::InvalidSession)
    }

    /// Fetch the session associated with a classroom sessions token.
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The opaque session token from the session cookie.
    ///
    /// # Returns
    /// Returns [`Ok(Some(ClassroomSessions))`](AllClassroomSessions) containing the user id and account type.
    ///
    /// Returns [`Ok(None)`](None) if the token could not be found or has expired.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn retrieve_session(
        db: impl PgExecutor<'_>,
        token: &str,
    ) -> Result<Option<AllClassroomSessions>, ApiError> {
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

        Ok(Some(AllClassroomSessions {
            sessions: rows
                .into_iter()
                .map(|row| ClassroomSession {
                    id: row.classroom_id,
                    num: row.id_within_classroom,
                    name: row.name,
                })
                .collect::<Vec<ClassroomSession>>(),
        }))
    }

    /// Create a session for a classroom user for one classroom and insert it into the database.
    ///
    /// Workings for [`AllClassroomSessions`]: Takes a new token and inserts it into the token database.
    /// Then creates a session for every [`ClassroomSession`].
    ///
    /// # Arguments
    /// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
    ///
    /// `token` - The existing classroom sessions token this session needs to link to.
    ///
    /// `expires_at` - The [`DateTime<Utc>`](DateTime) when this session expires.
    ///
    /// `all_sessions` - The collection of classroom sessions to create.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn create_session(
        db: &mut PgConnection,
        token: &str,
        expires_at: DateTime<Utc>,
        all_sessions: &Vec<ClassroomSession>,
    ) -> Result<(), ApiError> {
        query!("INSERT INTO classroom_tokens (token) VALUES ($1)", token,)
            .execute(&mut *db)
            .await?;

        for session in all_sessions {
            ClassroomSession::create_session(
                &mut *db,
                token,
                expires_at,
                &(session.id, session.num),
            )
            .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::api_v1::create_token;
    use crate::db::test::{initialize_basic_user, initialize_classroom, put_user_in_classroom};
    use crate::get_connection_unwrap;
    use crate::structs::session::{AllClassroomSessions, ClassroomSession};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_and_get_all_sessions(db: PgPool) {
        let session = initialize_basic_user(&db).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), session.user_id).await;

        let (token, expires_at) = create_token(3600);

        let session = ClassroomSession {
            id: classroom_id,
            num: 1,
            name: "slachtoffer".to_owned(),
        };

        put_user_in_classroom(&db, &session).await;

        let all_sessions = AllClassroomSessions {
            sessions: vec![session],
        };

        AllClassroomSessions::create_session(
            get_connection_unwrap!(db),
            &token,
            expires_at,
            &all_sessions.sessions,
        )
        .await
        .unwrap();

        let retrieved_sessions = AllClassroomSessions::retrieve_session(&db, &token)
            .await
            .unwrap()
            .unwrap();

        // Validates if the user is in session, meaning the session is returned if it is true.
        assert_eq!(retrieved_sessions, all_sessions);
    }
}
