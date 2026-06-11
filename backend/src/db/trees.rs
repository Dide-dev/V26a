/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::pagination::PaginationOptions;
use crate::db_structs::session::Claims;
use crate::db_structs::users::AccountType;
use crate::{
    db_structs::trees::{
        Language, Subject, TreeAddData, TreeData, TreeFilters, TreeGeneratedData, TreeLinkedData,
        TreeMetadata, TreeStructure,
    },
    errors::ApiError,
};
use chrono::{DateTime, Utc};
use sqlx::query_builder::Separated;
use sqlx::{PgConnection, PgExecutor, Postgres, QueryBuilder, query, query_as};
use std::collections::{HashMap, HashSet};

/// Check whether the given subject exists.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `id` - The id of the subject whose existence to check.
///
/// # Returns
/// A `bool` indicating whether the given subject exists.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn subject_exists(db: impl PgExecutor<'_>, id: i32) -> Result<bool, ApiError> {
    query!("SELECT EXISTS (SELECT 1 FROM subjects WHERE id = $1)", id)
        .fetch_one(db)
        .await?
        .exists
        .ok_or(ApiError::DatabaseException)
}

/// Retrieve the list of all subjects from the database.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// # Returns
/// A map of all the possible tree subjects stored in the database indexed by id,
/// as [`HashMap<i32, Subject>`].
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_subjects(
    db: impl PgExecutor<'_>,
) -> Result<HashMap<String, Vec<Subject>>, ApiError> {
    let results = query_as!(Subject, "SELECT * FROM subjects")
        .fetch_all(db)
        .await?;
    let mut map: HashMap<String, Vec<Subject>> = HashMap::new();
    for subject in results {
        map.entry(subject.language_code.clone())
            .and_modify(|vec| vec.push(subject.clone()))
            .or_insert(vec![subject]);
    }
    Ok(map)
}

/// Check whether the given language exists.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `language_code` - The code of the language whose existence to check.
///
/// # Returns
/// A `bool` indicating whether the given language exists.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn language_exists(
    db: impl PgExecutor<'_>,
    language_code: &str,
) -> Result<bool, ApiError> {
    query!(
        "SELECT EXISTS (SELECT 1 FROM languages WHERE code = $1)",
        language_code
    )
    .fetch_one(db)
    .await?
    .exists
    .ok_or(ApiError::DatabaseException)
}

/// Retrieve the list of all languages from the database.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// # Returns
/// A list of all the possible tree languages stored in the database, as [`Vec<Language>`].
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_languages(db: impl PgExecutor<'_>) -> Result<Vec<Language>, ApiError> {
    let results = query_as!(Language, "SELECT * FROM languages")
        .fetch_all(db)
        .await?;

    Ok(results)
}

/// Parse an ISBN of either 10 or 13 digits, and return it as integer in the 13-digit
/// representation.
///
/// # Arguments
/// `isbn` - The ISBN to parse, as [`&str`].
///
/// # Returns
/// The given ISBN in the 13-digit format as integer.
///
/// # Errors
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if the given string is not a valid ISBN.
pub fn parse_isbn(isbn: &str) -> Result<i64, ApiError> {
    let check_digit_10 = |mut n: i64| -> i64 {
        let mut s = 0;
        for i in 0..9 {
            let d = n % 10;
            s += d * (i + 2);
            n /= 10;
        }
        (11 - s % 11) % 11
    };
    let check_digit_13 = |mut n: i64| -> i64 {
        let mut s = 0;
        n /= 10;
        for i in 0..12 {
            let d = n % 10;
            s += d * if i % 2 == 0 { 3 } else { 1 };
            n /= 10;
        }
        (10 - s % 10) % 10
    };

    let no_dashes = isbn.chars().filter(|c| *c != '-').collect::<String>();
    if no_dashes.len() == 10 {
        let given_check_digit: i64 = if no_dashes.as_bytes()[9] == b'X' {
            10
        } else {
            (no_dashes.as_bytes()[9] as char)
                .to_digit(10)
                .ok_or(ApiError::InvalidIsbn)?
                .into()
        };
        let num = no_dashes[..9]
            .parse::<i64>()
            .map_err(|_| ApiError::InvalidIsbn)?;
        let computed_check_digit = check_digit_10(num);
        if computed_check_digit != given_check_digit {
            return Err(ApiError::InvalidIsbn);
        }
        #[allow(clippy::inconsistent_digit_grouping)]
        let isbn13_base = 978_0_000_00000_0_i64 + 10 * num;
        Ok(isbn13_base + check_digit_13(isbn13_base))
    } else if no_dashes.len() == 13 {
        let num = no_dashes
            .parse::<i64>()
            .map_err(|_| ApiError::InvalidIsbn)?;
        if check_digit_13(num) != num % 10 {
            return Err(ApiError::InvalidIsbn);
        }
        Ok(num)
    } else {
        Err(ApiError::InvalidIsbn)
    }
}

/// Add a new book to the database. If a book with the given ISBN already exists, do nothing.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_data` - The [`TreeData`] which contains the book data.
///
/// # Returns
/// `true` if a book with the given ISBN already existed, `false` otherwise.
///
/// # Errors
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if the book's ISBN is invalid.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_book(db: impl PgExecutor<'_>, tree_data: &TreeData) -> Result<bool, ApiError> {
    // We pass a TreeData to this function because that's the only struct
    // that contains book info. If this does not contain a book, there's
    // nothing to do and we can return.
    if tree_data.book_isbn.is_none() {
        return Ok(false);
    }

    let result = query!(
        "WITH insertion AS (
            INSERT INTO books (isbn, title, author, edition)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            RETURNING FALSE AS exists
        )
        SELECT * FROM insertion
        UNION
        SELECT TRUE AS exists FROM books WHERE isbn = $1",
        tree_data
            .book_isbn
            .as_ref()
            .map(|isbn| parse_isbn(isbn))
            .transpose()?,
        tree_data.book_name,
        tree_data.book_author,
        tree_data.book_edition,
    )
    .fetch_one(db)
    .await?;

    result.exists.ok_or(ApiError::DatabaseException)
}

