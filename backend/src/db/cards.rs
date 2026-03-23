use crate::db_structs::cards::{CardAddData, CardGetData, CardUpdateData};
use std::collections::BTreeMap;

use crate::errors::ApiError;
use sqlx::{PgConnection, PgExecutor, query};

/// Add a new card to the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `card_data` - The data assigned to the new card as [`CardAddData`].
///
/// # Returns
/// The id assigned to the new card if successful.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_card(db: impl PgExecutor<'_>, card_data: CardAddData) -> Result<i32, ApiError> {
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

/// Remove a card from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `card_id` - The id of the card which should be removed.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn remove_card(db: impl PgExecutor<'_>, card_id: i32) -> Result<(), ApiError> {
    // We only return the id to handle the case where the card does not exist with fetch_optional().
    query!("DELETE FROM cards WHERE id = $1 RETURNING id", card_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .ok_or(ApiError::MissingCard(card_id))?;

    Ok(())
}

/// Update a card in the database.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `card_data` - The data used to update the card with, as [`CardUpdateData`].
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_card(db: &mut PgConnection, card_data: CardUpdateData) -> Result<(), ApiError> {
    // We only return the id to handle the case where the card does not exist with fetch_optional().
    query!(
        "UPDATE cards SET (front, back, last_modified) = ($2, $3, CURRENT_TIMESTAMP) WHERE id = $1 RETURNING id",
        card_data.card_id,
        card_data.new_front,
        card_data.new_back,
    )
    .fetch_optional(&mut *db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .ok_or(ApiError::MissingCard(card_data.card_id))?;

    // Also update the `last_modified` field in the tree the card belongs to.
    query!(
        "UPDATE trees SET last_modified = CURRENT_TIMESTAMP
             WHERE id = (SELECT tree_id FROM cards where id = $1)",
        card_data.card_id,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// Get all cards from a tree.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree of which the cards are asked.
///
/// # Returns
/// The cards of the tree as a [`BTreeMap`].
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_tree_cards(
    db: impl PgExecutor<'_>,
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

/// Get the id of the tree the card belongs to.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `card_id` - The id of the card of which the corresponding tree is queried.
///
/// # Returns
/// The id of the tree the card belongs to, according to the `cards` table in the database.
///
/// # Error
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_tree_of_card(db: impl PgExecutor<'_>, card_id: i32) -> Result<i32, ApiError> {
    // We retrieve the `tree_id` of the tree the card belongs to, and by doing so we simultaneously
    // check whether the card with `card_id` exists in the database.
    let tree_id = query!("SELECT tree_id FROM cards where id=$1", card_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .ok_or(ApiError::MissingCard(card_id))?
        .tree_id;

    Ok(tree_id)
}
