use crate::db::trees::verify_tree_ownership;
use crate::db_structs::cards::{CardAddData, CardUpdateData};

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
/// `card_data` - The data used to update the card with, as [`CardUpdateData`].
///
/// `user_id` - The id of the user who sent this request. Used to verify ownership of the tree being edited.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns an [`ApiError::IncompleteRequest`] if both the `new_front` and `new_back` fields in
/// [`CardUpdateData`] are [`None`].
///
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
    let tree_id_option = query!("SELECT tree_id FROM cards where id=$1", card_data.card_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    let Some(tree_id) = tree_id_option else {
        return Err(ApiError::MissingCard(card_data.card_id));
    };

    // Check whether the tree of which a card is being updated exists and is owned by the user.
    verify_tree_ownership(db, tree_id.tree_id, user_id).await?;

    let mut updated_rows = 0;

    // We only update the front if the field contains a new value.
    if let Some(new_front) = card_data.new_front {
        updated_rows += query!(
            "UPDATE cards SET front = $2 WHERE id = $1",
            card_data.card_id,
            new_front,
        )
        .execute(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .rows_affected();
    }

    // We only update the back if the field contains a new value.
    if let Some(new_back) = card_data.new_back {
        updated_rows += query!(
            "UPDATE cards SET back = $2 WHERE id = $1",
            card_data.card_id,
            new_back,
        )
        .execute(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .rows_affected();
    }

    if updated_rows == 0 {
        return Err(ApiError::IncompleteRequest(
            ["newFront".to_string(), "newBack".to_string()].to_vec(),
        ));
    }

    Ok(())
}
