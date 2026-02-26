use crate::errors::ApiError;
use anyhow::Error;
use sqlx::{PgPool, query};

/// Fetches the counter value from the given database. Returning the value if successful,
/// or an error if not.
///
/// # Arguments
/// `db` - a reference to the database connection to use for the query.
pub async fn fetch_counter_value(db: &PgPool) -> Result<i32, ApiError> {
    ensure_counter_exists(db).await?;
    Ok(query!("SELECT counter_value FROM counters WHERE id = 0")
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .counter_value)
}

/// Adds a number to the counter value from the given database. Returning the updated value
/// if successful, or an error if not.
///
/// # Arguments
/// `db` - a reference to the database connection to use for the query.
///
/// `amount` - the amount to add to the counter.
pub async fn add_counter_value(db: &PgPool, amount: i32) -> Result<i32, ApiError> {
    ensure_counter_exists(db).await?;

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
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .counter_value;

    Ok(new_counter_value)
}

/// Checks if the counter exists in the database, creating it if it doesn't. Returning nothing if
/// successful, or an error if not.
///
/// # Arguments
/// `db` - a reference to the database connection to use for the query.
async fn ensure_counter_exists(db: &PgPool) -> Result<(), ApiError> {
    let exists = query!("SELECT EXISTS(SELECT 1 FROM counters WHERE id = 0)")
        .fetch_one(db)
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
