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
    /// Returns an [`ClassroomIdRetrievalError`](ApiError::ClassroomIdRetrievalError) if the
    /// classroom id could not be retrieved from the api uri.
    ///
    /// # Errors
    /// May also return a [`DatabaseException`](ApiError::DatabaseException).
    pub async fn get_claims(parts: &mut Parts) -> Result<ClassroomClaims, ApiError> {
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
    pub async fn get_session(
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
    use crate::db::classroom_claims::get_classroom_id_from_uri;
    use crate::db::test::{
        initialize_basic_user, initialize_classroom, put_classroom_token_in_db,
        put_user_in_classroom,
    };
    use crate::db_structs::session::ClassroomClaims;
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_and_get_session(db: PgPool) {
        let claims = initialize_basic_user(&db).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), claims.sub).await;

        let (token, expires_at) = create_token(3600);
        put_classroom_token_in_db(&db, &token).await;

        let claims = ClassroomClaims {
            id: classroom_id,
            num: 1,
            name: "slachtoffer".to_owned(),
        };

        put_user_in_classroom(&db, &claims).await;

        ClassroomClaims::create_session(
            get_connection_unwrap!(db),
            &token,
            expires_at,
            &(claims.id, claims.num),
        )
        .await
        .unwrap();

        let retrieved_claims = ClassroomClaims::get_session(&db, &token, classroom_id)
            .await
            .unwrap()
            .unwrap();

        // Validates if the user is in claims, meaning the session is returned if it is true.
        assert_eq!(retrieved_claims, claims);
    }

    #[test]
    fn test_get_classroom_id_from_uri_pass() {
        let uri = "/33/all/hail/Lorax/1/";
        let result = get_classroom_id_from_uri(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 33);

        let uri = "33/all/hail/Lorax/1/";
        let result = get_classroom_id_from_uri(uri);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_classroom_id_from_uri_fail() {
        let uri = "/thirtythree/all/hail/Lorax/1/";
        let result = get_classroom_id_from_uri(uri);
        assert!(result.is_err());
    }
}
