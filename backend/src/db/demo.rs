use anyhow::Error;
use sqlx::{PgPool, query};

pub async fn fetch_counter_value(db: &PgPool) -> Result<i32, Error> {
    ensure_counter_exists(db).await?;
    Ok(query!("SELECT counter_value FROM counters WHERE id = 0")
        .fetch_one(db)
        .await?
        .counter_value)
}

pub async fn increment_counter_value(db: &PgPool) -> Result<i32, Error> {
    ensure_counter_exists(db).await?;

    let new_counter_value = query!(
        "UPDATE counters
         SET counter_value = (
             (SELECT counter_value
              FROM counters
              WHERE id = 0
             ) + 1
        )
         WHERE id = 0
         RETURNING counter_value"
    )
    .fetch_one(db)
    .await?
    .counter_value;

    Ok(new_counter_value)
}

async fn ensure_counter_exists(db: &PgPool) -> Result<(), Error> {
    let exists = query!("SELECT EXISTS(SELECT 1 FROM counters WHERE id = 0)")
        .fetch_one(db)
        .await?
        .exists
        .unwrap();

    if !exists {
        query!("INSERT INTO counters VALUES (0, $1)", 0) // pass variables like this to avoid SQL injection
            .execute(db)
            .await?;
    }

    Ok(())
}
