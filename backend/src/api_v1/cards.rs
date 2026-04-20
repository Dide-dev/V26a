/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::session::Claims;
use crate::api_v1::trees::verify_tree_permissions;
use crate::db::cards::{add_card, get_card_text, get_tree_of_card, update_card};
use crate::db_structs::cards::{CardAddData, CardUpdateData, LibraryCardAddData};
use crate::errors::ApiError;
use crate::{Config, get_connection};
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;
use crate::db::cards_library::cards_library_add;

/// Creates an `Axum` [`Router`] for the cards category.
pub fn cards() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_card))
        .route("/update", post(update_existing_card))
        .route("/add-to-library", post(add_card_to_library))
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

#[utoipa::path(
    post,
    path = "/add-to-library",
    context_path = "/api/v1/cards",
    request_body = LibraryCardAddData,
    responses(
        (status = 200, description = "Card successfully added to library."),
        (status = 400, description = "Tree with given id does not exist."),
        (status = 400, description = "Card with given id does not exist."),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Adds a card to the card library. Deep copies the card information to the database.
/// Only works if the person calling this api endpoint is the actual owner of the tree containing the card.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `card_data` - The data used to add the card to the library, as [`LibraryCardAddData`].
/// Existing card data will be overwritten with the new data.
///
/// # Returns
/// Nothing.
///
/// # Errors
///
/// Returns [`MissingTree`](ApiError::MissingTree) if the `tree_id` referenced in `card_data` does not exist.
///
/// Returns [`MissingCard`](ApiError::MissingCard) if the `card_id` referenced in `card_data` does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to
/// add this card to the card library.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_card_to_library(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(card_data): Json<LibraryCardAddData>,
) -> Result<Json<i32>, ApiError> {
    let owner_id = claims.sub.clone();

    // Verify tree exists and user is owner of tree
    verify_tree_permissions(&db, card_data.tree_id, claims).await?;

    // Verify card exists and retrieves the front and back
    let card_text = get_card_text(&db, card_data.card_id).await?;

    let new_card_id = cards_library_add(&db, &owner_id, &card_data.title, &card_data.tags, &card_text).await?;

    Ok(Json(new_card_id))
}
