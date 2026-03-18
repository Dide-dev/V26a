use crate::Config;
use crate::db::cards::{add_card, get_tree_of_card, update_card};
use crate::db_structs::cards::{CardAddData, CardUpdateData};

use crate::api_v1::session::Claims;
use crate::api_v1::trees::verify_tree_ownership;
use crate::db::get_connection;
use crate::errors::ApiError;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the cards category.
pub fn cards() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_card))
        .route("/update", post(update_existing_card))
}

/// Add a card to the database.
///
/// # Returns
/// The id assigned to the new card.
///
/// # Error
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change the tree
/// for the given `tree_id` referenced in `card_data`.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with the `tree_id` referenced in `card_data` exists.
///
/// May return an [`ApiError`] instead.
async fn add_new_card(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(card_data): Json<CardAddData>,
) -> Result<Json<i32>, ApiError> {
    // Check whether the tree of which a card is being updated exists and is owned by the user.
    verify_tree_ownership(&db, card_data.tree_id, claims.sub).await?;

    let id = add_card(&db, card_data).await?;

    Ok(Json(id))
}

/// Update the contents of an existing card in the database.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns [`MissingCard`](ApiError::MissingCard) if the `card_id` referenced in `card_data` does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change the tree
/// the card belongs to.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if the tree the card belongs to no longer exists.
///
/// May return an [`ApiError`] instead.
async fn update_existing_card(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(card_data): Json<CardUpdateData>,
) -> Result<(), ApiError> {
    let tree_id = get_tree_of_card(&db, card_data.card_id).await?;

    // Check whether the tree of which a card is being updated exists and is owned by the user.
    verify_tree_ownership(&db, tree_id, claims.sub).await?;

    update_card(&mut *get_connection(db).await?, card_data).await?;

    Ok(())
}
