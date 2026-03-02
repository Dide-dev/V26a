use crate::trees::TreeData;

use crate::errors::ApiError;
use sqlx::{PgPool, query};

/// Add a new tree to the database, returning its assigned index.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_data` - The data assigned to the new tree as [`TreeData`]
///
/// `owner_id` - The id of the owner of the tree
///
/// # Returns
/// The index assigned to the new tree if successful, or an [`ApiError`] if not.
pub async fn add_tree(db: &PgPool, tree_data: TreeData, owner_id: i32) -> Result<i32, ApiError> {
    let id_record = query!(
        "INSERT INTO trees (owner_id, title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, structure)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, '{}')
        RETURNING id",
        owner_id,
        tree_data.title,
        tree_data.description,
        tree_data.book_name,
        tree_data.book_edition,
        tree_data.book_isbn,
        tree_data.book_chapter,
        tree_data.book_exercise,
        tree_data.assignment_text,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(id_record.id)
}