/// Add a list of tags for a given tree in the database.
///
/// # Arguments
/// `db` - A connection to the database, as [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree to assign the tags to.
///
/// `tags` - The list of tags as [`&[String]`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_tags(
    db: &mut PgConnection,
    tree_id: i32,
    tags: &[String],
) -> Result<(), ApiError> {
    query!(
        "INSERT INTO tags (name) SELECT UNNEST($1::VARCHAR(32)[]) ON CONFLICT DO NOTHING",
        tags
    )
    .execute(&mut *db)
    .await?;

    // This needs to be queried separately because the previous query doesn't insert all of the tags
    // if there are duplicates.
    let tag_ids = query!(
        "SELECT id FROM UNNEST($1::VARCHAR(32)[]) AS DATA(tag_name) JOIN tags ON name = tag_name",
        tags
    )
    .fetch_all(&mut *db)
    .await?;

    query!(
        "INSERT INTO tree_tags (tree_id, tag_id) SELECT $1 AS tree_id, tag_id FROM UNNEST($2::INTEGER[]) AS DATA(tag_id)",
        tree_id,
        &tag_ids.iter().map(|r| r.id).collect::<Vec<_>>()
    )
    .execute(&mut *db)
    .await?;

    Ok(())
}

/// Add a new tree to the database.
///
/// Also adds a new book, if the referenced book ISBN does not yet exist.
///
/// # Arguments
/// `db` - A connection to the database, as [`&mut PgConnection`](PgConnection).
///
/// `tree_add_data` - The data assigned to the new tree as [`TreeAddData`].
///
/// `owner_id` - The id of the owner of the tree.
///
/// # Returns
/// The id assigned to the new tree.
///
/// # Errors
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if the book's ISBN is invalid.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_tree(
    db: &mut PgConnection,
    tree_add_data: &TreeAddData,
    owner_id: i32,
) -> Result<i32, ApiError> {
    let tree_structure = serde_json::to_value(&tree_add_data.tree_structure)?;
    let tree_data = &tree_add_data.tree_data;

    add_book(&mut *db, tree_data).await?;

    let tree_id = query!(
        "INSERT INTO trees (owner_id, published, title, description, language_code, subject_id, book_isbn, book_chapter, book_exercise, assignment_text, structure)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id",
        owner_id,
        tree_data.published,
        tree_data.title,
        tree_data.description,
        tree_data.language_code,
        tree_data.subject_id,
        tree_data.book_isbn.as_ref().map(|isbn| parse_isbn(isbn)).transpose()?,
        tree_data.book_chapter,
        tree_data.book_exercise,
        tree_data.assignment_text,
        tree_structure,
    )
    .fetch_one(&mut *db)
    .await?
    .id;

    add_tags(db, tree_id, &tree_data.tags).await?;

    Ok(tree_id)
}

/// Removes a tree and all of its cards from the database.
/// Does **not** check if the tree existed in the first place.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - the id of the tree to remove
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn remove_tree(db: impl PgExecutor<'_>, tree_id: i32) -> Result<(), ApiError> {
    // When this tree is deleted from database, the cards are automatically deleted with it.
    // This is because of the `ON DELETE CASCADE` in the foreign key of the 'cards' table.
    query!("DELETE FROM trees WHERE id=$1", tree_id)
        .execute(db)
        .await?;

    Ok(())
}

/// Update the structure of an existing tree in the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree which is updated.
///
/// `new_tree_structure` - The new structure of the tree as a [`TreeStructure`].
/// Completely overwrites the existing data in the database.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`SerdeException`](ApiError::SerdeException) or a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_tree_structure(
    db: impl PgExecutor<'_>,
    tree_id: i32,
    new_tree_structure: &TreeStructure,
) -> Result<(), ApiError> {
    let new_tree_structure = serde_json::to_value(new_tree_structure)?;

    query!(
        "UPDATE trees SET (last_modified, structure) = (current_timestamp, $2) WHERE id = $1",
        tree_id,
        new_tree_structure,
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Update the data of an existing tree in the database.
///
/// # Arguments
/// `db` - A connection to the database, as [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree which is updated.
///
/// `new_tree_data` - The new data of the tree as [`TreeData`].
/// Completely overwrites the existing data in the database.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if the new book's ISBN is invalid.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_tree_data(
    db: &mut PgConnection,
    tree_id: i32,
    new_tree_data: TreeData,
) -> Result<(), ApiError> {
    add_book(&mut *db, &new_tree_data).await?;

    query!(
        "UPDATE trees SET
        (last_modified, title, description, language_code, subject_id, book_isbn, book_chapter, book_exercise, assignment_text, published)
        = (current_timestamp, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        WHERE id = $1",
        tree_id,
        new_tree_data.title,
        new_tree_data.description,
        new_tree_data.language_code,
        new_tree_data.subject_id,
        new_tree_data.book_isbn.as_ref().map(|isbn| parse_isbn(isbn)).transpose()?,
        new_tree_data.book_chapter,
        new_tree_data.book_exercise,
        new_tree_data.assignment_text,
        new_tree_data.published,
    )
    .execute(&mut *db)
    .await?;

    // Delete the old tags and add the new ones.
    query!("DELETE FROM tree_tags WHERE tree_id = $1", tree_id)
        .execute(&mut *db)
        .await?;
    add_tags(db, tree_id, &new_tree_data.tags).await?;

    Ok(())
}

/// Request all data of a specific tree from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree which is queried.
///
/// # Returns
/// All data of the tree as a [`Some`] if the tree exists, or a [`None`] if it doesn't.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn db_get_tree_all_data(
    db: &mut PgConnection,
    tree_id: i32,
) -> Result<Option<(TreeMetadata, TreeStructure)>, ApiError> {
    let field = query!(
        r#"SELECT
            trees.*,
            users.name AS owner_name,
            subjects.name AS "subject_name?",
            languages.name AS language_name,
            COALESCE(books.title, '') AS book_name,
            COALESCE(books.author, '') AS book_author,
            COALESCE(books.edition, '') AS book_edition,
            ARRAY(SELECT name FROM tags JOIN tree_tags ON tag_id = tags.id WHERE tree_id = trees.id) AS tags,
            ARRAY(SELECT
                (SELECT name FROM users as co_users WHERE co_users.id = co_authors.user_id)
                FROM co_authors WHERE tree_id = trees.id
            ) AS co_authors        FROM trees
        LEFT JOIN books ON trees.book_isbn = books.isbn
        LEFT JOIN users ON users.id = trees.owner_id
        LEFT JOIN subjects ON subjects.id = trees.subject_id
        LEFT JOIN languages ON trees.language_code = languages.code
        WHERE trees.id = $1"#,
        tree_id,
    )
    .fetch_optional(&mut *db)
    .await?;

    let Some(field) = field else {
        return Ok(None);
    };

    let tree_structure = serde_json::from_value(field.structure)?;

    let tree_metadata = TreeMetadata {
        data: TreeData {
            title: field.title,
            description: field.description,
            tags: field.tags.unwrap_or(Vec::new()),
            language_code: field.language_code,
            subject_id: field.subject_id,
            book_name: field.book_name.unwrap(),
            book_edition: field.book_edition.unwrap(),
            book_author: field.book_author.unwrap(),
            book_isbn: field.book_isbn.map(|n| n.to_string()),
            book_chapter: field.book_chapter,
            book_exercise: field.book_exercise,
            assignment_text: field.assignment_text,
            published: field.published,
        },
        gen_data: TreeGeneratedData {
            id: field.id,
            owner_id: field.owner_id,
            created_at: field.created_at,
            last_modified: field.last_modified,
            times_visited: field.times_visited,
            last_visited: field.last_visited,
        },
        linked_data: TreeLinkedData {
            owner_name: field.owner_name,
            subject_name: field.subject_name,
            language_name: field.language_name,
            co_authors: field.co_authors.unwrap_or(Vec::new()),
        },
    };

    Ok(Some((tree_metadata, tree_structure)))
}

/// Get the id of the owner of the tree.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree of which the owner is queried.
///
/// # Returns
/// The id of the owner of the tree as a [`Some`] if the tree exists, or a [`None`] if it doesn't.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_tree_owner(
    db: impl PgExecutor<'_>,
    tree_id: i32,
) -> Result<Option<i32>, ApiError> {
    let owner = query!("SELECT owner_id FROM trees WHERE id = $1", tree_id,)
        .fetch_optional(db)
        .await?
        .map(|r| r.owner_id);

    Ok(owner)
}

