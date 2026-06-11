/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::cards::{CardData, CardFilters, LibraryCardGetData};
use crate::errors::ApiError;
use sqlx::{PgConnection, PgExecutor, Postgres, QueryBuilder, query};

/// Add a list of tags for a given library card in the database.
///
/// # Arguments
/// `db` - A connection to the database, as [`&mut PgConnection`](PgConnection).
///
/// `library_card_id` - The id of the library card to assign the tags to.
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
    library_card_id: i32,
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
        "INSERT INTO library_cards_tags (library_card_id, tag_id) SELECT $1 AS library_card_id, tag_id FROM UNNEST($2::INTEGER[]) AS DATA(tag_id)",
        library_card_id,
        &tag_ids.iter().map(|r| r.id).collect::<Vec<_>>()
    )
    .execute(&mut *db)
    .await?;

    Ok(())
}

/// Add a card to the card library database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `owner_id` - The id of the owner as [`i32`].
///
/// `title` - The title of the card in the library as [`String`].
///
/// `tags` - The tags to find this card while filtering the cards library as [`&[String]`].
///
/// `card_text` - The front and back text of the card as [`CardData`].
///
/// # Returns
/// The id assigned to the new card in the cards library if successful.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn cards_library_add(
    db: &mut PgConnection,
    owner_id: i32,
    title: &String,
    tags: &[String],
    card_text: &CardData,
) -> Result<i32, ApiError> {
    let library_card_id = query!(
        "INSERT INTO library_cards (owner_id, title, front, back)
        VALUES ($1, $2, $3, $4)
        RETURNING id",
        owner_id,
        title,
        card_text.front,
        card_text.back
    )
    .fetch_one(&mut *db)
    .await?
    .id;

    // Delete the old tags and add the new ones.
    query!(
        "DELETE FROM library_cards_tags WHERE library_card_id = $1",
        library_card_id
    )
    .execute(&mut *db)
    .await?;
    add_tags(db, library_card_id, tags).await?;

    Ok(library_card_id)
}

/// Get the amount of cards satisfying some filters.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `filters` - The filters to apply to the library cards.
///
/// # Returns
/// The amount of cards satisfying the filters.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_cards_count(
    db: impl PgExecutor<'_>,
    filters: &CardFilters,
) -> Result<i64, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new("");
    if !filters.tags.is_empty() {
        list_query_tag_cte(&mut query, &filters.tags);
    }

    query.push("SELECT COUNT(*)");
    query.push(CARD_LIST_QUERY_JOINS);

    apply_card_filters(&mut query, filters);

    let card_count: i64 = query.build_query_scalar().fetch_one(db).await?;

    Ok(card_count)
}

/// Get the list of all cards satisfying some filters.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `filters` - The filters to apply to the cards.
///
/// # Returns
/// A list with the cards satisfying the filters.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_cards_list(
    db: impl PgExecutor<'_>,
    filters: &CardFilters,
) -> Result<Vec<LibraryCardGetData>, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new("");
    if !filters.tags.is_empty() {
        list_query_tag_cte(&mut query, &filters.tags);
    }

    query.push(
        "
        SELECT library_cards.id AS card_id, library_cards.created_at, title, front, back, users.name AS owner_name,
          ARRAY(SELECT name
                FROM tags
                         JOIN library_cards_tags ON tag_id = id
                WHERE library_card_id = library_cards.id)       AS tags",
    );
    query.push(CARD_LIST_QUERY_JOINS);

    apply_card_filters(&mut query, filters);

    query.push(
        // newest first
        " ORDER BY created_at DESC ",
    );

    let library_cards = query
        .build_query_as::<LibraryCardGetData>()
        .fetch_all(db)
        .await?;

    Ok(library_cards)
}

const CARD_LIST_QUERY_JOINS: &str = r"
    FROM library_cards
    LEFT JOIN users ON users.id = library_cards.owner_id
";

fn list_query_tag_cte<'a>(query: &mut QueryBuilder<'a, Postgres>, tags: &'a Vec<String>) {
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

