/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::session::Claims;
use crate::api_v1::trees::verify_tree_permissions;
use crate::db::cards::{add_card, get_tree_of_card, update_card};
use crate::db_structs::cards::{CardAddData, CardUpdateData};
use crate::errors::ApiError;
use crate::{Config, get_connection};
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the cards category.
pub fn cards() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_card))
        .route("/update", post(update_existing_card))
}

#[utoipa::path(
    post,
    path = "/add",
    context_path = "/api/v1/cards",
    request_body = CardAddData,
    responses(
        (status = 200, description = "Card successfully added with returned id.", body = i32, content_type = "application/json"),
        (status = 400, description = "Tree with given id does not exist."),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Add a card to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `card_data` - The data used to initialize the card, as [`CardAddData`].
///
/// # Returns
/// The id assigned to the new card.
///
/// # Errors
/// Returns [`MissingTree`](ApiError::MissingTree) if no tree with the `tree_id` referenced in `card_data` exists.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the tree for the given `tree_id` referenced in `card_data`.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_card(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(card_data): Json<CardAddData>,
) -> Result<Json<i32>, ApiError> {
    // Check whether the tree of which a card is being updated exists and is owned by the user.
    verify_tree_permissions(&db, card_data.tree_id, claims).await?;

    let id = add_card(&db, &card_data).await?;

    Ok(Json(id))
}

#[utoipa::path(
    post,
    path = "/update",
    context_path = "/api/v1/cards",
    request_body = CardUpdateData,
    responses(
        (status = 200, description = "Card successfully updated."),
        (status = 400, description = "Tree with given id does not exist."),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Update the contents of an existing card in the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `card_data` - The data used to update the card, as [`CardUpdateData`].
/// Existing card data will be overwritten with the new data.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingCard`](ApiError::MissingCard) if the `card_id` referenced in `card_data` does not exist.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if the tree the card belongs to no longer exists.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the tree the card belongs to.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_existing_card(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(card_data): Json<CardUpdateData>,
) -> Result<(), ApiError> {
    let tree_id = get_tree_of_card(&db, card_data.card_id).await?;

    // Check whether the tree of which a card is being updated exists and is owned by the user.
    verify_tree_permissions(&db, tree_id, claims).await?;

    update_card(get_connection!(db), &card_data).await?;

    Ok(())
}
