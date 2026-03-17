use crate::api_v1::session::Claims;
use crate::db::cards::{get_tree_cards, remove_card};
use crate::db::trees::{add_tree, db_get_tree_all_data, get_tree_owner, is_structure_valid};
use crate::db_structs::trees::{
    TreeData, TreeRemoveCardData, TreeStructure, TreeUpdateStructureData, TreeWithCards,
};
use crate::{Config, db};

use crate::errors::ApiError;

use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};

use sqlx::{PgConnection, PgPool};

/// Creates an `Axum` [`Router`] for the trees category.
pub fn trees() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_tree))
        .route("/update-structure", post(update_tree_structure))
        .route("/remove-card", post(remove_card_from_tree))
        .route("/get-with-cards/{tree_id}", get(get_tree_with_cards))
}

/// Add a tree to the database.
///
/// # Returns
/// The id assigned to the new tree as [`Json`].
///
/// # Error
/// May return an [`ApiError::DatabaseException`] instead.
async fn add_new_tree(
    claims: Claims,
    db: Extension<PgPool>,
    Json(tree_data): Json<TreeData>,
) -> Result<Json<i32>, ApiError> {
    let id = add_tree(&db, tree_data, claims.sub).await?;

    let json_id = Json(id);
    Ok(json_id)
}

/// Update the structure of an existing tree in the database.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns an [`ApiError::MissingTree`] if no tree with the `id` specified in `tree_update_data` exists.
///
/// Returns an [`ApiError::DatabaseMemoryCap`] if more than 10,000 cards in the `Cards` table
/// link to `tree_id`.
///
/// Returns an [`ApiError::NoPermission`] if the user does not have permission to change the tree
/// for the given `tree_id`.
///
/// Returns an [`ApiError::MismatchedStructure`] if either of the following are the case:
/// * Not all cards in the structure reference the `tree_id` in the `Cards` database table.
/// * A card is referenced more than once in the structure.
///
/// May return an [`ApiError::DatabaseException`] or an [`ApiError::SerdeException`] instead.
async fn update_tree_structure(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(tree_update_data): Json<TreeUpdateStructureData>,
) -> Result<(), ApiError> {
    // Verify the tree's existence and the user's permissions.
    verify_tree_ownership(&db, tree_update_data.id, claims.sub).await?;

    let mut tx = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    update_structure_if_valid(&mut tx, tree_update_data.id, tree_update_data.new_structure).await?;

    tx.commit().await.map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// TODO: Add docs.
async fn remove_card_from_tree(
    claims: Claims,
    db: Extension<PgPool>,
    Json(tree_card_removal_data): Json<TreeRemoveCardData>,
) -> Result<(), ApiError> {
    // Verify the tree's existence and the user's permissions.
    verify_tree_ownership(&db, tree_card_removal_data.id, claims.sub).await?;

    let mut tx = db.begin().await.map_err(|_| ApiError::DatabaseException)?;

    // First, we remove the card from the database, within the transaction.
    remove_card(&mut *tx, tree_card_removal_data.card_id).await?;

    // Then, we check if the structure is still valid with the card being removed.
    // This ensures the card is actually removed from the structure.
    update_structure_if_valid(
        &mut tx,
        tree_card_removal_data.id,
        tree_card_removal_data.new_structure,
    )
    .await?;

    tx.commit().await.map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// TODO: Add docs.
async fn update_structure_if_valid(
    db: &mut PgConnection,
    tree_id: i32,
    new_structure: TreeStructure,
) -> Result<(), ApiError> {
    // Verify the new structure matches the cards linked to the tree in the database.
    let is_structure_valid = is_structure_valid(&mut *db, tree_id, &new_structure).await?;
    if !is_structure_valid {
        return Err(ApiError::MismatchedStructure);
    }

    db::trees::update_tree_structure(db, tree_id, new_structure).await?;

    Ok(())
}

/// Request all data of a specific tree together with all its cards from the database.
///
/// # Returns
/// The tree data together with the cards as [`Json<TreeComplete>`].
///
/// # Error
/// Return an [`ApiError::MissingTree`] if `tree_id` does not exist.
///
/// May return an [`ApiError`] instead.
async fn get_tree_with_cards(
    db: Extension<PgPool>,
    Path(tree_id): Path<i32>,
) -> Result<Json<TreeWithCards>, ApiError> {
    let (data, gen_data, structure) = db_get_tree_all_data(&db, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    let tree_cards = get_tree_cards(&db, tree_id).await?;

    Ok(Json(TreeWithCards {
        data,
        gen_data,
        structure,
        cards: tree_cards,
    }))
}

/// Checks whether the tree exists and if it is owner by the user.
///
/// # Params
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_id` - The tree of which the ownerships gets verified.
///
/// `user_id` - The id of the user who sent this request.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns an [`ApiError::NoPermission`] if the user does not have permission to change the tree
/// for the given `tree_id`.
///
/// Returns an [`ApiError::MissingTree`] if no tree with `tree_id` exists.
///
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn verify_tree_ownership(
    db: &PgPool,
    tree_id: i32,
    user_id: i32,
) -> Result<(), ApiError> {
    // Check whether the tree exists.
    let owner_id = get_tree_owner(db, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    if user_id == owner_id {
        return Err(ApiError::NoPermission);
    }

    Ok(())
}