/// Check whether a user is a co-author of a tree.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree of which the owner is queried.
///
/// `co_author_id` - The id of the given co-author.
///
/// # Returns
/// A boolean, indicating whether the given user is indeed a co-author of the given tree.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn is_co_author_of_tree(
    db: impl PgExecutor<'_>,
    tree_id: &i32,
    co_author_id: &i32,
) -> Result<bool, ApiError> {
    query!(
        "SELECT EXISTS (SELECT 1 FROM co_authors WHERE tree_id = $1 AND user_id = $2)",
        tree_id,
        co_author_id
    )
    .fetch_one(db)
    .await?
    .exists
    .ok_or(ApiError::DatabaseException)
}

/// Add a co-author to a tree.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree to which a co-author should be added.
///
/// `co_author_id` - The id of the user that should be added as co-author.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_co_author(
    db: impl PgExecutor<'_>,
    tree_id: i32,
    co_author_id: i32,
) -> Result<(), ApiError> {
    query!(
        "INSERT INTO co_authors (tree_id, user_id)
        VALUES ($1, $2)",
        tree_id,
        co_author_id
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Removes a co-author from a tree.
/// If the given user id was not actually a co-author, this function just sends a query that does nothing.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree from which a co-author should be removed.
///
/// `co_author_id` - The id of the user that should be removed as co-author.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn remove_co_author(
    db: impl PgExecutor<'_>,
    tree_id: i32,
    co_author_id: i32,
) -> Result<(), ApiError> {
    query!(
        "DELETE FROM co_authors
        WHERE tree_id=$1 AND user_id=$2",
        tree_id,
        co_author_id
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Checks whether the cards in a tree structure all exist in the `Cards` table, and link to the tree
/// and whether any duplicate cards exist in the tree structure.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
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
/// # Errors
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
    .await?
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
/// # Example
/// ```rs
/// let mut query = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM trees");
/// apply_tree_filters(query, &filters)?;
/// let trees_count: i64 = query
///         .build_query_scalar()
///         .fetch_one(db)
///         .await?;
/// ```
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if the filters contain an invalid ISBN.
fn apply_tree_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    filters: &'a TreeFilters,
    claims: Option<&Claims>,
) -> Result<(), ApiError> {
    let TreeFilters {
        owner_name,
        book,
        tag,
        language_code,
        subject,
        created_after,
        created_before,
        show_own_private,
        show_private,
        editable,
    } = filters;

    let is_admin = claims.is_some_and(|c| c.account_type == AccountType::Admin);
    let option_user_id = claims.map(|c| c.sub);

    // Determine if there will be any filters to apply, if not, we should not continue with the
    // function as it would create an empty WHERE clause.
    let has_conditions = !(*show_private && is_admin)
        || owner_name.is_some()
        || (*editable && option_user_id.is_some())
        || !book.is_empty()
        || !tag.is_empty()
        || language_code.is_some()
        || subject.is_some()
        || created_after.is_some()
        || created_before.is_some();

    if !has_conditions {
        return Ok(());
    }

    builder.push(" WHERE ");
    let mut separated = builder.separated(" AND ");

    // Check if we should NOT show all trees.
    if !(*show_private && is_admin) {
        separated.push("(published = true");

        if *show_own_private && let Some(user_id) = option_user_id {
            separated.push_unseparated(" OR (owner_id = ");
            separated.push_bind_unseparated(user_id);

            separated.push_unseparated(
                " OR EXISTS(
                SELECT 1
                FROM co_authors
                WHERE co_authors.user_id = ",
            );
            separated.push_bind_unseparated(user_id);
            separated.push_unseparated(" AND co_authors.tree_id = trees.id))");
        }

        separated.push_unseparated(")");
    }
    apply_tree_filters_owner_name(&mut separated, owner_name.as_ref());
    apply_tree_filters_editable(&mut separated, *editable, option_user_id);
    apply_tree_filters_books(&mut separated, book)?;
    apply_tree_filters_tags(&mut separated, tag);
    apply_tree_filters_language_code(&mut separated, language_code.as_ref());
    apply_tree_filters_subject(&mut separated, subject.as_ref());
    apply_tree_filters_created_time(
        &mut separated,
        created_after.as_ref(),
        created_before.as_ref(),
    );

    Ok(())
}

/// Adds the filters related to the owner of a tree to a Postgres [`QueryBuilder`].
///
/// # Arguments
/// `separated` - The query to be mutated in-place, as a [`Separated`] created from a [`QueryBuilder`].
///
/// `owner_name` - A filter specifying who should own the trees shown.
///
/// # Returns
/// Nothing.
fn apply_tree_filters_owner_name<'args>(
    separated: &mut Separated<'_, 'args, Postgres, &str>,
    owner_name: Option<&'args String>,
) {
    if let Some(owner_name) = owner_name {
        separated
            .push("(users.name = ")
            .push_bind_unseparated(owner_name)
            .push_unseparated(")");
    }
}

/// Adds the filters related to the editability of a tree to a Postgres [`QueryBuilder`].
///
/// # Arguments
/// `separated` - The query to be mutated in-place, as a [`Separated`] created from a [`QueryBuilder`].
///
/// `editable` - A filter specifying whether the user wants to see only editable trees.
///
/// `option_user_id` - The logged-in user performing the filter, optional.
///
/// # Returns
/// Nothing.
fn apply_tree_filters_editable(
    separated: &mut Separated<'_, '_, Postgres, &str>,
    editable: bool,
    option_user_id: Option<i32>,
) {
    if editable && let Some(user_id) = option_user_id {
        separated.push("(owner_id = ");

        separated.push_bind_unseparated(user_id);

        separated.push_unseparated(
            " OR EXISTS(
                SELECT 1
                FROM co_authors
                WHERE co_authors.user_id = ",
        );
        separated.push_bind_unseparated(user_id);
        separated.push_unseparated(" AND co_authors.tree_id = trees.id))");
    }
}

/// Adds the filters related to the books of a tree to a Postgres [`QueryBuilder`].
///
/// # Arguments
/// `separated` - The query to be mutated in-place, as a [`Separated`] created from a [`QueryBuilder`].
///
/// `books` - A filter specifying what books should have trees shown.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`InvalidIsbn`](ApiError::InvalidIsbn) if the filters contain an invalid ISBN.
fn apply_tree_filters_books<'args>(
    separated: &mut Separated<'_, 'args, Postgres, &str>,
    books: &'args [String],
) -> Result<(), ApiError> {
    if !books.is_empty() {
        let parsed_isbns = books
            .iter()
            .map(|isbn| parse_isbn(isbn))
            .collect::<Result<Vec<_>, _>>()?;
        separated
            .push("(book_isbn = ANY(")
            .push_bind_unseparated(parsed_isbns)
            .push_unseparated("))");
    }

    Ok(())
}

