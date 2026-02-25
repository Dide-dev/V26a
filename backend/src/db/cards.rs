use anyhow::{Error, Ok};
use sqlx::{PgPool, query};

/// Returns the index assigned to the new tree
pub async fn add_card(db: &PgPool, owner_id: i32, front: &str, back: &str) -> Result<i32, Error> {
    let id_record = query!(
        "INSERT INTO hints (owner_id, front, back)
        VALUES ($1, $2, $3)
        RETURNING id",
        owner_id,
        front,
        back,
    )
        .fetch_one(db)
        .await?;

    Ok(id_record.id)
}