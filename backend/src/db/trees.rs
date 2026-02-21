use anyhow::{Error, Ok};
use sqlx::{PgPool, query};

/// Returns the index assigned to the new tree
pub async fn add_tree(db: &PgPool, owner_id: i32, title: &str) -> Result<i32, Error> {
    let id_record = query!(
        "INSERT INTO trees (owner_id, title, structure)
        VALUES ($1, $2, '{}')
        RETURNING id",
        owner_id,
        title
    )
    .fetch_one(db)
    .await?;

    Ok(id_record.id)
}
