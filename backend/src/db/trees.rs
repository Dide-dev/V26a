/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::pagination::PaginationOptions;
use crate::{
    db_structs::trees::{
        TreeAddData, TreeData, TreeFilters, TreeGeneratedData, TreeList, TreeMetadata,
        TreeStructure,
    },
    errors::ApiError,
};
use sqlx::{PgExecutor, Postgres, QueryBuilder, query};
use std::collections::HashSet;

/// Add a new tree to the database.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `tree_data` - The data assigned to the new tree as [`TreeData`].
///
/// `tree_structure` - The tree structure with cards as [`TreeStructure`].
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
    tree_add_data: &TreeAddData,
    owner_id: i32,
) -> Result<i32, ApiError> {
    let tree_structure = serde_json::to_value(&tree_add_data.tree_structure)
        .map_err(|_| ApiError::SerdeException)?;
    let tree_data = &tree_add_data.tree_data;

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
        tree_structure,
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

    // We check whether all cards exist in the `cards` table and link to the `tree_id`,
    // by verifying each of the `card_id`'s is contained in `db_card_ids`.
    let all_cards_exist_in_database = structure_card_ids.iter().all(|i| db_card_ids.contains(i));

    // We check whether any duplicates exist, by converting the Vec<> into a HashSet<>
    // and verifying their length stays the same.
    let hash_structure_card_ids = structure_card_ids.iter().collect::<HashSet<_>>();
    let no_duplicates_in_structure = structure_card_ids.len() == hash_structure_card_ids.len();

    Ok(all_cards_exist_in_database && no_duplicates_in_structure)
}

/// Adds [`TreeFilters`] to a Postgres [`QueryBuilder`]
///
/// # Arguments
/// `builder` - The [`QueryBuilder`] to be mutated in-place.
///
/// `filters` - The [`TreeFilters`] to apply to the query.
///
/// `user_id` - The logged in user performing the filter, optional.
///
/// # Example:
/// ```rs
/// let mut query = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM trees");
/// apply_tree_filters(query, &filters);
/// let trees_count: i64 = query
///         .build_query_scalar()
///         .fetch_one(db)
///         .await
///         .map_err(|_| ApiError::DatabaseException)?;
/// ```
///
/// # To future developers:
/// This function can and will become too big, good luck if that happens!
fn apply_tree_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    filters: &'a TreeFilters,
    user_id: Option<i32>,
) {
    builder.push(" WHERE ");
    let mut separated = builder.separated(" AND ");
    separated.push("(published = true");
    if let Some(user_id) = user_id
        && filters.show_own_private
    {
        separated.push_unseparated(" OR (published = false AND owner_id = ");
        separated.push_bind_unseparated(user_id);
        separated.push_unseparated(")");
    }
    separated.push_unseparated(")");
    if let Some(owner_id) = filters.owner_id {
        separated
            .push("(owner_id = ")
            .push_bind_unseparated(owner_id)
            .push_unseparated(")");
    }
    if !filters.book.is_empty() {
        separated
            .push("(book_name = ANY(")
            .push_bind_unseparated(&filters.book)
            .push_unseparated("))");
    }
    if let Some(created_after) = filters.created_after {
        separated
            .push("(created_at >= ")
            .push_bind_unseparated(created_after)
            .push_unseparated(")");
    }
    if let Some(created_before) = filters.created_before {
        separated
            .push("(created_at <= ")
            .push_bind_unseparated(created_before)
            .push_unseparated(")");
    }
}

/// Get the amount of trees satisfying some filters.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `filters` - The filters to apply to the trees.
///
/// `user_id` - The logged in user performing the filter, optional.
///
/// # Returns
/// The amount of trees satisfying the filters at the requested page,
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_count(
    db: impl PgExecutor<'_>,
    filters: &TreeFilters,
    user_id: Option<i32>,
) -> Result<i64, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM trees");

    apply_tree_filters(&mut query, filters, user_id);

    let trees_count: i64 = query
        .build_query_scalar()
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    Ok(trees_count)
}

