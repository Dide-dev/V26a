use anyhow::{Error, Ok};
use sqlx::{PgPool, query};

/// TODO: Returns the index assigned to the new tree
pub async fn add_tree(db: &PgPool) -> Result<(), Error> {
    // FIX: unknown if this query is working, for it compiles
    query!(
        "INSERT INTO trees (owner_id, title, structure)
        VALUES (0, 'test_value', '{}')"
    )
    .execute(db)
    .await?;

    Ok(())
}
