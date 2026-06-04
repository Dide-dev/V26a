/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::cards::CardData;
use crate::errors::ApiError;
use sqlx::{PgConnection, PgExecutor, query, query_as};
use std::collections::BTreeMap;

/// Add a new card to the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - Tree to add the card to.
///
/// `card_data` - The data assigned to the new card as [`CardData`].
///
/// # Returns
/// The id assigned to the new card if successful.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_card(
    db: impl PgExecutor<'_>,
    tree_id: i32,
    card_data: &CardData,
) -> Result<i32, ApiError> {
    let card_id = query!(
        "INSERT INTO cards (tree_id, front, back)
        VALUES ($1, $2, $3)
        RETURNING id",
        tree_id,
        card_data.front,
        card_data.back,
    )
    .fetch_one(db)
    .await?
    .id;

    Ok(card_id)
}

/// Duplicates a card in the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `card_id` - The id of the card that needs to be duplicated as [`i32`].
///
/// `tree_id` - The id of the tree to which the duplicated card needs to go, as [`i32`].
///
/// # Returns
/// The id assigned to the new card if successful.
///
/// # Errors
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn duplicate_card(
    db: impl PgExecutor<'_>,
    card_id: i32,
    tree_id: i32,
) -> Result<i32, ApiError> {
    query!(
        "
            WITH original AS (
                SELECT front, back
                FROM cards
                WHERE id = $1
            )
            INSERT INTO cards (tree_id, front, back)
            SELECT $2, original.front, original.back
            FROM original
            RETURNING id
            ",
        card_id,
        tree_id,
    )
    .fetch_optional(db)
    .await?
    .ok_or(ApiError::MissingCard(card_id))
    .map(|r| r.id)
}

/// Remove a card from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `card_id` - The id of the card which should be removed.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn remove_card(db: impl PgExecutor<'_>, card_id: i32) -> Result<(), ApiError> {
    // We only return the id to handle the case where the card does not exist with fetch_optional().
    query!("DELETE FROM cards WHERE id = $1 RETURNING id", card_id,)
        .fetch_optional(db)
        .await?
        .ok_or(ApiError::MissingCard(card_id))?;

    Ok(())
}

