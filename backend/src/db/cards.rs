use crate::db::trees::{get_tree_owner, tree_exists};
use crate::db_structs::cards::CardData;

use crate::errors::ApiError;
use sqlx::{PgPool, query};

/// Add a new card to the database, returning its assigned index.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `card_data` - The data assigned to the new card as [`CardData`].
///
/// `user_id` - The id of the user who sent this request. Used to verify ownership of the tree being edited.
///
/// # Returns
/// The index assigned to the new card if successful, or an [`ApiError`] if not.
///
/// # Error
/// Returns an [`ApiError::MissingTree`] if the `tree_id` referenced in [`CardData`] does not exist.
///
/// Returns an [`ApiError::MissingAuthorisation`] if the `user_id` does not match the `owner_id` field
/// for the given `tree_id` in `card_data`.
pub async fn add_card(db: &PgPool, card_data: CardData, user_id: i32) -> Result<i32, ApiError> {
    // Check whether the tree which the card is being added to exists.
    let tree_exists = tree_exists(db, card_data.tree_id).await?;

    if !tree_exists {
        return Err(ApiError::MissingTree(card_data.tree_id));
    }

    // Check whether the user is authorized to add a card to the tree.
    let owner_id = get_tree_owner(db, card_data.tree_id).await?;

    if !(user_id == owner_id) {
        return Err(ApiError::MissingAuthorisation);
    }

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