/// Adds [`CardFilters`] to a Postgres [`QueryBuilder`]
///
/// # Arguments
/// `builder` - The [`QueryBuilder`] to be mutated in-place.
///
/// `filters` - The [`CardFilters`] to apply to the query.
fn apply_card_filters<'a>(builder: &mut QueryBuilder<'a, Postgres>, filters: &'a CardFilters) {
    let CardFilters {
        owner_name,
        created_after,
        created_before,
        tags,
    } = filters;

    if owner_name.is_none()
        && created_after.is_none()
        && created_before.is_none()
        && tags.is_empty()
    {
        return;
    }

    builder.push(" WHERE ");
    let mut separated = builder.separated(" AND ");
    if let Some(owner_name) = owner_name {
        separated
            .push("(users.name = ")
            .push_bind_unseparated(owner_name)
            .push_unseparated(")");
    }
    if let Some(created_after) = created_after {
        separated
            .push("(library_cards.created_at >= ")
            .push_bind_unseparated(created_after)
            .push_unseparated(")");
    }
    if let Some(created_before) = created_before {
        separated
            .push("(library_cards.created_at <= ")
            .push_bind_unseparated(created_before)
            .push_unseparated(")");
    }
    if !tags.is_empty() {
        separated.push(
            r"library_cards.id IN (
                SELECT library_card_id
                FROM library_cards_tags
                WHERE tag_id IN (SELECT id FROM requested_tags)
                GROUP BY library_card_id
                HAVING COUNT(DISTINCT tag_id) =
                       (SELECT COUNT(*) FROM requested_tags)
            )",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::cards::get_card_text;
    use crate::db::test::{initialize_card, initialize_db};
    use crate::db::users::get_user_all_data;
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_cards_library_add(db: PgPool) {
        let (owner_id, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let title = "title".to_owned();
        let tags = vec!["tag1".to_owned(), "tag2".to_owned()];
        let card_text = get_card_text(&db, card_id).await.unwrap();

        let card_lib_id = cards_library_add(
            get_connection_unwrap!(db),
            owner_id,
            &title,
            &tags,
            &card_text,
        )
        .await
        .unwrap();

        let found_card_data = query!(
            "SELECT owner_id, title, front, back FROM library_cards WHERE id=$1",
            card_lib_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(found_card_data.owner_id, owner_id);
        assert_eq!(found_card_data.title, title);
        assert_eq!(found_card_data.front, card_text.front);
        assert_eq!(found_card_data.back, card_text.back);

        let found_tags = query!("SELECT name FROM tags")
            .fetch_all(&db)
            .await
            .unwrap();

        assert_eq!(
            found_tags
                .iter()
                .map(|record| record.name.clone())
                .collect::<Vec<_>>(),
            tags
        );
    }

    #[test]
    fn test_apply_card_filters() {
        use chrono::{DateTime, Utc};

        let mut no_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM library_cards");
        let empty_filters = CardFilters {
            ..Default::default()
        };
        apply_card_filters(&mut no_filters_query, &empty_filters);
        assert_eq!(no_filters_query.sql(), "SELECT * FROM library_cards");

        let mut all_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM library_cards");
        let all_filters = CardFilters {
            owner_name: Some("zeszeven".to_string()),
            created_after: Some(DateTime::<Utc>::MIN_UTC),
            created_before: Some(DateTime::<Utc>::MAX_UTC),
            tags: vec!["tags".to_owned()],
        };
        apply_card_filters(&mut all_filters_query, &all_filters);
        assert_eq!(
            all_filters_query.sql(),
            "SELECT * FROM library_cards WHERE (users.name = $1) AND (library_cards.created_at >= $2) AND (library_cards.created_at <= $3) AND library_cards.id IN (\n                SELECT library_card_id\n                FROM library_cards_tags\n                WHERE tag_id IN (SELECT id FROM requested_tags)\n                GROUP BY library_card_id\n                HAVING COUNT(DISTINCT tag_id) =\n                       (SELECT COUNT(*) FROM requested_tags)\n            )"
        );

        let mut partial_filters_query =
            QueryBuilder::<Postgres>::new("SELECT * FROM library_cards");
        let partial_filters = CardFilters {
            owner_name: Some("zeszeven".to_string()),
            created_after: Some(DateTime::<Utc>::MIN_UTC),
            created_before: None,
            tags: vec![],
        };
        apply_card_filters(&mut partial_filters_query, &partial_filters);
        assert_eq!(
            partial_filters_query.sql(),
            "SELECT * FROM library_cards WHERE (users.name = $1) AND (library_cards.created_at >= $2)"
        );
    }

    #[sqlx::test]
    async fn test_get_queried_cards_count(db: PgPool) {
        let empty_filters = CardFilters {
            ..Default::default()
        };

        assert_eq!(
            get_queried_cards_count(&db, &empty_filters).await.unwrap(),
            0
        );

        // Add card to library
        let (owner_id, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let card_text = get_card_text(&db, card_id).await.unwrap();

        let _ = cards_library_add(
            get_connection_unwrap!(db),
            owner_id,
            &String::new(),
            &[],
            &card_text,
        )
        .await
        .unwrap();

        assert_eq!(
            get_queried_cards_count(&db, &empty_filters).await.unwrap(),
            1
        );
    }

    #[sqlx::test]
    async fn test_get_queried_cards_list(db: PgPool) {
        use chrono::{TimeDelta, Utc};

        let empty_filters = CardFilters {
            ..Default::default()
        };

        let empty_list = get_queried_cards_list(&db, &empty_filters).await.unwrap();
        assert_eq!(empty_list, vec![]);

        // Add card to library
        let (owner_id, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let owner_name = get_user_all_data(&db, owner_id)
            .await
            .unwrap()
            .updateable_data
            .data
            .name;
        let title = "title".to_owned();
        let tags = vec!["tag1".to_owned(), "tag2".to_owned()];
        let card_text = get_card_text(&db, card_id).await.unwrap();

        let card_lib_id = cards_library_add(
            get_connection_unwrap!(db),
            owner_id,
            &title,
            &tags,
            &card_text,
        )
        .await
        .unwrap();

        let no_filter_list = get_queried_cards_list(&db, &empty_filters).await.unwrap();
        let created_at = query!(
            "SELECT created_at FROM library_cards WHERE id = $1",
            card_lib_id
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .created_at;

        let success_outcome = vec![LibraryCardGetData {
            card_id,
            title,
            owner_name,
            created_at,
            tags,
            front: card_text.front,
            back: card_text.back,
        }];

        assert_eq!(no_filter_list, success_outcome);

        let owner_name_filter = CardFilters {
            owner_name: Some("ThisOwnerNameIsntUsed".to_string()),
            ..Default::default()
        };
        let owner_name_list = get_queried_cards_list(&db, &owner_name_filter)
            .await
            .unwrap();

        assert_eq!(owner_name_list, vec![]);

        let future_filter = CardFilters {
            created_after: Some(Utc::now() + TimeDelta::seconds(2)),
            ..Default::default()
        };
        let future_tree_list = get_queried_cards_list(&db, &future_filter).await.unwrap();
        assert_eq!(future_tree_list, vec![]);
        let recent_filter = CardFilters {
            created_after: Some(Utc::now() - TimeDelta::seconds(2)),
            ..Default::default()
        };
        let recent_tree_list = get_queried_cards_list(&db, &recent_filter).await.unwrap();
        assert_eq!(recent_tree_list, success_outcome);

        let ancient_filter = CardFilters {
            created_before: Some(Utc::now() - TimeDelta::seconds(2)),
            ..Default::default()
        };
        let ancient_tree_list = get_queried_cards_list(&db, &ancient_filter).await.unwrap();
        assert_eq!(ancient_tree_list, vec![]);
        let existing_filter = CardFilters {
            created_before: Some(Utc::now() + TimeDelta::seconds(2)),
            ..Default::default()
        };
        let existing_tree_list = get_queried_cards_list(&db, &existing_filter).await.unwrap();
        assert_eq!(existing_tree_list, success_outcome);

        let success_tags_filter = CardFilters {
            owner_name: None,
            created_after: None,
            created_before: None,
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
        };
        let success_tags_list = get_queried_cards_list(&db, &success_tags_filter)
            .await
            .unwrap();

        assert_eq!(success_tags_list, success_outcome);

        let failure_tags_filter = CardFilters {
            owner_name: None,
            created_after: None,
            created_before: None,
            tags: vec!["tag3".to_owned()],
        };
        let failure_tags_list = get_queried_cards_list(&db, &failure_tags_filter)
            .await
            .unwrap();

        assert_eq!(failure_tags_list, vec![]);
    }
}
