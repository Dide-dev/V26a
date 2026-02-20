use crate::db::trees::add_tree;

use axum::Extension;
use axum::http::StatusCode;
use sqlx::PgPool;

/// Returns the index assigned to the new tree
pub fn add_new_tree(db: Extension<PgPool>) -> Result<i32, StatusCode> {
    todo!()
}
