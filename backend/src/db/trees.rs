use crate::{
    db_structs::trees::{TreeData, TreeGeneratedData, TreeStructure},
    errors::ApiError,
};

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
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, '{\"rootNodes\": []}')
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

/// Request all data of a specific tree from the database.
///
/// # Arguments
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_id` - The id of the tree of which is queried.
///
/// # Returns
/// All data of the tree as a [`Some`] if the tree exists, or a [`None`] if it doesn't.
///
/// # Error
/// May return an [`ApiError::DatabaseException`] instead.
pub async fn db_get_tree_all_data(
    db: &PgPool,
    tree_id: i32,
) -> Result<Option<(TreeData, TreeGeneratedData, TreeStructure)>, ApiError> {
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
        last_modified: field.last_modified,
        times_visited: field.times_visited,
        last_visited: field.last_visited,
    };

    Ok(Some((tree_data, tree_gen_data, tree_structure)))
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
async fn get_tree_owner(db: &PgPool, tree_id: i32) -> Result<Option<i32>, ApiError> {
    let owner = query!("SELECT owner_id FROM trees WHERE id = $1", tree_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .map(|r| r.owner_id);

    Ok(owner)
}

/// Checks whether the tree exists and if it is owner by the user.
///
/// # Params
/// `db` - A reference to the database connection to use for the query.
///
/// `tree_id` - The tree of which the ownerships gets verified.
///
/// `user_id` - The id of the user who sent this request.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns an [`ApiError::MissingTree`] if no tree with `tree_id` exists.
///
/// Returns an [`ApiError::NoPermission`] if the `user_id` does not match the `owner_id` field
/// for the given `tree_id`.
pub async fn verify_tree_ownership(
    db: &PgPool,
    tree_id: i32,
    user_id: i32,
) -> Result<(), ApiError> {
    // Check whether the tree which the card is being added to exists.
    let owner_id = get_tree_owner(db, tree_id)
        .await?
        .ok_or(ApiError::MissingTree(tree_id))?;

    // Check whether the user is authorized to add a card to the tree.
    if user_id != owner_id {
        return Err(ApiError::NoPermission);
    }

    Ok(())
}
