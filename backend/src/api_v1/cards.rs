/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::trees::verify_tree_permissions;
use crate::db::cards::{add_card, get_card_text, get_tree_of_card, update_card};
use crate::db::cards_library::{
    cards_library_add, get_queried_cards_count, get_queried_cards_list,
};
use crate::db_structs::cards::{
    CardAddData, CardFilters, CardUpdateData, LibraryCardAddData, LibraryCardGetData,
};
use crate::db_structs::session::Claims;
use crate::errors::ApiError;
use crate::{ArcState, get_connection};
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the cards category.
pub fn cards() -> Router<ArcState> {
    Router::new()
        .route("/add", post(add_new_card))
        .route("/update", post(update_existing_card))
        .route("/{card_id}/add-to-library", post(add_card_to_library))
        .route("/get-library", get(get_cards_from_library))
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
    verify_tree_permissions(get_connection!(db), card_data.tree_id, &claims).await?;

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
    verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;

    update_card(get_connection!(db), &card_data).await?;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/{card_id}/add-to-library",
    context_path = "/api/v1/cards",
    request_body = LibraryCardAddData,
    responses(
        (status = 200, description = "Card successfully added to library."),
        (status = 400, description = "Tree and/or card with given id does not exist."),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("card_id" = i32, Path),
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
/// The id of the new card in the library database as [`Json<i32>`].
///
/// # Errors
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
    Path(card_id): Path<i32>,
    Json(card_data): Json<LibraryCardAddData>,
) -> Result<Json<i32>, ApiError> {
    let owner_id = claims.sub;

    // Verify card exists and get id of tree
    let tree_id = get_tree_of_card(&db, card_id).await?;

    // Verify tree exists and user is owner of tree
    verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;

    // Retrieves the front and back of the card
    let card_text = get_card_text(&db, card_id).await?;

    let library_card_id = cards_library_add(
        &db,
        &owner_id,
        &card_data.title,
        &card_data.tags,
        &card_text,
    )
    .await?;

    Ok(Json(library_card_id))
}

#[utoipa::path(
    post,
    path = "/get-library",
    context_path = "/api/v1/cards",
    responses(
        (status = 200, description = "Successfully retrieved cards from libary."),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("filters" = CardFilters, Query),
    )
)]
/// Gets cards from the card library given the name of the id of the owner or given tags.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `filters` - The requested filters to apply to the cards, given as [`CardFilters`] through a query.
///
/// # Returns
/// A list with the cards satisfying the filters.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_cards_from_library(
    _claims: Claims,
    Extension(db): Extension<PgPool>,
    Query(filters): Query<CardFilters>,
) -> Result<Json<Vec<LibraryCardGetData>>, ApiError> {
    let mut transaction = db.begin().await?;

    let cards_count = get_queried_cards_count(&mut *transaction, &filters).await?;
    if cards_count == 0 {
        return Ok(Json(Vec::new()));
    }

    let cards = get_queried_cards_list(&mut *transaction, &filters).await?;

    transaction.commit().await?;

    Ok(Json(cards))
}
