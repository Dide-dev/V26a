use crate::db::trees::get_tree_owner;
use crate::db_structs::cards::CardData;

use crate::errors::ApiError;
use sqlx::{PgPool, query};

/// Add a new card to the database, returning its assigned id.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `card_data` - The data assigned to the new card as [`CardData`].
///
/// `user_id` - The id of the user who sent this request. Used to verify ownership of the tree being edited.
///
/// # Returns
/// The id assigned to the new card if successful.
///
/// # Error
/// Returns an [`ApiError::MissingTree`] if the `tree_id` referenced in [`CardData`] does not exist.
///
/// Returns an [`ApiError::NoPermission`] if the `user_id` does not match the `owner_id` field
/// for the given `tree_id` in `card_data`.
///
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn add_card(db: &PgPool, card_data: CardData, user_id: i32) -> Result<i32, ApiError> {
    // Check whether the tree which the card is being added to exists.
    let owner_id = get_tree_owner(db, card_data.tree_id)
        .await?
        .ok_or(ApiError::MissingTree(card_data.tree_id))?;

    // Check whether the user is authorized to add a card to the tree.
    if user_id != owner_id {
        return Err(ApiError::NoPermission);
    }

    let card_id = query!(
        "INSERT INTO cards (tree_id, front, back)
        VALUES ($1, $2, $3)
        RETURNING id",
        card_data.tree_id,
        card_data.front,
        card_data.back,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .id;

    Ok(card_id)
}