/// Adds the filters related to the tags of a tree to a Postgres [`QueryBuilder`].
///
/// # Arguments
/// `separated` - The query to be mutated in-place, as a [`Separated`] created from a [`QueryBuilder`].
///
/// `tags` - A filter specifying the tags a tree should have.
///
/// # Returns
/// Nothing.
fn apply_tree_filters_tags<'args>(
    separated: &mut Separated<'_, 'args, Postgres, &str>,
    tags: &'args [String],
) {
    // requested_tags is defined in tree_list_query_tag_cte() for improved efficiency
    if !tags.is_empty() {
        separated.push(
            r"trees.id IN (
                SELECT tree_id
                FROM tree_tags
                WHERE tag_id IN (SELECT id FROM requested_tags)
                GROUP BY tree_id
                HAVING COUNT(DISTINCT tag_id) =
                       (SELECT COUNT(*) FROM requested_tags)
            )",
        );
    }
}

/// Adds the filters related to the language of a tree to a Postgres [`QueryBuilder`].
///
/// # Arguments
/// `separated` - The query to be mutated in-place, as a [`Separated`] created from a [`QueryBuilder`].
///
/// `language_code` - A filter specifying the language code representing the tree's language.
///
/// # Returns
/// Nothing.
fn apply_tree_filters_language_code<'args>(
    separated: &mut Separated<'_, 'args, Postgres, &str>,
    language_code: Option<&'args String>,
) {
    if let Some(language_code) = language_code {
        separated
            .push("(trees.language_code = ")
            .push_bind_unseparated(language_code)
            .push_unseparated(")");
    }
}

/// Adds the filters related to the subject of a tree to a Postgres [`QueryBuilder`].
///
/// # Arguments
/// `separated` - The query to be mutated in-place, as a [`Separated`] created from a [`QueryBuilder`].
///
/// `subject` - A filter specifying the subject a tree should be about
///
/// # Returns
/// Nothing.
fn apply_tree_filters_subject<'args>(
    separated: &mut Separated<'_, 'args, Postgres, &str>,
    subject: Option<&'args String>,
) {
    if let Some(subject) = subject {
        separated
            .push("(subjects.name = ")
            .push_bind_unseparated(subject)
            .push_unseparated(")");
    }
}

/// Adds the filters related to the creation time of a tree to a Postgres [`QueryBuilder`].
///
/// # Arguments
/// `separated` - The query to be mutated in-place, as a [`Separated`] created from a [`QueryBuilder`].
///
/// `created_after` - A filter specifying the time a tree should have been created after.
///
/// `created_before` - A filter specifying the time a tree should have been created before.
///
/// # Returns
/// Nothing.
fn apply_tree_filters_created_time<'args>(
    separated: &mut Separated<'_, 'args, Postgres, &str>,
    created_after: Option<&'args DateTime<Utc>>,
    created_before: Option<&'args DateTime<Utc>>,
) {
    if let Some(created_after) = created_after {
        separated
            .push("(trees.created_at >= ")
            .push_bind_unseparated(created_after)
            .push_unseparated(")");
    }
    if let Some(created_before) = created_before {
        separated
            .push("(trees.created_at <= ")
            .push_bind_unseparated(created_before)
            .push_unseparated(")");
    }
}

