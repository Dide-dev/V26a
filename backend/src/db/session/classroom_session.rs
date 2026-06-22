/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use crate::structs::session::{ClassroomSession, SessionType};
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use sqlx::{PgConnection, PgExecutor, PgPool, query};

impl SessionType for ClassroomSession {
    fn get_cookie_name() -> &'static str {
        "classroom_sessions"
    }
}

impl ClassroomSession {
    /// Used in `FromRequestParts` for `ClassroomSession` to retrieve the `ClassroomSession`.
    ///
    /// # Arguments
    /// `parts` - a [`&mut Parts`](Parts).
    ///
    /// # Returns
    /// Returns the [`ClassroomSession`] if there is a non-expired session.
    ///
    /// Returns a [`MissingAuthentication`](ApiError::MissingAuthentication) if no token was found.
    ///
    /// Returns an [`InvalidSession`](ApiError::InvalidSession) if the session has expired.
    ///
    /// Returns an [`ClassroomIdRetrievalError`](ApiError::ClassroomIdRetrievalError) if the
    /// classroom id could not be retrieved from the api uri.
    ///
    /// # Errors
    /// May also return a [`DatabaseException`](ApiError::DatabaseException).
    pub async fn get_session(parts: &mut Parts) -> Result<ClassroomSession, ApiError> {
        let jar = CookieJar::from_headers(&parts.headers);
        let db = parts
            .extensions
            .get::<PgPool>()
            .ok_or(ApiError::DatabaseException)?;

        // Get the classroom id from the uri
        let classroom_id = get_classroom_id_from_uri(parts.uri.path())?;

        // Extract the token from the cookie jar.
        let token = Self::extract_token(&jar)?;

        ClassroomSession::retrieve_session(db, &token, classroom_id)
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
    /// `classroom_id` - The id of the classroom retrieved from the api uri.
    ///
    /// # Returns
    /// Returns [`Ok(Some(ClassroomSession))`](ClassroomSession) containing the user id and account type.
    ///
    /// Returns [`Ok(None)`](None) if the token could not be found or has expired.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn retrieve_session(
        db: impl PgExecutor<'_>,
        token: &str,
        classroom_id: i32,
    ) -> Result<Option<ClassroomSession>, ApiError> {
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

        let session = match row {
            Some(row) => Some(ClassroomSession {
                id: row.classroom_id,
                num: row.id_within_classroom,
                name: row.name,
            }),
            None => None,
        };

        Ok(session)
    }

    /// Create a session for a classroom user for one classroom and insert it into the database.
    ///
    /// Workings for `ClassroomSession`: Connects it to an existing token.
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
    pub async fn create_session(
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

/// If the relative api call path starts with `/{id}/`, returns `id`.
/// Otherwise, returns an [`ClassroomIdRetrievalError`](ApiError::ClassroomIdRetrievalError).
fn get_classroom_id_from_uri(uri: &str) -> Result<i32, ApiError> {
    let mut vec = uri.split('/').collect::<Vec<&str>>();
    vec.retain(|&s| !s.is_empty() && s != "." && s != "..");

    match vec[0].parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(ApiError::ClassroomIdRetrievalError),
    }
}

#[cfg(test)]
mod tests {
    use crate::api_v1::create_token;
    use crate::db::classroom_session::get_classroom_id_from_uri;
    use crate::db::test::{
        initialize_basic_user, initialize_classroom, put_classroom_token_in_db,
        put_user_in_classroom,
    };
    use crate::get_connection_unwrap;
    use crate::structs::session::ClassroomSession;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_and_get_session(db: PgPool) {
        let session = initialize_basic_user(&db).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), session.user_id).await;

        let (token, expires_at) = create_token(3600);
        put_classroom_token_in_db(&db, &token).await;

        let session = ClassroomSession {
            id: classroom_id,
            num: 1,
            name: "slachtoffer".to_owned(),
        };

        put_user_in_classroom(&db, &session).await;

        ClassroomSession::create_session(
            get_connection_unwrap!(db),
            &token,
            expires_at,
            &(session.id, session.num),
        )
        .await
        .unwrap();

        let retrieved_session = ClassroomSession::retrieve_session(&db, &token, classroom_id)
            .await
            .unwrap()
            .unwrap();

        // Validates if the user is in session, meaning the session is returned if it is true.
        assert_eq!(retrieved_session, session);
    }

    #[test]
    fn test_get_classroom_id_from_uri_pass() {
        // check if fn works with leading slash
        let uri = "/33/all/hail/Lorax/1/";
        let result = get_classroom_id_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 33);

        // also check if fn works without leading slash
        let uri = "33/all/hail/Lorax/1/";
        let result = get_classroom_id_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 33);
    }

    #[test]
    fn test_get_classroom_id_from_uri_fail() {
        let uri = "/thirtythree/all/hail/Lorax/1/";
        let result = get_classroom_id_from_uri(uri);
        assert!(result.is_err());
    }
}
