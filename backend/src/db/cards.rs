use crate::db_structs::cards::CardData;

use crate::errors::ApiError;
use sqlx::{PgPool, query};

/// Add a new card to the database, returning its assigned index.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `card_data` - The data assigned to the new card as [`CardData`]
///
/// # Returns
/// The index assigned to the new card if successful, or an [`ApiError`] if not.
pub async fn add_card(db: &PgPool, card_data: CardData) -> Result<i32, ApiError> {
    let id_record = query!(
        "INSERT INTO cards (tree_id, front, back)
        VALUES ($1, $2, $3)
        RETURNING id",
        card_data.tree_id,
        card_data.front,
        card_data.back,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(id_record.id)
}