/// Get the list of all trees satisfying some filters at a given page.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `page` - Which page is requested.
///
/// `filters` - The filters to apply to the trees.
///
/// `user_id` - The logged in user performing the filter, optional.
///
/// # Returns
/// A list with the trees satisfying the filters at the requested page,
/// together with a boolean if there exists a next page.
///
/// # Error
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_list(
    db: impl PgExecutor<'_>,
    page: &PaginationOptions,
    filters: &TreeFilters,
    user_id: Option<i32>,
) -> Result<TreeList, ApiError> {
    // Prevent overflow errors
    if !page.input_is_valid() {
        return Err(ApiError::InvalidPage);
    }

    let mut query = QueryBuilder::<Postgres>::new("
        SELECT title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, published
        FROM trees"
    );

    apply_tree_filters(&mut query, filters, user_id);
    query
        .push(
            // newest first
            " ORDER BY created_at DESC \
              LIMIT ",
        )
        // Return an extra entry to the API so it can efficiently know if there is a next page.
        .push_bind(i64::from(page.size + 1))
        .push(" OFFSET ")
        // Page is 1-based
        .push_bind(i64::from((page.page - 1) * page.size));

    let mut trees = query
        .build_query_as::<TreeData>()
        .fetch_all(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    // User requested a page that yields no results. This should only occur if the user requests a
    // page that is too high, as get_queried_count should be used to confirm results exist.
    if trees.is_empty() {
        return Err(ApiError::InvalidPage);
    }

    // Check if there is a next page to be requested by the client, and remove the extra added entry if there is.
    // The "unwrap" should not ever trigger an exception, as it is checked to be > 0 at the start of the function.
    let has_next = trees.len() > usize::try_from(page.size).unwrap();
    if has_next {
        trees.pop();
    }

    Ok(TreeList { has_next, trees })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::test::{
        initialize_basic_user, initialize_card, initialize_db, initialize_tree_with_data,
    };
    use crate::db_structs::trees::{Node, RootNode};
    use crate::get_connection_unwrap;
    use sqlx::{PgPool, query_as};

    #[sqlx::test]
    async fn test_add_tree(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

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

        let tree_add_data = TreeAddData {
            tree_data: tree_data.clone(),
            tree_structure: TreeStructure { root_nodes: vec![] },
        };

        let tree_id = add_tree(&db, &tree_add_data, user_id).await.unwrap();

        let found_owner_id = query!("SELECT owner_id FROM trees WHERE id = $1", tree_id)
            .fetch_one(&db)
            .await
            .unwrap()
            .owner_id;

        assert_eq!(found_owner_id, user_id);

        let found_tree_data = query_as!(TreeData, "SELECT title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, published FROM trees WHERE id = $1", tree_id)
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(found_tree_data, tree_data);

        let found_structure = query!("SELECT structure FROM trees WHERE id = $1", tree_id)
            .fetch_one(&db)
            .await
            .unwrap()
            .structure;

        serde_json::from_value::<TreeStructure>(found_structure).unwrap();
    }

    #[sqlx::test]
    async fn test_update_tree_structure(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;

        let card_id = initialize_card(get_connection_unwrap!(db), tree_id).await;

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
        assert_eq!(void, Ok(()));

        let updated_tree_structure = query!("SELECT structure FROM trees WHERE id = $1", tree_id)
            .fetch_one(&db)
            .await
            .unwrap();

        let updated_tree_structure: TreeStructure =
            serde_json::from_value(updated_tree_structure.structure).unwrap();

        assert_eq!(new_tree_structure, updated_tree_structure);
    }

    #[sqlx::test]
    async fn test_db_get_tree_all_data(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

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

        let tree_id = initialize_tree_with_data(&db, user_id, &tree_data, &structure).await;

        let (found_tree_data, found_tree_gen_data, found_tree_structure) =
            db_get_tree_all_data(&db, tree_id).await.unwrap().unwrap();

        assert_eq!(tree_data, found_tree_data);
        assert_eq!(tree_id, found_tree_gen_data.id);
        assert_eq!(user_id, found_tree_gen_data.owner_id);
        assert_eq!(structure, found_tree_structure);
    }

    #[sqlx::test]
    async fn test_get_tree_owner(db: PgPool) {
        let (user_id, tree_id) = initialize_db(get_connection_unwrap!(db)).await;

        let found_owner_id = get_tree_owner(&db, tree_id).await.unwrap();
        assert_eq!(found_owner_id, Some(user_id));
    }

    #[sqlx::test]
    async fn test_is_structure_valid(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;

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

    #[test]
    fn test_filter_query_builder() {
        use chrono::{DateTime, Utc};

        let mut no_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let empty_filters = TreeFilters {
            ..Default::default()
        }; // TODO: test user_id
        apply_tree_filters(&mut no_filters_query, &empty_filters, None);
        assert_eq!(
            no_filters_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );

        let mut all_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let all_filters = TreeFilters {
            owner_id: Some(67),
            book: vec!["1984".to_owned()],
            created_after: Some(DateTime::<Utc>::MIN_UTC),
            created_before: Some(DateTime::<Utc>::MAX_UTC),
            show_own_private: true,
        };
        apply_tree_filters(&mut all_filters_query, &all_filters, Some(123));
        assert_eq!(
            all_filters_query.sql(),
            "SELECT * FROM trees WHERE (published = true OR (published = false AND owner_id = $1)) AND (owner_id = $2) AND (book_name = ANY($3)) AND (created_at >= $4) AND (created_at <= $5)"
        );

        let mut partial_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let partial_filters = TreeFilters {
            owner_id: Some(67),
            created_after: Some(DateTime::<Utc>::MIN_UTC),
            ..Default::default()
        };
        apply_tree_filters(&mut partial_filters_query, &partial_filters, None);
        assert_eq!(
            partial_filters_query.sql(),
            "SELECT * FROM trees WHERE (published = true) AND (owner_id = $1) AND (created_at >= $2)"
        );

        let mut show_private_no_id_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let show_private_no_id_filters = TreeFilters {
            show_own_private: true,
            ..Default::default()
        };
        apply_tree_filters(
            &mut show_private_no_id_query,
            &show_private_no_id_filters,
            None,
        );
        assert_eq!(
            show_private_no_id_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );

        let mut hide_private_with_id_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let show_private_no_id_filters = TreeFilters {
            show_own_private: false,
            ..Default::default()
        };
        apply_tree_filters(
            &mut hide_private_with_id_query,
            &show_private_no_id_filters,
            Some(123),
        );
        assert_eq!(
            hide_private_with_id_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );
    }

    /// TODO: rewrite test with initialize functions
    #[sqlx::test]
    async fn test_update_tree_data(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;

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
        let tree_structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "RootNode".to_string(),
                children: vec![],
            }],
        };
        let tree_add_data = TreeAddData {
            tree_data,
            tree_structure,
        };
        let tree_id = add_tree(&db, &tree_add_data, user_id).await.unwrap();

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
