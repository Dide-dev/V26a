/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::cards::{CardData, CardFilters, LibraryCardGetData};
use crate::errors::ApiError;
use sqlx::{PgExecutor, Postgres, QueryBuilder, query};

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
    db: impl PgExecutor<'_>,
    owner_id: &i32,
    title: &String,
    tags: &[String],
    card_text: &CardData,
) -> Result<i32, ApiError> {
    let library_card_id = query!(
        "INSERT INTO library_cards (owner_id, title, tags, front, back)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id",
        owner_id,
        title,
        tags,
        card_text.front,
        card_text.back
    )
    .fetch_one(db)
    .await?
    .id;

    Ok(library_card_id)
}

/// Get the amount of cards satisfying some filters.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `filters` - The filters to apply to the trees.
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
    let mut query = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM library_cards");

    apply_card_filters(&mut query, filters);

    let card_count: i64 = query.build_query_scalar().fetch_one(db).await?;

    Ok(card_count)
}

/// Get the list of all cards satisfying some filters.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `filters` - The filters to apply to the trees.
///
/// # Returns
/// A list with the trees satisfying the filters.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_cards_list(
    db: impl PgExecutor<'_>,
    filters: &CardFilters,
) -> Result<Vec<LibraryCardGetData>, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new(
        "
        SELECT id AS card_id, owner_id, created_at, title, tags, front, back
        FROM library_cards",
    );

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

/// Adds [`CardFilters`] to a Postgres [`QueryBuilder`]
///
/// # Arguments
/// `builder` - The [`QueryBuilder`] to be mutated in-place.
///
/// `filters` - The [`CardFilters`] to apply to the query.
fn apply_card_filters<'a>(builder: &mut QueryBuilder<'a, Postgres>, filters: &'a CardFilters) {
    let CardFilters {
        owner_id,
        created_after,
        created_before,
        tags,
    } = filters;

    if owner_id.is_none() && created_after.is_none() && created_before.is_none() && tags.is_empty()
    {
        return;
    }

    builder.push(" WHERE ");
    let mut separated = builder.separated(" AND ");
    if let Some(owner_id) = owner_id {
        separated
            .push("(owner_id = ")
            .push_bind_unseparated(owner_id)
            .push_unseparated(")");
    }
    if let Some(created_after) = created_after {
        separated
            .push("(created_at >= ")
            .push_bind_unseparated(created_after)
            .push_unseparated(")");
    }
    if let Some(created_before) = created_before {
        separated
            .push("(created_at <= ")
            .push_bind_unseparated(created_before)
            .push_unseparated(")");
    }
    if !tags.is_empty() {
        separated
            .push("(tags @>")
            .push_bind_unseparated(tags)
            .push_unseparated("::varchar[])");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::cards::get_card_text;
    use crate::db::test::{initialize_card, initialize_db};
    use crate::db::trees::get_tree_owner;
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_cards_library_add(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let owner_id = get_tree_owner(&db, tree_id).await.unwrap().unwrap();
        let title = "title".to_owned();
        let tags = vec!["tag1".to_owned(), "tag2".to_owned()];
        let card_text = get_card_text(&db, card_id).await.unwrap();

        let card_lib_id = cards_library_add(&db, &owner_id, &title, &tags, &card_text)
            .await
            .unwrap();

        let found_card_data = query!(
            "SELECT owner_id, title, tags, front, back FROM library_cards WHERE id=$1",
            card_lib_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(found_card_data.owner_id, owner_id);
        assert_eq!(found_card_data.title, title);
        assert_eq!(found_card_data.tags, Some(tags));
        assert_eq!(found_card_data.front, card_text.front);
        assert_eq!(found_card_data.back, card_text.back);
    }

    #[test]
    fn test_apply_card_filters() {
        use chrono::{DateTime, Utc};

        let mut no_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM library_cards");
        let empty_filters = CardFilters {
            owner_id: None,
            created_after: None,
            created_before: None,
            tags: vec![],
        };
        apply_card_filters(&mut no_filters_query, &empty_filters);
        assert_eq!(no_filters_query.sql(), "SELECT * FROM library_cards");

        let mut all_filters_query = QueryBuilder::<Postgres>::new("SELECT * FROM library_cards");
        let all_filters = CardFilters {
            owner_id: Some(67),
            created_after: Some(DateTime::<Utc>::MIN_UTC),
            created_before: Some(DateTime::<Utc>::MAX_UTC),
            tags: vec!["tags".to_owned()],
        };
        apply_card_filters(&mut all_filters_query, &all_filters);
        assert_eq!(
            all_filters_query.sql(),
            "SELECT * FROM library_cards WHERE (owner_id = $1) AND (created_at >= $2) AND (created_at <= $3) AND (tags @>$4::varchar[])"
        );

        let mut partial_filters_query =
            QueryBuilder::<Postgres>::new("SELECT * FROM cards_library");
        let partial_filters = CardFilters {
            owner_id: Some(67),
            created_after: Some(DateTime::<Utc>::MIN_UTC),
            created_before: None,
            tags: vec![],
        };
        apply_card_filters(&mut partial_filters_query, &partial_filters);
        assert_eq!(
            partial_filters_query.sql(),
            "SELECT * FROM cards_library WHERE (owner_id = $1) AND (created_at >= $2)"
        );
    }
}
