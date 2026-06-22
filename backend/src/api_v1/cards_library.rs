/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::trees::verify_tree_permissions;
use crate::db::cards::{get_card_text, get_tree_of_card};
use crate::db::cards_library::{
    cards_library_add, cards_library_delete, cards_library_get_owner, get_queried_cards_count,
    get_queried_cards_list,
};
use crate::errors::ApiError;
use crate::get_connection;
use crate::state::ArcState;
use crate::structs::Validated;
use crate::structs::cards::{CardFilters, LibraryCardAddData, LibraryCardGetData};
use crate::structs::session::Session;
use crate::structs::users::AccountType;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the cards library category.
pub fn cards_library() -> Router<ArcState> {
    Router::new()
        .route("/cards", get(get_cards_from_library))
        .route(
            "/cards/{card_id}",
            post(add_card_to_library).delete(delete_card_from_library),
        )
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
        ("session" = Session, Cookie),
        ("card_id" = i32, Path),
    )
)]
/// Adds a card to the card library. Deep copies the card information to the database.
/// Only works if the person calling this api endpoint is the actual owner of the tree containing the card.
///
/// # Arguments
/// `session` - The session cookie of the user.
///
/// `card_data` - The data used to add the card to the library, as [`LibraryCardAddData`].
/// Existing card data will be overwritten with the new data.
///
/// `card_id` - The id of the card from a tree to put in the library. Note that this is different
/// from a `library_card_id`, as that refers to the id of a card that is in the cards library.
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
    session: Session,
    Extension(db): Extension<PgPool>,
    Path(card_id): Path<i32>,
    Json(card_data): Json<LibraryCardAddData>,
) -> Result<Json<i32>, ApiError> {
    card_data.validate_inputs()?;
    let owner_id = session.user_id;

    // Verify card exists and get id of tree
    let tree_id = get_tree_of_card(&db, card_id).await?;

    // Verify tree exists and user is owner of tree
    verify_tree_permissions(get_connection!(db), tree_id, &session).await?;

    // Retrieves the front and back of the card
    let card_text = get_card_text(&db, card_id).await?;

    let library_card_id = cards_library_add(
        get_connection!(db),
        owner_id,
        &card_data.title,
        card_data.tags,
        &card_text,
    )
    .await?;

    Ok(Json(library_card_id))
}

#[utoipa::path(
    delete,
    path = "/cards/{library_card_id}",
    context_path = "/api/v1/library",
    responses(
        (status = 200, description = "Card successfully removed from library."),
        (status = 400, description = "Card with given id does not exist."),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Session, Cookie),
        ("library_card_id" = i32, Path),
    )
)]
/// Removes a card from the card library.
/// Only works if the person calling this api endpoint is the actual owner of the library card,
/// or is an admin.
///
/// # Arguments
/// `session` - The session cookie of the user.
///
/// `library_card_id` - The id of a library card to remove. Note that this is different
/// from a `card_id`, which is the id of cards that are in a tree.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingCard`](ApiError::MissingCard) if the given `library_card_id` does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to
/// remove this card from the card library.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn delete_card_from_library(
    session: Session,
    Extension(db): Extension<PgPool>,
    Path(library_card_id): Path<i32>,
) -> Result<(), ApiError> {
    let Some(owner) = cards_library_get_owner(&db, library_card_id).await? else {
        return Err(ApiError::MissingCard(library_card_id));
    };

    if session.user_id != owner && session.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    cards_library_delete(&db, library_card_id).await
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
        ("session" = Session, Cookie),
        ("filters" = CardFilters, Query),
    )
)]
/// Get cards from the card library, that satisfy the given filters.
///
/// # Arguments
/// `session` - The session cookie of the user.
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
    _session: Session,
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
