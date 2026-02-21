use crate::db::trees::add_tree;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Router};
use sqlx::PgPool;

pub fn trees() -> Router {
    Router::new().route("/add_new_tree", post(add_new_tree))
}

#[derive(serde::Deserialize)]
pub struct Tree {
    owner_id: i32,
    title: String,
}

/// Add a tree to the database
/// Returns the index assigned to the new tree
async fn add_new_tree(
    db: Extension<PgPool>,
    axum::Json(tree): axum::Json<Tree>,
) -> Result<(), StatusCode> {
    let _id = add_tree(&db, tree.owner_id, &tree.title)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
