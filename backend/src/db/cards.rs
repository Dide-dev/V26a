use crate::db_structs::cards::CardData;

use anyhow::{Error, Ok};
use sqlx::{PgPool, query};

/// Add a new tree to the database, returning its assigned index.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
/// `card_data` - The data assigned to the new tree
/// `owner_id` - The id of the owner of the tree
///
/// # Returns
/// Returns the index assigned to the new card.
pub async fn add_card(db: &PgPool, card_data: CardData, tree_id: i32) -> Result<i32, Error> {
    let id_record = query!(
        "INSERT INTO cards (tree_id, front, back)
        VALUES ($1, $2, $3)
        RETURNING id",
        tree_id,
        card_data.front,
        card_data.back,
    )
        .fetch_one(db)
        .await?;

    Ok(id_record.id)
}