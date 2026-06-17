/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::trees::verify_tree_permissions;
use crate::db::cards::{get_card_text, get_tree_of_card};
use crate::db::cards_library::{
    cards_library_add, get_queried_cards_count, get_queried_cards_list,
};
use crate::db_structs::Validated;
use crate::db_structs::cards::{CardFilters, LibraryCardAddData, LibraryCardGetData};
use crate::db_structs::session::Claims;
use crate::errors::ApiError;
use crate::get_connection;
use crate::state::ArcState;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the cards library category.
pub fn cards_library() -> Router<ArcState> {
    Router::new()
        .route("/cards", get(get_cards_from_library))
        .route("/cards/{card_id}", post(add_card_to_library))
}

#[utoipa::path(
    post,
    path = "/cards/{card_id}",
    context_path = "/api/v1/library",
    request_body = LibraryCardAddData,
    responses(
        (status = 200, description = "Card successfully added to library.", body = i32),
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
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_card_to_library(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(card_id): Path<i32>,
    Json(card_data): Json<LibraryCardAddData>,
) -> Result<Json<i32>, ApiError> {
    card_data.validate_inputs()?;
    let owner_id = claims.sub;

    // Verify card exists and get id of tree
    let tree_id = get_tree_of_card(&db, card_id).await?;

    // Verify tree exists and user is owner of tree
    verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;

    // Retrieves the front and back of the card
    let card_text = get_card_text(&db, card_id).await?;

    let library_card_id = cards_library_add(
        get_connection!(db),
        owner_id,
        &card_data.title,
        &card_data.tags,
        &card_text,
    )
    .await?;

    Ok(Json(library_card_id))
}

#[utoipa::path(
    get,
    path = "/cards",
    context_path = "/api/v1/library",
    responses(
        (status = 200, description = "Successfully retrieved cards from library.", body = Vec<LibraryCardGetData>),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("filters" = CardFilters, Query),
    )
)]
/// Get cards from the card library, that satisfy the given filters.
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
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
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
