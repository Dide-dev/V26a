use crate::Config;
use crate::api_v1::session::Claims;
use crate::db::cards::get_tree_cards;
use crate::db::trees::{add_tree, db_get_tree_all_data};
use crate::db_structs::trees::{TreeData, TreeWithCards};

use crate::errors::ApiError;

use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};

use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the trees category.
pub fn trees() -> Router<Config> {
    Router::new()
        .route("/add", post(add_new_tree))
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