const TREE_LIST_QUERY_JOINS: &str = r"
    FROM trees
    LEFT JOIN books ON books.isbn = trees.book_isbn
    LEFT JOIN users ON users.id = trees.owner_id
    LEFT JOIN subjects ON subjects.id = trees.subject_id
    LEFT JOIN languages ON languages.code = trees.language_code
";

fn tree_list_query_tag_cte<'a>(query: &mut QueryBuilder<'a, Postgres>, tags: &'a Vec<String>) {
    query
        .push(
            r"
            WITH requested_tags AS (
                SELECT id
                FROM tags
                WHERE name = ANY(",
        )
        .push_bind(tags)
        .push("))");
}

/// Get the amount of trees satisfying some filters.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `filters` - The filters to apply to the trees.
///
/// `user_id` - The logged in user performing the filter, optional.
///
/// # Returns
/// The amount of trees satisfying the filters at the requested page.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_count(
    db: impl PgExecutor<'_>,
    filters: &TreeFilters,
    claims: Option<&Claims>,
) -> Result<i64, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new("");
    if !filters.tag.is_empty() {
        tree_list_query_tag_cte(&mut query, &filters.tag);
    }
    query.push("SELECT COUNT(*)");
    query.push(TREE_LIST_QUERY_JOINS);

    apply_tree_filters(&mut query, filters, claims)?;

    let trees_count: i64 = query.build_query_scalar().fetch_one(db).await?;

    Ok(trees_count)
}

/// Get the list of all trees satisfying some filters at a given page.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
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
/// # Errors
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_list(
    db: impl PgExecutor<'_>,
    page: &PaginationOptions,
    filters: &TreeFilters,
    claims: Option<&Claims>,
) -> Result<Vec<TreeMetadata>, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new("");
    if !filters.tag.is_empty() {
        tree_list_query_tag_cte(&mut query, &filters.tag);
    }
    query.push(
        r#"
    SELECT trees.*,
        users.name                                                     AS owner_name,
        subjects.name                                                  AS "subject_name?",
        languages.name                                                 AS language_name,
        COALESCE(books.title, '')                                      AS book_name,
        COALESCE(books.author, '')                                     AS book_author,
        COALESCE(books.edition, '')                                    AS book_edition,
        ARRAY(SELECT name
            FROM tags
                JOIN tree_tags ON tag_id = id
            WHERE tree_id = trees.id)                                  AS tags,
        ARRAY(SELECT
            (SELECT name
                FROM users as co_users
                WHERE co_users.id = co_authors.user_id)
            FROM co_authors WHERE tree_id = trees.id)                  AS co_authors"#,
    );
    query.push(TREE_LIST_QUERY_JOINS);

    apply_tree_filters(&mut query, filters, claims)?;
    query
        .push(
            // newest first
            " ORDER BY trees.created_at DESC \
              LIMIT ",
        )
        .push_bind(i64::from(page.get_size()))
        .push(" OFFSET ")
        // Page is 1-based
        .push_bind(i64::from((page.get_page() - 1) * page.get_size()));

    let rows = query.build().fetch_all(db).await?;

    let trees = rows
        .iter()
        .map(TreeMetadata::from_pg_row)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // User requested a page that yields no results. This should only occur if the user requests a
    // page that is too high, as get_queried_count should be used to confirm results exist.
    if trees.is_empty() {
        return Err(ApiError::InvalidPage);
    }

    Ok(trees)
}

#[cfg(test)]
#[allow(clippy::inconsistent_digit_grouping)]
mod tests {
    use super::*;
    use crate::db::test::{
        initialize_basic_user, initialize_card, initialize_db, initialize_empty_tree,
        initialize_tree_with_data,
    };
    use crate::db_structs::trees::{Node, RootNode};
    use crate::get_connection_unwrap;
    use chrono::{TimeDelta, Utc};
    use sqlx::PgPool;

    #[test]
    fn test_parse_isbn() {
        // Valid ISBN 10
        let result = parse_isbn("0-451-52493-4");
        assert_eq!(result, Ok(978_0_451_52493_5));

        // Invalid ISBN 10
        let result = parse_isbn("0-451-52493-5");
        assert_eq!(result, Err(ApiError::InvalidIsbn));

        // Valid ISBN 10 with X check digit
        let result = parse_isbn("0-074-62542-X");
        assert_eq!(result, Ok(978_0_074_62542_2));

        // Valid ISBN 13
        let result = parse_isbn("979-1-034-90821-9");
        assert_eq!(result, Ok(979_1_034_90821_9));

        // Invalid ISBN 13
        let result = parse_isbn("979-1-034-90821-1");
        assert_eq!(result, Err(ApiError::InvalidIsbn));
    }

    #[sqlx::test]
    async fn test_add_book(db: PgPool) {
        let tree_data = TreeData {
            book_isbn: Some("978-0-061-12008-4".to_string()),
            ..Default::default()
        };

        // Insert when ISBN does not exist
        let existed = add_book(get_connection_unwrap!(db), &tree_data)
            .await
            .unwrap();
        assert!(!existed);

        // Insert when ISBN exists
        let existed = add_book(get_connection_unwrap!(db), &tree_data)
            .await
            .unwrap();
        assert!(existed);
    }

