use std::collections::BTreeMap;

use crate::db::trees::verify_tree_ownership;
use crate::db_structs::cards::{CardAddData, CardGetData, CardUpdateData};

use crate::errors::ApiError;
use sqlx::{PgPool, query};

/// Add a new card to the database, returning its assigned id.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `card_data` - The data assigned to the new card as [`CardAddData`].
///
/// `user_id` - The id of the user who sent this request. Used to verify ownership of the tree being edited.
///
/// # Returns
/// The id assigned to the new card if successful.
///
/// # Error
/// Returns an [`ApiError::MissingTree`] if the `tree_id` referenced in [`CardAddData`] does not exist.
///
/// Returns an [`ApiError::NoPermission`] if the `user_id` does not match the `owner_id` field
/// for the given `tree_id` in `card_data`.
///
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn add_card(db: &PgPool, card_data: CardAddData, user_id: i32) -> Result<i32, ApiError> {
    // Check whether the tree which the card is being added to exists and is owned by the user.
    let is_owner = verify_tree_ownership(db, card_data.tree_id, user_id).await?;
    if !is_owner {
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

/// Update a card in the database.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `card_data` - The data used to update the card with, as [`CardUpdateData`].
///
/// `user_id` - The id of the user who sent this request. Used to verify ownership of the tree being edited.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns an [`ApiError::MissingCard`] if the `card_id`referenced in [`CardUpdateData`] does not exist.
///
/// Returns an [`ApiError::NoPermission`] if the `user_id` does not match the `owner_id` field
/// of the tree, to which the card being edited belongs.
///
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn update_card(
    db: &PgPool,
    card_data: CardUpdateData,
    user_id: i32,
) -> Result<(), ApiError> {
    // We retrieve the `tree_id` of the tree the card belongs to, and by doing so we simultaneously
    // check whether the card with `card_id` exists in the database.
    let tree_id = query!("SELECT tree_id FROM cards where id=$1", card_data.card_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .ok_or(ApiError::MissingCard(card_data.card_id))?
        .tree_id;

    // Check whether the tree of which a card is being updated exists and is owned by the user.
    let is_owner = verify_tree_ownership(db, tree_id, user_id).await?;
    if !is_owner {
        return Err(ApiError::NoPermission);
    }

    query!(
        "UPDATE cards SET (front, back, last_modified) = ($2, $3, CURRENT_TIMESTAMP) WHERE id = $1",
        card_data.card_id,
        card_data.new_front,
        card_data.new_back,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    // Also update the `last_modified` field in the tree the card belongs to.
    query!(
        "UPDATE trees SET last_modified = CURRENT_TIMESTAMP WHERE id = $1",
        tree_id,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// Get all cards from a tree.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_id` - The id of the tree of which the cards are asked.
///
/// # Returns
/// The cards of the tree as a [`BTreeMap`].
///
/// # Error
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn get_tree_cards(
    db: &PgPool,
    tree_id: i32,
) -> Result<BTreeMap<i32, CardGetData>, ApiError> {
    let cards = query!(
        "SELECT id, front, back
        FROM cards
        WHERE tree_id = $1",
        tree_id
    )
    .fetch_all(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(cards
        .into_iter()
        .map(|card| {
            (
                card.id,
                CardGetData {
                    front: card.front,
                    back: card.back,
                },
            )
        })
        .collect())
}
