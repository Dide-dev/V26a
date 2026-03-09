use crate::db::trees::verify_tree_ownership;
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
    // Check whether the tree which the card is being added to exists and is owned by the user.
    verify_tree_ownership(db, card_data.tree_id, user_id).await?;

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

/// Update a card in the database.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `card_data` - The data used to update the card with, as [`CardData`].
///
/// `user_id` - The id of the user who sent this request. Used to verify ownership of the tree being edited.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns an [`ApiError::IncompleteRequest`] if the `card_id` field in [`CardData`] is [`None`].
///
/// Returns an [`ApiError::MissingCard`] if the `card_id` referenced in [`CardData`] does not exist.
///
/// Returns an [`ApiError::IncorrectRequest`] if the `tree_id` referenced in [`CardData`] does not
/// match the `tree_id`, which the card being edited is related to.
///
/// Returns an [`ApiError::NoPermission`] if the `user_id` does not match the `owner_id` field
/// of the tree, to which the card being edited belongs.
///
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn update_card(db: &PgPool, card_data: CardData, user_id: i32) -> Result<(), ApiError> {
    // As the `card_id` field is optional in `CardData`, we check whether it is specified.
    let card_id = card_data
        .card_id
        .ok_or(ApiError::IncompleteRequest("cardId".to_string()))?;

    let tree_id = query!("SELECT tree_id FROM cards where id=$1", card_id,)
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::MissingCard(card_id))?
        .tree_id;

    // The `tree_id` field in `CardData` is not used,
    // but a mismatch indicates a mistake in the request.
    if tree_id != card_data.tree_id {
        return Err(ApiError::IncorrectRequest("treeId".to_string()));
    }

    // Check whether the tree of which a card is being updated exists and is owned by the user.
    verify_tree_ownership(db, tree_id, user_id).await?;

    query!(
        "UPDATE cards SET (front, back) = ($2, $3) WHERE id = $1",
        card_id,
        card_data.front,
        card_data.back,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}
