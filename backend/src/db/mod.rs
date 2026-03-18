use crate::errors::ApiError;
use sqlx::pool::PoolConnection;
use sqlx::{PgPool, Postgres};

pub mod auth;
pub mod cards;
pub mod demo;
pub mod trees;
pub mod users;

/// Get a [`PoolConnection`] from a [`PgPool`].
///
/// # Example
/// To get a [`PgConnection`](sqlx::PgConnection), you need to deref the value obtained and use it as a mutable reference.
/// ```rs
/// function_using_pg_connection(&mut *get_connection(db).await?).await?;
/// ```
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) if the connection can not be
/// acquired.
pub async fn get_connection(pool: PgPool) -> Result<PoolConnection<Postgres>, ApiError> {
    pool.acquire()
        .await
        .map_err(|_| ApiError::DatabaseException)
}
