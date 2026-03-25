/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

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
/// **Warning:** might panic. If not writing tests, use the [`get_connection`] macro instead.
/// Exists only for some test where the return type does not and should not match
/// Result<_, [`ApiError`](crate::errors::ApiError)>.
///
/// # Panic
/// May panic if the connection can not be acquired.
#[cfg(test)]
#[macro_export]
macro_rules! get_connection_unwrap {
    ( $pool:expr ) => {
        &mut *($pool.acquire().await.unwrap())
    };
}
