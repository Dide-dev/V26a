/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::api_v1::extract_token;
use crate::db::session::get_session;
use crate::db_structs::session::{AllClassroomClaims, Claims, ClaimsType, ClassroomClaims};
use crate::errors::ApiError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use chrono::Utc;
use sqlx::PgPool;
use serde::de::DeserializeOwned;

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

fn get_claims<T: ClaimsType + DeserializeOwned>(
    parts: &mut Parts,
    state: &ArcState,
) -> Result<T, ApiError> {
    // Extract the token from the cookie jar.
    let jar = CookieJar::from_headers(&parts.headers);
    let token = extract_token(&jar)?;

    let db = parts
        .extensions
        .get::<PgPool>()
        .ok_or(ApiError::DatabaseException)?;

    get_session(db, &token)
        .await?
        .ok_or(ApiError::InvalidSession)
    // MERGE: this only gets one session for now,
    // should retrieve one from set for classrooms
}

impl FromRequestParts<ArcState> for Claims {
    type Rejection = ApiError;

    /// Get [`Claims`] from a user's cookies
    ///
    /// # Example
    /// Passing `claims: `[`Claims`] into an axum function will automatically call this function.
    /// ```rs
    /// async fn protected(claims: Claims) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user does not
    /// have a session cookie, or if cookies can not be extracted from the request.
    ///
    /// Returns [`ExpiredSession`](ApiError::ExpiredSession) if the user's session has expired.
    ///
    /// Returns [`InvalidSession`](ApiError::InvalidSession) if the user's session can not be created.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        get_claims(parts, state).await
    }
}

impl FromRequestParts<ArcState> for Option<Claims> {
    type Rejection = ApiError;

    /// Get optional [`Claims`] from a user's cookies
    ///
    /// # Example
    /// Passing `optional_claims: `[`Option<Claims>`](Claims) into an axum function will automatically call this function.
    /// ```rs
    /// async fn semi_protected(optional_claims: Option<Claims>) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Returns
    /// Some([`Claims`]) if the user is authenticated.
    ///
    /// None if one of the following is true:
    /// - The user does not have a session cookie,
    /// - The cookies can not be extracted from the request,
    /// - The user's session has expired,
    /// - A database query requesting the session data failed.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Claims::from_request_parts(parts, state).await.ok())
    }
}

impl FromRequestParts<ArcState> for ClassroomClaims {
    type Rejection = ApiError;

    /// Get JWT [`ClassroomClaims`] from a user's cookies
    ///
    /// # Example
    /// Passing `claims: `[`ClassroomClaims`] into an axum function will automatically call this function.
    /// ```rs
    /// async fn protected(claims: ClassroomClaims) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user does not
    /// have a session cookie, or if cookies can not be extracted from the request.
    ///
    /// Returns [`ExpiredSession`](ApiError::ExpiredSession) if the user's session has expired.
    ///
    /// Returns [`InvalidSession`](ApiError::InvalidSession) if the user's session can not be
    /// decrypted, or if another error is given by [`jsonwebtoken`].
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        // Get the classroom id from the uri
        let classroom_id = get_classroom_id_from_uri(parts.uri.path())?;

        // Get all active sessions
        let all_classroom_claims = get_claims::<AllClassroomClaims>(parts, state)?;

        // Get session corresponding to the correct classroom id
        let classroom_claims = all_classroom_claims
            .sessions
            .into_iter()
            .find(|s| s.id == classroom_id);

        // Check if a session exists for this classroom
        let Some(classroom_claims) = classroom_claims else {
            return Err(ApiError::MissingAuthentication);
        };

        // Check if this classroom session is expired
        if Utc::now().timestamp() > classroom_claims.exp {
            return Err(ApiError::ExpiredSession);
        }

        Ok(classroom_claims)
    }
}

impl FromRequestParts<ArcState> for Option<ClassroomClaims> {
    type Rejection = ApiError;

    /// Get optional JWT [`ClassroomClaims`] from a user's cookies
    ///
    /// # Example
    /// Passing `optional_claims: `[`Option<ClassroomClaims>`](ClassroomClaims) into an axum function will automatically call this function.
    /// ```rs
    /// async fn semi_protected(optional_claims: Option<ClassroomClaims>) -> Result<(), ApiError> {
    /// ...
    /// }
    /// ```
    ///
    /// # Returns
    /// Some([`ClassroomClaims`]) if the user is authenticated.
    ///
    /// None if one of the following is true:
    /// - The user does not have a session cookie,
    /// - The cookies can not be extracted from the request,
    /// - The user's session has expired,
    /// - The user's session can not be decrypted,
    /// - another error is given by [`jsonwebtoken`]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ArcState,
    ) -> Result<Self, Self::Rejection> {
        Ok(ClassroomClaims::from_request_parts(parts, state).await.ok())
    }
}