/// Update a card in the database.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `card_id` - The card to update.
///
/// `card_data` - The new card data, as [`CardData`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_card(
    db: &mut PgConnection,
    card_id: i32,
    card_data: &CardData,
) -> Result<(), ApiError> {
    // We only return the id to handle the case where the card does not exist with fetch_optional().
    query!(
        "UPDATE cards SET (front, back, last_modified) = ($2, $3, CURRENT_TIMESTAMP) WHERE id = $1 RETURNING id",
        card_id,
        card_data.front,
        card_data.back,
    )
    .fetch_optional(&mut *db)
    .await?
    .ok_or(ApiError::MissingCard(card_id))?;

    // Also update the `last_modified` field in the tree the card belongs to.
    query!(
        "UPDATE trees SET last_modified = CURRENT_TIMESTAMP
             WHERE id = (SELECT tree_id FROM cards where id = $1)",
        card_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Get all cards from a tree.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `tree_id` - The id of the tree of which the cards are asked.
///
/// # Returns
/// The cards of the tree as a [`BTreeMap`].
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_tree_cards(
    db: impl PgExecutor<'_>,
    tree_id: i32,
) -> Result<BTreeMap<i32, CardData>, ApiError> {
    let cards = query!(
        "SELECT id, front, back
        FROM cards
        WHERE tree_id = $1",
        tree_id
    )
    .fetch_all(db)
    .await?;

    Ok(cards
        .into_iter()
        .map(|card| {
            (
                card.id,
                CardData {
                    front: card.front,
                    back: card.back,
                },
            )
        })
        .collect())
}

/// Retrieves a single card from the cards database, if the card exists.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `card_id` - The id associated with the card, as [`i32`].
///
/// # Returns
/// The front and back of the card as [`CardData`].
///
/// # Errors
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_card_text(db: impl PgExecutor<'_>, card_id: i32) -> Result<CardData, ApiError> {
    let card = query_as!(
        CardData,
        "SELECT front, back
         FROM cards
         WHERE id = $1",
        card_id
    )
    .fetch_optional(db)
    .await?
    .ok_or(ApiError::MissingCard(card_id))?;

    Ok(card)
}

/// Get the id of the tree the card belongs to.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `card_id` - The id of the card of which the corresponding tree is queried.
///
/// # Returns
/// The id of the tree the card belongs to, according to the `cards` table in the database.
///
/// # Errors
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_tree_of_card(db: impl PgExecutor<'_>, card_id: i32) -> Result<i32, ApiError> {
    // We retrieve the `tree_id` of the tree the card belongs to, and by doing so we simultaneously
    // check whether the card with `card_id` exists in the database.
    let tree_id = query!("SELECT tree_id FROM cards where id=$1", card_id,)
        .fetch_optional(db)
        .await?
        .ok_or(ApiError::MissingCard(card_id))?
        .tree_id;

    Ok(tree_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test::{initialize_card, initialize_card_with_data, initialize_db};
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_add_card(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;

        let card_data = CardData {
            front: "front".to_owned(),
            back: "back".to_owned(),
        };

        let card_id = add_card(&db, tree_id, &card_data).await.unwrap();

        let found_card_data = query_as!(
            CardData,
            "SELECT front, back FROM cards WHERE id=$1",
            card_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(found_card_data, card_data);
    }

    #[sqlx::test]
    async fn test_remove_card(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        remove_card(&db, card_id).await.unwrap();

        let does_card_exist = query!("SELECT EXISTS (SELECT 1 FROM cards WHERE id=$1)", card_id)
            .fetch_one(&db)
            .await
            .unwrap()
            .exists
            .unwrap();
        assert!(!does_card_exist);

        // Assert removing the card again leads to an error.
        let err = remove_card(&db, card_id).await;
        assert_eq!(err, Err(ApiError::MissingCard(card_id)));
    }

    #[sqlx::test]
    async fn test_update_card(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;

        let missing_card_id = 0;
        let new_card_data = CardData {
            front: "FRONT".to_owned(),
            back: "BACK".to_owned(),
        };

        let error = update_card(get_connection_unwrap!(db), missing_card_id, &new_card_data)
            .await
            .unwrap_err();
        assert_eq!(error, ApiError::MissingCard(missing_card_id));

        let card_id = initialize_card(&db, tree_id).await;

        update_card(get_connection_unwrap!(db), card_id, &new_card_data)
            .await
            .unwrap();

        let found_card_data = query!("SELECT front, back FROM cards WHERE id=$1", card_id)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_card_data = CardData {
            front: found_card_data.front,
            back: found_card_data.back,
        };

        assert_eq!(found_card_data, new_card_data);
    }

    #[sqlx::test]
    async fn test_get_tree_cards(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_data1 = CardData {
            front: "front1".to_owned(),
            back: "back1".to_owned(),
        };
        let card_data2 = CardData {
            front: "front2".to_owned(),
            back: "back2".to_owned(),
        };

        let card_id1 = initialize_card_with_data(&db, tree_id, &card_data1).await;
        let card_id2 = initialize_card_with_data(&db, tree_id, &card_data2).await;

        let found_card_data = get_tree_cards(&db, tree_id).await.unwrap();

        let mut expected_card_data = BTreeMap::new();
        expected_card_data.insert(card_id1, card_data1);
        expected_card_data.insert(card_id2, card_data2);
        assert_eq!(found_card_data, expected_card_data);
    }

    #[sqlx::test]
    async fn test_get_card_text(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let card_text = get_card_text(&db, card_id).await;
        let expected_card_text = CardData {
            front: "front".to_owned(),
            back: "back".to_owned(),
        };
        assert_eq!(card_text, Ok(expected_card_text));
    }

    #[sqlx::test]
    async fn test_get_tree_of_card(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let found_tree_id = get_tree_of_card(&db, card_id).await;
        assert_eq!(found_tree_id, Ok(tree_id));
    }

    #[sqlx::test]
    async fn test_duplicate_card(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let duplicate_card_id = duplicate_card(&db, card_id, tree_id).await.unwrap();
        assert_ne!(duplicate_card_id, tree_id);

        let card_text = get_card_text(&db, duplicate_card_id).await.unwrap();
        let expected_card_text = CardData {
            front: "front".to_owned(),
            back: "back".to_owned(),
        };
        assert_eq!(card_text, expected_card_text);
    }
}
