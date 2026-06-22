/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::trees::verify_tree_permissions;
use crate::db::cards::{get_tree_of_card, update_card};
use crate::errors::ApiError;
use crate::structs::Validated;
use crate::structs::cards::CardData;
use crate::structs::session::Session;
use crate::{ArcState, get_connection};
use axum::extract::Path;
use axum::routing::put;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the cards category.
pub fn cards() -> Router<ArcState> {
    Router::new().route("/{card_id}", put(update_existing_card))
}

#[utoipa::path(
    put,
    path = "/{card_id}",
    context_path = "/api/v1/cards",
    request_body = CardData,
    responses(
        (status = 200, description = "Card successfully updated."),
        (status = 400, description = "Tree with given id does not exist."),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Session, Cookie),
        ("card_id" = i32, Path),
    )
)]
/// Update the contents of an existing card in the database.
///
/// # Arguments
/// `session` - The session cookie of the user.
///
/// `card_id` - The card to update.
///
/// `card_data` - The new data of the card, as [`CardData`].
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
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_existing_card(
    session: Session,
    Extension(db): Extension<PgPool>,
    Path(card_id): Path<i32>,
    Json(card_data): Json<CardData>,
) -> Result<(), ApiError> {
    card_data.validate_inputs()?;

    let tree_id = get_tree_of_card(&db, card_id).await?;

    // Check whether the tree of which a card is being updated exists and is owned by the user.
    verify_tree_permissions(get_connection!(db), tree_id, &session).await?;

    update_card(get_connection!(db), card_id, &card_data).await?;

    Ok(())
}
