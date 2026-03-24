/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use sqlx::{PgConnection, query};

/// Fetches the counter value from the given database.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// # Returns
/// The value if successful, or a [`DatabaseException`](ApiError::DatabaseException) if not.
pub async fn fetch_counter_value(db: &mut PgConnection) -> Result<i32, ApiError> {
    ensure_counter_exists(&mut *db).await?;
    Ok(query!("SELECT counter_value FROM counters WHERE id = 0")
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .counter_value)
}

/// Adds a number to the counter value from the given database.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `amount` - The amount to add to the counter.
///
/// # Returns
/// The updated value.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_counter_value(db: &mut PgConnection, amount: i32) -> Result<i32, ApiError> {
    ensure_counter_exists(&mut *db).await?;

    let new_counter_value = query!(
        "UPDATE counters
         SET counter_value = (
             (SELECT counter_value
              FROM counters
              WHERE id = 0
             ) + $1
        )
         WHERE id = 0
         RETURNING counter_value",
        amount
    )
    .fetch_one(&mut *db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .counter_value;

    Ok(new_counter_value)
}

/// Checks if the counter exists in the database, creating it if it doesn't.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// # Returns
/// Nothing.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn ensure_counter_exists(db: &mut PgConnection) -> Result<(), ApiError> {
    let exists = query!("SELECT EXISTS(SELECT 1 FROM counters WHERE id = 0)")
        .fetch_one(&mut *db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .exists
        .unwrap();

    if !exists {
        query!("INSERT INTO counters VALUES (0, $1)", 0)
            .execute(db)
            .await
            .map_err(|_| ApiError::DatabaseException)?;
    }

    Ok(())
}
