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
    fn get_cookie_name() -> &'static str {
        "classroom_sessions"
    }
}

impl AllClassroomClaims {
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
    pub async fn get_claims(parts: &mut Parts) -> Result<AllClassroomClaims, ApiError> {
        let jar = CookieJar::from_headers(&parts.headers);
        let db = parts
            .extensions
            .get::<PgPool>()
            .ok_or(ApiError::DatabaseException)?;

        // Extract the token from the cookie jar.
        let token = AllClassroomClaims::extract_token(&jar)?;

        AllClassroomClaims::get_session(db, &token)
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
    /// # Returns
    /// Returns [`Ok(Some(AllClassroomClaims))`](AllClassroomClaims) containing the user id and account type.
    ///
    /// Returns [`Ok(None)`](None) if the token could not be found or has expired.
    ///
    /// # Errors
    /// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
    pub async fn get_session(
        db: impl PgExecutor<'_>,
        token: &str,
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
                .into_iter()
                .map(|row| ClassroomClaims {
                    id: row.classroom_id,
                    num: row.id_within_classroom,
                    name: row.name,
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
    pub async fn create_session(
        db: &mut PgConnection,
        token: &str,
        expires_at: DateTime<Utc>,
        all_claims: &Vec<ClassroomClaims>,
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
}

#[cfg(test)]
mod tests {
    use crate::api_v1::create_token;
    use crate::db::test::{initialize_basic_user, initialize_classroom, put_user_in_classroom};
    use crate::db_structs::session::{AllClassroomClaims, ClassroomClaims};
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_and_get_all_sessions(db: PgPool) {
        let claims = initialize_basic_user(&db).await;
        let classroom_id = initialize_classroom(get_connection_unwrap!(db), claims.sub).await;

        let (token, expires_at) = create_token(3600);

        let claims = ClassroomClaims {
            id: classroom_id,
            num: 1,
            name: "slachtoffer".to_owned(),
        };

        put_user_in_classroom(&db, &claims).await;

        let all_claims = AllClassroomClaims {
            claims: vec![claims],
        };

        AllClassroomClaims::create_session(
            get_connection_unwrap!(db),
            &token,
            expires_at,
            &all_claims.claims,
        )
        .await
        .unwrap();

        let retrieved_claims = AllClassroomClaims::get_session(&db, &token)
            .await
            .unwrap()
            .unwrap();

        // Validates if the user is in claims, meaning the session is returned if it is true.
        assert_eq!(retrieved_claims, all_claims);
    }
}
