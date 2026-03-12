use crate::api_v1::session::Claims;
use crate::db::cards::get_tree_cards;
use crate::db::trees::{
    add_tree, db_get_tree_all_data, verify_tree_ownership, verify_structure_cards_exist,
};
use crate::db_structs::trees::{TreeData, TreeUpdateData, TreeWithCards};
use crate::{Config, db};

use crate::errors::ApiError;

use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};

use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the trees category.
pub fn trees() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_tree))
        .route("/update-structure", post(update_tree_structure))
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
/// Returns an [`ApiError::NoPermission`] if the user does not have permission to change the tree
/// for the given `tree_id`.
///
/// May return an [`ApiError::DatabaseException`] or an [`ApiError::SerdeException`] instead.
async fn update_tree_structure(
    claims: Claims,
    db: Extension<PgPool>,
    Json(tree_update_data): Json<TreeUpdateData>,
) -> Result<(), ApiError> {
    // Verify the tree's existence and the user's permissions.
    let is_owner = verify_tree_ownership(&db, tree_update_data.id, claims.sub).await?;
    if !is_owner {
        return Err(ApiError::NoPermission);
    }

    // Verify the new structure matches the cards linked to the tree in the database.
    let cards_match_structure =
        verify_structure_cards_exist(&db, tree_update_data.id, &tree_update_data.new_structure)
            .await?;
    if !cards_match_structure {
        return Err(ApiError::MismatchedStructure);
    }

    db::trees::update_tree_structure(&db, tree_update_data.id, tree_update_data.new_structure)
        .await?;

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
