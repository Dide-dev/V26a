use crate::db_structs::trees::TreeData;

use crate::errors::ApiError;
use sqlx::{PgPool, query};

/// Add a new tree to the database.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_data` - The data assigned to the new tree as [`TreeData`].
///
/// `owner_id` - The id of the owner of the tree.
///
/// # Returns
/// The id assigned to the new tree.
///
/// # Error
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn add_tree(db: &PgPool, tree_data: TreeData, owner_id: i32) -> Result<i32, ApiError> {
    let tree_id = query!(
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
    .map_err(|_| ApiError::DatabaseException)?
    .id;

    Ok(tree_id)
}

/// Get the id of the owner of the tree.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_id` - The id of the tree of which the owner is queried.
///
/// # Returns
/// The id of the owner of the tree as a [`Some`] if the tree exists, or a [`None`] if it doesn't.
///
/// # Error
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn get_tree_owner(db: &PgPool, tree_id: i32) -> Result<Option<i32>, ApiError> {
    let owner = query!("SELECT owner_id FROM trees WHERE id = $1", tree_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .map(|r| r.owner_id);

    Ok(owner)
}
