use crate::Config;
use crate::api_v1::session::Claims;
use crate::db::trees::add_tree;
use crate::db_structs::trees::TreeData;

use crate::errors::ApiError;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the trees category.
pub fn trees() -> Router<Config> {
    Router::new().route("/add-new-tree", post(add_new_tree))
}

/// Add a tree to the database.
///
/// # Returns
/// The id assigned to the new tree as [`Json`] if successful, or an [`ApiError`] if not.
async fn add_new_tree(
    claims: Claims,
    db: Extension<PgPool>,
    Json(tree_data): Json<TreeData>,
) -> Result<Json<i32>, ApiError> {
    let id = add_tree(&db, tree_data, claims.sub).await?;

    let json_id = Json(id);
    Ok(json_id)
}