    #[sqlx::test]
    async fn test_add_tree(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

        let tree_data = TreeData {
            tags: vec!["tag1".to_string()],
            ..Default::default()
        };

        let tree_add_data = TreeAddData {
            tree_data: tree_data.clone(),
            tree_structure: TreeStructure { root_nodes: vec![] },
        };

        let tree_id = add_tree(get_connection_unwrap!(db), &tree_add_data, user_id)
            .await
            .unwrap();

        let found_owner_id = query!("SELECT owner_id FROM trees WHERE id = $1", tree_id)
            .fetch_one(&db)
            .await
            .unwrap()
            .owner_id;

        assert_eq!(found_owner_id, user_id);

        let record = query!(
            "SELECT trees.title, trees.description, trees.language_code, trees.subject_id, COALESCE(books.title, '') AS book_name, COALESCE(books.author, '') AS book_author, COALESCE(books.edition, '') AS book_edition, trees.book_isbn, trees.book_chapter, trees.book_exercise, trees.assignment_text, trees.published, ARRAY(SELECT name FROM tags JOIN tree_tags ON tag_id = id WHERE tree_id = trees.id) AS tags
            FROM trees
            LEFT JOIN books ON trees.book_isbn = books.isbn
            WHERE id = $1", tree_id)
            .fetch_one(&db)
            .await
            .unwrap();
        let found_tree_data = TreeData {
            title: record.title,
            description: record.description,
            tags: record.tags.unwrap_or(Vec::new()),
            language_code: record.language_code,
            subject_id: record.subject_id,
            book_name: record.book_name.unwrap(),
            book_edition: record.book_edition.unwrap(),
            book_author: record.book_author.unwrap(),
            book_isbn: record.book_isbn.map(|n| n.to_string()),
            book_chapter: record.book_chapter,
            book_exercise: record.book_exercise,
            assignment_text: record.assignment_text,
            published: record.published,
        };

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
    async fn test_update_tree_data(db: PgPool) {
        let (user_id, _) = initialize_db(get_connection_unwrap!(db)).await;

        // Add a tree to the database
        let tree_data = TreeData {
            tags: vec!["tag1".to_string()],
            ..Default::default()
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
        let tree_id = add_tree(get_connection_unwrap!(db), &tree_add_data, user_id)
            .await
            .unwrap();

        // Update the tree data
        let new_tree_data = TreeData {
            title: "Updated tree".to_string(),
            description: "Updated description".to_string(),
            language_code: "nl".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            subject_id: Some(3),
            book_name: "Updated book".to_string(),
            book_author: "John Writer".to_string(),
            book_edition: "2nd".to_string(),
            book_isbn: Some("9780136019701".to_string()),
            book_chapter: "3".to_string(),
            book_exercise: "5b".to_string(),
            assignment_text: "Updated assignment".to_string(),
            published: true,
        };
        update_tree_data(get_connection_unwrap!(db), tree_id, new_tree_data.clone())
            .await
            .unwrap();

        // Verify the data was updated
        let (found_tree_metadata, _) = db_get_tree_all_data(get_connection_unwrap!(db), tree_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found_tree_metadata.data, new_tree_data);
    }

    #[sqlx::test]
    async fn test_db_get_tree_all_data(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

        let tree_data = TreeData {
            published: true,
            ..Default::default()
        };

        let structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![],
            }],
        };

        let tree_id =
            initialize_tree_with_data(get_connection_unwrap!(db), user_id, &tree_data, &structure)
                .await;

        let (found_tree_metadata, found_tree_structure) =
            db_get_tree_all_data(get_connection_unwrap!(db), tree_id)
                .await
                .unwrap()
                .unwrap();

        assert_eq!(tree_data, found_tree_metadata.data);
        assert_eq!(tree_id, found_tree_metadata.gen_data.id);
        assert_eq!(user_id, found_tree_metadata.gen_data.owner_id);
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
    fn test_apply_tree_filters() {
        use chrono::{DateTime, Utc};

        let mut no_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let empty_filters = TreeFilters {
            ..Default::default()
        };
        apply_tree_filters(&mut no_filters_query, &empty_filters, None).unwrap();
        assert_eq!(
            no_filters_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );
        let admin_claims = Claims {
            sub: 123,
            exp: 0,
            account_type: AccountType::Admin,
        };

        let mut all_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let all_filters = TreeFilters {
            owner_name: Some("67".to_owned()),
            book: vec!["0-451-52493-4".to_owned()],
            tag: vec!["funky".to_owned()],
            language_code: Some("nl".to_owned()),
            subject: Some("Wiskunde A".to_owned()),
            created_after: Some(DateTime::<Utc>::MIN_UTC),
            created_before: Some(DateTime::<Utc>::MAX_UTC),
            show_own_private: true,
            show_private: false,
            editable: false,
        };
        apply_tree_filters(&mut all_filters_query, &all_filters, Some(&admin_claims)).unwrap();
        assert_eq!(
            all_filters_query.sql(),
            "SELECT * FROM trees WHERE (published = true OR (owner_id = $1 OR EXISTS(\n                SELECT 1\n                FROM co_authors\n                WHERE co_authors.user_id = $2 AND co_authors.tree_id = trees.id))) AND (users.name = $3) AND (book_isbn = ANY($4)) AND trees.id IN (\n                SELECT tree_id\n                FROM tree_tags\n                WHERE tag_id IN (SELECT id FROM requested_tags)\n                GROUP BY tree_id\n                HAVING COUNT(DISTINCT tag_id) =\n                       (SELECT COUNT(*) FROM requested_tags)\n            ) AND (trees.language_code = $5) AND (subjects.name = $6) AND (trees.created_at >= $7) AND (trees.created_at <= $8)"
        );

        let mut partial_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let partial_filters = TreeFilters {
            owner_name: Some("67".to_owned()),
            created_after: Some(DateTime::<Utc>::MIN_UTC),
            ..Default::default()
        };
        apply_tree_filters(&mut partial_filters_query, &partial_filters, None).unwrap();
        assert_eq!(
            partial_filters_query.sql(),
            "SELECT * FROM trees WHERE (published = true) AND (users.name = $1) AND (trees.created_at >= $2)"
        );
    }

    #[test]
    fn test_apply_tree_filters_own_unpublished_trees() {
        let user_claims = Claims {
            sub: 123,
            exp: 0,
            account_type: AccountType::Author,
        };

        // check if show_own_private gets made properly
        let mut show_own_private_no_id_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let show_own_private_no_id_filters = TreeFilters {
            show_own_private: true,
            ..Default::default()
        };
        apply_tree_filters(
            &mut show_own_private_no_id_query,
            &show_own_private_no_id_filters,
            None,
        )
        .unwrap();
        assert_eq!(
            show_own_private_no_id_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );

        let mut show_own_private_with_id_query =
            QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let show_own_private_with_id_filters = TreeFilters {
            show_own_private: true,
            ..Default::default()
        };
        apply_tree_filters(
            &mut show_own_private_with_id_query,
            &show_own_private_with_id_filters,
            Some(&user_claims),
        )
        .unwrap();
        assert_eq!(
            show_own_private_with_id_query.sql(),
            "SELECT * FROM trees WHERE (published = true OR (owner_id = $1 OR EXISTS(\n                SELECT 1\n                FROM co_authors\n                WHERE co_authors.user_id = $2 AND co_authors.tree_id = trees.id)))",
        );

        let mut hide_own_private_with_id_query =
            QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let hide_own_private_with_id_filters = TreeFilters {
            show_own_private: false,
            ..Default::default()
        };
        apply_tree_filters(
            &mut hide_own_private_with_id_query,
            &hide_own_private_with_id_filters,
            Some(&user_claims),
        )
        .unwrap();
        assert_eq!(
            hide_own_private_with_id_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );
    }

    #[test]
    fn test_apply_tree_filters_unpublished_trees() {
        let user_claims = Claims {
            sub: 123,
            exp: 0,
            account_type: AccountType::Author,
        };
        let admin_claims = Claims {
            sub: 123,
            exp: 0,
            account_type: AccountType::Admin,
        };

        // Check if show_private gets made properly
        let mut show_private_no_admin_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let show_private_no_admin_filters = TreeFilters {
            show_private: true,
            ..Default::default()
        };
        apply_tree_filters(
            &mut show_private_no_admin_query,
            &show_private_no_admin_filters,
            Some(&user_claims),
        )
        .unwrap();
        assert_eq!(
            show_private_no_admin_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );

        let mut show_private_no_claims_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let show_private_no_admin_filters = TreeFilters {
            show_private: true,
            ..Default::default()
        };
        apply_tree_filters(
            &mut show_private_no_claims_query,
            &show_private_no_admin_filters,
            None,
        )
        .unwrap();
        assert_eq!(
            show_private_no_claims_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );

        let mut show_private_admin_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let show_private_admin_filters = TreeFilters {
            show_private: true,
            ..Default::default()
        };
        apply_tree_filters(
            &mut show_private_admin_query,
            &show_private_admin_filters,
            Some(&admin_claims),
        )
        .unwrap();
        assert_eq!(show_private_admin_query.sql(), "SELECT * FROM trees",);

        let mut hide_private_admin_query = QueryBuilder::<Postgres>::new("SELECT * FROM trees");
        let hide_private_admin_filters = TreeFilters {
            show_private: false,
            ..Default::default()
        };
        apply_tree_filters(
            &mut hide_private_admin_query,
            &hide_private_admin_filters,
            Some(&admin_claims),
        )
        .unwrap();
        assert_eq!(
            hide_private_admin_query.sql(),
            "SELECT * FROM trees WHERE (published = true)"
        );
    }

    #[sqlx::test]
    async fn test_get_queried_count(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;
        let claims = Claims {
            sub: user_id,
            exp: 0,
            account_type: AccountType::Author,
        };
        let empty_filters = TreeFilters {
            ..Default::default()
        };
        let show_private_filters = TreeFilters {
            show_own_private: true,
            ..Default::default()
        };
        let basic_structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![],
            }],
        };

        assert_eq!(
            get_queried_count(&db, &empty_filters, None).await.unwrap(),
            0
        );
        assert_eq!(
            get_queried_count(&db, &show_private_filters, Some(&claims))
                .await
                .unwrap(),
            0
        );

        initialize_empty_tree(get_connection_unwrap!(db), user_id).await;
        assert_eq!(
            get_queried_count(&db, &empty_filters, None).await.unwrap(),
            0
        );
        assert_eq!(
            get_queried_count(&db, &show_private_filters, Some(&claims))
                .await
                .unwrap(),
            1
        );
        // Show private enabled, but not logged in
        assert_eq!(
            get_queried_count(&db, &show_private_filters, None)
                .await
                .unwrap(),
            0
        );

        for _ in 0..100 {
            initialize_empty_tree(get_connection_unwrap!(db), user_id).await;
        }
        assert_eq!(
            get_queried_count(&db, &show_private_filters, Some(&claims))
                .await
                .unwrap(),
            101
        );

        let public_tree = TreeData {
            published: true,
            ..Default::default()
        };
        initialize_tree_with_data(
            get_connection_unwrap!(db),
            user_id,
            &public_tree,
            &basic_structure,
        )
        .await;
        assert_eq!(
            get_queried_count(&db, &show_private_filters, Some(&claims))
                .await
                .unwrap(),
            102
        );
        assert_eq!(
            get_queried_count(&db, &empty_filters, None).await.unwrap(),
            1
        );

        let book_filter = TreeFilters {
            book: vec!["978-0-061-12008-4".to_owned()],
            ..Default::default()
        };
        let book_tree = TreeData {
            book_isbn: Some("978-0-061-12008-4".to_owned()),
            published: true,
            ..Default::default()
        };
        initialize_tree_with_data(
            get_connection_unwrap!(db),
            user_id,
            &book_tree,
            &basic_structure,
        )
        .await;
        assert_eq!(get_queried_count(&db, &book_filter, None).await.unwrap(), 1);
        assert_eq!(
            get_queried_count(&db, &empty_filters, None).await.unwrap(),
            2
        );
    }

    #[sqlx::test]
    async fn test_get_queried_list(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;
        let claims = Claims {
            sub: user_id,
            exp: 0,
            account_type: AccountType::Author,
        };
        let empty_filters = TreeFilters {
            ..Default::default()
        };
        let show_private_filters = TreeFilters {
            show_own_private: true,
            ..Default::default()
        };
        let basic_structure = TreeStructure {
            root_nodes: vec![RootNode {
                label: "Orientation".to_owned(),
                children: vec![],
            }],
        };
        let first_page = PaginationOptions::new(1, 50, 1000, 500).unwrap();
        let one_item_page1 = PaginationOptions::new(1, 1, 1000, 500).unwrap();
        let one_item_page2 = PaginationOptions::new(2, 1, 1000, 500).unwrap();

        let no_trees = get_queried_list(&db, &first_page, &empty_filters, None).await;
        assert_eq!(no_trees, Err(ApiError::InvalidPage));

        let private_tree = TreeData::default();
        let public_tree = TreeData {
            published: true,
            ..Default::default()
        };
        initialize_tree_with_data(
            get_connection_unwrap!(db),
            user_id,
            &private_tree,
            &basic_structure,
        )
        .await;
        initialize_tree_with_data(
            get_connection_unwrap!(db),
            user_id,
            &public_tree,
            &basic_structure,
        )
        .await;

        let private_tree_list =
            get_queried_list(&db, &first_page, &show_private_filters, Some(&claims))
                .await
                .unwrap()
                .into_iter()
                .map(|d| d.data)
                .collect::<Vec<_>>();
        assert_eq!(
            private_tree_list,
            vec![public_tree.clone(), private_tree.clone()]
        );

        let public_tree_list = get_queried_list(&db, &first_page, &empty_filters, None)
            .await
            .unwrap()
            .into_iter()
            .map(|d| d.data)
            .collect::<Vec<_>>();
        assert_eq!(public_tree_list, vec![public_tree.clone()]);

        let private_tree_list_one_item1 =
            get_queried_list(&db, &one_item_page1, &show_private_filters, Some(&claims))
                .await
                .unwrap()
                .into_iter()
                .map(|d| d.data)
                .collect::<Vec<_>>();
        let private_tree_list_one_item2 =
            get_queried_list(&db, &one_item_page2, &show_private_filters, Some(&claims))
                .await
                .unwrap()
                .into_iter()
                .map(|d| d.data)
                .collect::<Vec<_>>();
        assert_eq!(private_tree_list_one_item1, vec![public_tree.clone()]);
        assert_eq!(private_tree_list_one_item2, vec![private_tree.clone()]);

        // Test all filters from here, owner_id is tested in integration tests.
        let future_filter = TreeFilters {
            created_after: Some(Utc::now() + TimeDelta::seconds(2)),
            show_own_private: true,
            ..Default::default()
        };
        let future_tree_list =
            get_queried_list(&db, &first_page, &future_filter, Some(&claims)).await;
        assert_eq!(future_tree_list, Err(ApiError::InvalidPage));
        let recent_filter = TreeFilters {
            created_after: Some(Utc::now() - TimeDelta::seconds(2)),
            show_own_private: true,
            ..Default::default()
        };
        let recent_tree_list = get_queried_list(&db, &first_page, &recent_filter, Some(&claims))
            .await
            .unwrap()
            .into_iter()
            .map(|d| d.data)
            .collect::<Vec<_>>();
        assert_eq!(
            recent_tree_list,
            vec![public_tree.clone(), private_tree.clone()]
        );

        let ancient_filter = TreeFilters {
            created_before: Some(Utc::now() - TimeDelta::seconds(2)),
            show_own_private: true,
            ..Default::default()
        };
        let ancient_tree_list =
            get_queried_list(&db, &first_page, &ancient_filter, Some(&claims)).await;
        assert_eq!(ancient_tree_list, Err(ApiError::InvalidPage));
        let existing_filter = TreeFilters {
            created_before: Some(Utc::now() + TimeDelta::seconds(2)),
            show_own_private: true,
            ..Default::default()
        };
        let existing_tree_list =
            get_queried_list(&db, &first_page, &existing_filter, Some(&claims))
                .await
                .unwrap()
                .into_iter()
                .map(|d| d.data)
                .collect::<Vec<_>>();
        assert_eq!(
            existing_tree_list,
            vec![public_tree.clone(), private_tree.clone()]
        );

        let book_filter = TreeFilters {
            book: vec!["978-0-061-12008-4".to_owned()],
            ..Default::default()
        };
        let book_tree = TreeData {
            book_isbn: Some("9780061120084".to_owned()),
            published: true,
            ..Default::default()
        };
        let book_tree_id = initialize_tree_with_data(
            get_connection_unwrap!(db),
            user_id,
            &book_tree,
            &basic_structure,
        )
        .await;
        let book_tree_list = get_queried_list(&db, &first_page, &book_filter, Some(&claims))
            .await
            .unwrap();
        let book_tree_data_list = book_tree_list
            .clone()
            .into_iter()
            .map(|d| d.data)
            .collect::<Vec<_>>();
        assert_eq!(book_tree_data_list, vec![book_tree]);
        let book_tree_id_list = book_tree_list
            .into_iter()
            .map(|d| d.gen_data.id)
            .collect::<Vec<_>>();
        assert_eq!(book_tree_id_list, vec![book_tree_id]);
    }

    #[sqlx::test]
    async fn test_co_author_exists(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let co_author_id = query!(
            "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING id",
            "co_author@email.com",
            "password_hash",
            "co_author"
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .id;

        add_co_author(&db, tree_id, co_author_id).await.unwrap();

        let result = is_co_author_of_tree(&db, &tree_id, &co_author_id)
            .await
            .unwrap();

        assert!(result);
    }

    #[sqlx::test]
    async fn test_co_author_not_exists(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let other_user = query!(
            "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING id",
            "other_user@email.com",
            "password_hash",
            "other_user"
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .id;

        let result = is_co_author_of_tree(&db, &tree_id, &other_user)
            .await
            .unwrap();

        assert!(!result);
    }

    #[sqlx::test]
    async fn test_co_author_duplicate_insert(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let co_author_id = query!(
            "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING id",
            "co_author@email.com",
            "password_hash",
            "co_author"
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .id;

        add_co_author(&db, tree_id, co_author_id).await.unwrap();

        let result = add_co_author(&db, tree_id, co_author_id).await;

        assert!(result.is_err());
    }
}
