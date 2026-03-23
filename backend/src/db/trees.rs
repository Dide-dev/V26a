use crate::api_v1::structs::PaginationOptions;
use crate::{
    db_structs::trees::{
        RootNode, TreeData, TreeFilters, TreeGeneratedData, TreeList, TreeMetadata, TreeStructure,
    },
    errors::ApiError,
};
use sqlx::{PgExecutor, query, query_as};
use std::collections::HashSet;

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

/// Get the list of all trees satisfying some filters at a given page.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `page` - Which page is requested.
///
/// `filters` - The filters to apply to the trees.
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
    filters: TreeFilters,
) -> Result<TreeList, ApiError> {
    // Prevent overflow errors
    if !page.input_is_valid() {
        return Err(ApiError::InvalidPage);
    }

    let mut trees = query_as!(
        TreeData,
        "SELECT title, description, book_name, book_edition, book_isbn, book_chapter, book_exercise, assignment_text, published
        FROM trees 
        WHERE published = true
        AND (owner_id = $3 OR $3 IS NULL)
        AND (book_name = ANY($4) OR CARDINALITY($4) = 0)
        AND (created_at >= $5 OR $5 IS NULL)
        AND (created_at <= $6 OR $6 IS NULL)
        ORDER BY created_at DESC -- newest first
        LIMIT $1 OFFSET $2",
        i64::from(page.size + 1), // Return an extra entry to the API so it can efficiently know if there is a next page.
        i64::from((page.page - 1) * page.size), // Page is 1-based
        filters.owner_id,
        &filters.book,
        filters.created_after,
        filters.created_before,
    ).fetch_all(db).await.map_err(|_| ApiError::DatabaseException)?;

    // User requested a page that yields no results
    // TODO: change to NoSuchTree error or something like that if page=1?
    if trees.is_empty() {
        return Err(ApiError::InvalidPage);
    }

    // Check if there is a next page to be requested by the client, and remove the extra added entry if there is.
    // The unwrap should not ever trigger an exception, as it is checked to be > 0 at the start of the function.
    let has_next = trees.len() > usize::try_from(page.size).unwrap();
    if has_next {
        trees.pop();
    }

    Ok(TreeList { has_next, trees })
}
