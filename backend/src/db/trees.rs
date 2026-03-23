use crate::{
    db_structs::trees::{TreeData, TreeGeneratedData, TreeStructure},
    errors::ApiError,
};
use std::collections::HashSet;

use crate::db_structs::trees::RootNode;
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
    tree_data: &TreeData,
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
        "INSERT INTO trees (owner_id, title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, structure)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
    new_tree_structure: &TreeStructure,
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

    // We check whether all cards exist in the `cards` table and link to the `tree_id`,
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
    use crate::db_structs::trees::Node;
    use crate::unsafe_get_connection;
    use sqlx::{PgConnection, PgPool};

    async fn initialize_db(db: &mut PgConnection) -> (i32, i32) {
        let user_id = initialize_user(&mut *db).await;
        let tree_id = initialize_empty_tree(&mut *db, user_id).await;

        (user_id, tree_id)
    }

    async fn initialize_user(db: impl PgExecutor<'_>) -> i32 {
        query!(
            "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING id",
            "email",
            "password_hash",
            "name"
        )
        .fetch_one(db)
        .await
        .unwrap()
        .id
    }

    async fn initialize_empty_tree(db: impl PgExecutor<'_>, owner_id: i32) -> i32 {
        let data = TreeData {
            title: "title".to_string(),
            description: None,
            book_name: None,
            book_edition: None,
            book_isbn: None,
            book_chapter: None,
            book_exercise: None,
            assignment_text: None,
            published: false,
        };

        let structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![],
            }],
        };

        initialize_tree(db, owner_id, &data, &structure).await
    }

    async fn initialize_tree(
        db: impl PgExecutor<'_>,
        owner_id: i32,
        tree_data: &TreeData,
        tree_structure: &TreeStructure,
    ) -> i32 {
        let structure = serde_json::to_value(tree_structure).unwrap();

        query!(
            "INSERT INTO trees (owner_id, published, title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, structure) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING id",
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
            structure,
        )
        .fetch_one(db)
        .await
        .unwrap()
        .id
    }

    async fn initialize_card(db: impl PgExecutor<'_>, tree_id: i32) -> i32 {
        query!(
            "INSERT INTO cards (tree_id, back) VALUES ($1, $2) RETURNING id",
            tree_id,
            "back"
        )
        .fetch_one(db)
        .await
        .unwrap()
        .id
    }

    #[sqlx::test]
    async fn test_add_tree(db: PgPool) {
        let user_id = initialize_user(&db).await;

        let tree_data = TreeData {
            title: "title".to_owned(),
            description: Some("desc".to_owned()),
            book_name: Some("book_name".to_owned()),
            book_edition: Some("book_edition".to_owned()),
            book_isbn: Some("book_isbn".to_owned()),
            book_chapter: Some("book_chapter".to_owned()),
            book_exercise: Some("book_exercise".to_owned()),
            assignment_text: Some("assignment_text".to_owned()),
            published: true,
        };

        let tree_id = add_tree(&db, &tree_data, user_id).await;
        assert!(tree_id.is_ok());

        let tree_id = tree_id.unwrap();
        let all_found_tree_data = query!("SELECT * FROM trees WHERE id = $1", tree_id).fetch_one(&db).await.unwrap();
        assert_eq!(all_found_tree_data.owner_id, user_id);
        
        let found_tree_data = TreeData {
            title: all_found_tree_data.title,
            description: all_found_tree_data.description,
            book_name: all_found_tree_data.book_name,
            book_edition: all_found_tree_data.book_edition,
            book_isbn: all_found_tree_data.book_isbn,
            book_chapter: all_found_tree_data.book_chapter,
            book_exercise: all_found_tree_data.book_exercise,
            assignment_text: all_found_tree_data.assignment_text,
            published: all_found_tree_data.published,
        };
        
        assert_eq!(found_tree_data, tree_data);
        
        let tree_structure = serde_json::from_value::<TreeStructure>(all_found_tree_data.structure);
        assert!(tree_structure.is_ok());
    }

    #[sqlx::test]
    async fn test_update_tree_structure(db: PgPool) {
        let (_, tree_id) = initialize_db(unsafe_get_connection!(db)).await;

        let card_id = initialize_card(unsafe_get_connection!(db), tree_id).await;

        let new_tree_structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![Node {
                    card_id,
                    children: vec![],
                }],
            }],
        };

        let void = update_tree_structure(&db, tree_id, &new_tree_structure).await;
        assert!(void.is_ok());

        let updated_tree_structure = query!("SELECT structure FROM trees WHERE id = $1", tree_id,)
            .fetch_one(&db)
            .await;
        assert!(updated_tree_structure.is_ok());

        let updated_tree_structure =
            serde_json::from_value::<TreeStructure>(updated_tree_structure.unwrap().structure)
                .unwrap();

        assert_eq!(new_tree_structure, updated_tree_structure);
    }

    #[sqlx::test]
    async fn test_db_get_tree_all_data(db: PgPool) {
        let user_id = initialize_user(&db).await;

        let tree_data = TreeData {
            title: "title".to_owned(),
            description: Some("desc".to_owned()),
            book_name: Some("book_name".to_owned()),
            book_edition: Some("book_edition".to_owned()),
            book_isbn: Some("book_isbn".to_owned()),
            book_chapter: Some("book_chapter".to_owned()),
            book_exercise: Some("book_exercise".to_owned()),
            assignment_text: Some("assignment_text".to_owned()),
            published: true,
        };

        let structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![],
            }],
        };

        let tree_id = initialize_tree(&db, user_id, &tree_data, &structure).await;

        let all_tree_data = db_get_tree_all_data(&db, tree_id).await;
        assert!(all_tree_data.is_ok());

        let all_tree_data = all_tree_data.unwrap();
        assert!(all_tree_data.is_some());

        let (found_tree_data, found_tree_gen_data, found_tree_structure) = all_tree_data.unwrap();
        assert_eq!(tree_data, found_tree_data);
        assert_eq!(tree_id, found_tree_gen_data.id);
        assert_eq!(user_id, found_tree_gen_data.owner_id);
        assert_eq!(structure, found_tree_structure);
    }

    #[sqlx::test]
    async fn test_get_tree_owner(db: PgPool) {
        let (user_id, tree_id) = initialize_db(unsafe_get_connection!(db)).await;

        let found_owner_id = get_tree_owner(&db, tree_id).await.unwrap();
        assert_eq!(found_owner_id, Some(user_id));
    }

    #[sqlx::test]
    async fn test_is_structure_valid(db: PgPool) {
        let (_, tree_id) = initialize_db(unsafe_get_connection!(db)).await;

        // Structure containing non-existent card_id should yield `false`.
        let invalid_structure_with_missing_card = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![Node {
                    card_id: 1,
                    children: vec![],
                }],
            }],
        };

        assert!(
            !is_structure_valid(&db, tree_id, &invalid_structure_with_missing_card)
                .await
                .unwrap()
        );

        // Happy path should yield `true`.
        let card_id = initialize_card(&db, tree_id).await;

        let valid_structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![Node {
                    card_id,
                    children: vec![],
                }],
            }],
        };

        assert!(
            is_structure_valid(&db, tree_id, &valid_structure)
                .await
                .unwrap()
        );

        // Structure containing doubled card should yield `false`.
        let invalid_structure_with_double_card = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![
                    Node {
                        card_id,
                        children: vec![],
                    },
                    Node {
                        card_id,
                        children: vec![],
                    },
                ],
            }],
        };

        assert!(
            !is_structure_valid(&db, tree_id, &invalid_structure_with_double_card)
                .await
                .unwrap()
        );
    }
}
