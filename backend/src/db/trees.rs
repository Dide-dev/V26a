use crate::{
    db_structs::trees::{TreeAllData, TreeData, TreeGeneratedData},
    errors::ApiError,
};

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
/// The index assigned to the new tree if successful, or an [`ApiError::DatabaseException`] if not.
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

/// Request all data of a specific tree from the database.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_id` - The id of the tree of which is queried.
///
/// # Returns
/// All data of the tree as a [`Some`] if the tree exists, or a [`None`] if it doesn't
/// if successful, or an [`ApiError::DatabaseException`] if not.
pub async fn db_get_tree_all_data(
    db: &PgPool,
    tree_id: i32,
) -> Result<Option<TreeAllData>, ApiError> {
    let field = query!(
        "SELECT *
        FROM trees
        WHERE id = $1",
        tree_id,
    )
    .fetch_optional(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    let Some(field) = field else {
        return Ok(None);
    };

    let tree_structure =
        serde_json::from_value(field.structure).map_err(|_| ApiError::DatabaseException)?;

    let tree_data = TreeData {
        title: field.title,
        description: field.description,
        book_name: field.book_name,
        book_edition: field.book_edition,
        book_isbn: field.book_isbn,
        book_chapter: field.book_chapter,
        book_exercise: field.book_exercise,
        assignment_text: field.assignment_text,
        published: field.published,
    };

    let tree_gen_data = TreeGeneratedData {
        id: field.id,
        owner_id: field.owner_id,
        created_at: field.created_at,
        last_edited: field.last_edited,
        times_visited: field.times_visited,
        last_visited: field.last_visited,
    };

    let tree_all_data = TreeAllData {
        data: tree_data,
        gen_data: tree_gen_data,
        structure: tree_structure,
    };

    Ok(Some(tree_all_data))
}

/// Get the id of the owner of the tree.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_id` - The id of the tree of which the owner is queried.
///
/// # Returns
/// The index of the owner of the tree as a [`Some`] if the tree exists, or a [`None`] if it doesn't
/// if successful, or an [`ApiError::DatabaseException`] if not.
pub async fn get_tree_owner(db: &PgPool, tree_id: i32) -> Result<Option<i32>, ApiError> {
    let owner = query!("SELECT owner_id FROM trees WHERE id = $1", tree_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .map(|r| r.owner_id);

    Ok(owner)
}
