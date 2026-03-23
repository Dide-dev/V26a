pub mod auth;
pub mod cards;
pub mod demo;
pub mod trees;
pub mod users;

/// Get a [`PgConnection`](sqlx::PgConnection) from a [`PgPool`](sqlx::PgPool).
///
/// # Errors
/// May return a [`DatabaseException`](crate::errors::ApiError::DatabaseException) if the connection
/// can not be acquired.
#[macro_export]
macro_rules! get_connection {
    ( $pool:expr ) => {
        &mut *($pool
            .acquire()
            .await
            .map_err(|_| ApiError::DatabaseException)?)
    };
}

/// Get a [`PgConnection`](sqlx::PgConnection) from a [`PgPool`](sqlx::PgPool).
///
/// **Warning:** might panic. Use the [`get_connection`] macro instead unless you're writing tests.
/// Exists only for cases where the return type does not and should not match
/// Result<_, [`ApiError`](crate::errors::ApiError)>.
///
/// # Panic
/// May panic if the connection can not be acquired.
#[macro_export]
macro_rules! unsafe_get_connection {
    ( $pool:expr ) => {
        &mut *($pool.acquire().await.unwrap())
    };
}
