/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::{
    db_structs::trees::{TreeData, TreeGeneratedData, TreeStructure},
    errors::ApiError,
};
use std::collections::HashSet;

use crate::db_structs::trees::{RootNode, TreeMetadata};
use sqlx::{PgExecutor, query};

/// Add a new tree to the database.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `tree_data` - The data assigned to the new tree as [`TreeData`].
///
/// `owner_id` - The id of the owner of the tree.
///
/// # Returns
/// The id assigned to the new tree.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_tree(
    db: impl PgExecutor<'_>,
    tree_data: TreeData,
    owner_id: i32,
) -> Result<i32, ApiError> {
    // TODO: This is hardcoded for now, but should come from the front-end along with the api-call.
    let structure = serde_json::to_value(TreeStructure {
        root_nodes: vec![
            RootNode {
                label: "Orientation".to_owned(),
                children: vec![],
            },
            RootNode {
                label: "Making & executing plans".to_owned(),
                children: vec![],
            },
            RootNode {
                label: "Completion".to_owned(),
                children: vec![],
            },
        ],
    })
    .map_err(|_| ApiError::SerdeException)?;

    let tree_id = query!(
        "INSERT INTO trees (owner_id, published, title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, structure)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id",
        owner_id,
        tree_data.published,
        tree_data.title,
        tree_data.description,
        tree_data.book_name,
        tree_data.book_edition,
        tree_data.book_isbn,
        tree_data.book_chapter,
        tree_data.book_exercise,
        tree_data.assignment_text,
        structure
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .id;

    Ok(tree_id)
}

/// Update the structure of an existing tree in the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `tree_id` - The id of the tree which is updated.
///
/// `new_tree_structure` - The new structure of the tree as a [`TreeStructure`].
/// Completely overwrites the existing data in the database.
///
/// # Returns
/// Nothing.
///
/// # Error
/// May return a [`SerdeException`](ApiError::SerdeException) or a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_tree_structure(
    db: impl PgExecutor<'_>,
    tree_id: i32,
    new_tree_structure: TreeStructure,
) -> Result<(), ApiError> {
    let new_tree_structure =
        serde_json::to_value(new_tree_structure).map_err(|_| ApiError::SerdeException)?;

    query!(
        "UPDATE trees SET (last_modified, structure) = (current_timestamp, $2) WHERE id = $1",
        tree_id,
        new_tree_structure,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// Update the data of an existing tree in the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `tree_id` - The id of the tree which is updated.
///
/// `new_tree_data` - The new data of the tree as [`TreeData`].
/// Completely overwrites the existing data in the database.
///
/// # Returns
/// Nothing.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_tree_data(
    db: impl PgExecutor<'_>,
    tree_id: i32,
    new_tree_data: TreeData,
) -> Result<(), ApiError> {
    query!(
        "UPDATE trees SET
        (last_modified, title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, published)
        = (current_timestamp, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        WHERE id = $1",
        tree_id,
        new_tree_data.title,
        new_tree_data.description,
        new_tree_data.book_name,
        new_tree_data.book_edition,
        new_tree_data.book_isbn,
        new_tree_data.book_chapter,
        new_tree_data.book_exercise,
        new_tree_data.assignment_text,
        new_tree_data.published,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// Request all data of a specific tree from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `tree_id` - The id of the tree which is queried.
///
/// # Returns
/// All data of the tree as a [`Some`] if the tree exists, or a [`None`] if it doesn't.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn db_get_tree_all_data(
    db: impl PgExecutor<'_>,
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
        serde_json::from_value(field.structure).map_err(|_| ApiError::SerdeException)?;

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
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `tree_id` - The id of the tree of which the owner is queried.
///
/// # Returns
/// The id of the owner of the tree as a [`Some`] if the tree exists, or a [`None`] if it doesn't.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_tree_owner(
    db: impl PgExecutor<'_>,
    tree_id: i32,
) -> Result<Option<i32>, ApiError> {
    let owner = query!("SELECT owner_id FROM trees WHERE id = $1", tree_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .map(|r| r.owner_id);

    Ok(owner)
}

/// Get the list of all trees owned by some owner.
/// Should only be called if the user is authorised to see private trees!
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `owner_id` - The id of the owner that is queried on.
///
/// # Returns
/// A [`Vec<TreeMetadata>`] containing the metadata of all trees owned by the owner (could be empty).
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_all_trees_by_owner(
    db: impl PgExecutor<'_>,
    owner_id: i32,
) -> Result<Vec<TreeMetadata>, ApiError> {
    let records = query!("SELECT * FROM trees WHERE owner_id = $1", owner_id)
        .fetch_all(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    let trees_metadata = records
        .into_iter()
        .map(|t| TreeMetadata {
            data: TreeData {
                title: t.title,
                description: t.description,
                book_name: t.book_name,
                book_edition: t.book_edition,
                book_isbn: t.book_isbn,
                book_chapter: t.book_chapter,
                book_exercise: t.book_exercise,
                assignment_text: t.assignment_text,
                published: t.published,
            },
            gen_data: TreeGeneratedData {
                id: t.id,
                owner_id: t.owner_id,
                created_at: t.created_at,
                last_modified: t.last_modified,
                times_visited: t.times_visited,
                last_visited: t.last_visited,
            },
        })
        .collect::<Vec<TreeMetadata>>();

    Ok(trees_metadata)
}

/// Checks whether the cards in a tree structure all exist in the `Cards` table, and link to the tree
/// and whether any duplicate cards exist in the tree structure.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `tree_id` - The id of the tree to check.
///
/// `tree_structure` - The structure to check as a [`TreeStructure`].
///
/// # Returns
/// `true` if both of the following hold:
/// * Each `card_id` in the structure exists in the `cards` table and has `tree_id` in the
///   corresponding column.
/// * No duplicate cards exist within the tree structure.
///
/// `false` if either of these don't hold.
///
/// # Error
/// Returns [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) if more than 9,999 cards are found which link to `tree_id`.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn is_structure_valid(
    db: impl PgExecutor<'_>,
    tree_id: i32,
    tree_structure: &TreeStructure,
) -> Result<bool, ApiError> {
    // We retrieve the card id's in the new structure.
    let structure_card_ids = tree_structure.flatten();

    let db_card_limit: u32 = 10_000;

    // We retrieve the card id's linked to the tree from the database.
    // The limit of 10,000 is arbitrarily set to avoid excessive memory usage.
    let db_card_ids = query!(
        "SELECT id FROM cards WHERE tree_id = $1 LIMIT $2",
        tree_id,
        i64::from(db_card_limit),
    )
    .fetch_all(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .iter()
    .map(|n| n.id)
    .collect::<HashSet<_>>();

    if db_card_ids.len() >= db_card_limit as usize {
        return Err(ApiError::DatabaseMemoryCap);
    }

    // We check whether all cards exist in the `Cards` table and link to the `tree_id`,
    // by verifying each of the `card_id`'s is contained in `db_card_ids`.
    let all_cards_exist_in_database = structure_card_ids.iter().all(|i| db_card_ids.contains(i));

    // We check whether any duplicates exist, by converting the Vec<> into a HashSet<>
    // and verifying their length stays the same.
    let hash_structure_card_ids = structure_card_ids.iter().collect::<HashSet<_>>();
    let no_duplicates_in_structure = structure_card_ids.len() == hash_structure_card_ids.len();

    Ok(all_cards_exist_in_database && no_duplicates_in_structure)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::users::add_user;
    use crate::db_structs::users::{UserData, UserLanguage};
    use sqlx::PgPool;

    /// TODO: rewrite test with initialize functions
    #[sqlx::test]
    async fn test_update_tree_data(db: PgPool) {
        let mut conn = db.acquire().await.unwrap();

        // Create an account

        let user_id = add_user(
            &mut conn,
            UserData {
                email: "test123@gmail.com".to_string(),
                password: "monkey".to_string(),
                name: "User123".to_string(),
                nickname: None,
                about_me: None,
                email_visible: false,
                language: UserLanguage::En,
            },
        )
        .await
        .unwrap();

        // Add a tree to the database
        let tree_data = TreeData {
            title: "Test tree".to_string(),
            description: None,
            book_name: None,
            book_edition: None,
            book_isbn: None,
            book_chapter: None,
            book_exercise: None,
            assignment_text: None,
            published: false,
        };
        let tree_id = add_tree(&db, tree_data, user_id).await.unwrap();

        // Update the tree data
        let new_tree_data = TreeData {
            title: "Updated tree".to_string(),
            description: Some("Updated description".to_string()),
            book_name: Some("Updated book".to_string()),
            book_edition: Some("2nd".to_string()),
            book_isbn: Some("9780136019701".to_string()),
            book_chapter: Some("3".to_string()),
            book_exercise: Some("5b".to_string()),
            assignment_text: Some("Updated assignment".to_string()),
            published: true,
        };
        update_tree_data(&db, tree_id, new_tree_data.clone())
            .await
            .unwrap();

        // Verify the data was updated
        let (found_tree_data, _, _) = db_get_tree_all_data(&db, tree_id).await.unwrap().unwrap();
        assert_eq!(found_tree_data, new_tree_data);
    }
}
