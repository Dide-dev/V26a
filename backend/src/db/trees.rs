use crate::db_structs::trees::TreeData;

use anyhow::{Error, Ok};
use sqlx::{PgPool, query};

/// Add a new tree to the database, returning its assigned index.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
/// `tree_data` - The data assigned to the new tree
/// `owner_id` - The id of the owner of the tree
///
/// # Returns
/// Returns the index assigned to the new tree.
pub async fn add_tree(db: &PgPool, tree_data: TreeData, owner_id: i32) -> Result<i32, Error> {
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
        .await?;

    Ok(id_record.id)
}
