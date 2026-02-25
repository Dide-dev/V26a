use crate::Config;
use crate::api_v1::session::Claims;
use crate::db::trees::add_tree;
use crate::trees::TreeData;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the trees category.
pub fn trees() -> Router<Config> {
    Router::new().route("/add-new-tree", post(add_new_tree))
}

/// Add a tree to the database.
/// Returns the index assigned to the new tree.
async fn add_new_tree(
    claims: Claims,
    db: Extension<PgPool>,
    axum::Json(tree_data): axum::Json<TreeData>,
) -> Result<Json<i32>, StatusCode> {
    let id = add_tree(&db, tree_data, claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json_id = Json(id);
    Ok(json_id)
}
