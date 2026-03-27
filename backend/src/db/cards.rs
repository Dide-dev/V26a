/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::cards::{CardAddData, CardGetData, CardUpdateData};
use std::collections::BTreeMap;

use crate::errors::ApiError;
use sqlx::{PgConnection, PgExecutor, query};

/// Add a new card to the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `card_data` - The data assigned to the new card as [`CardAddData`].
///
/// # Returns
/// The id assigned to the new card if successful.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_card(db: impl PgExecutor<'_>, card_data: &CardAddData) -> Result<i32, ApiError> {
    let card_id = query!(
        "INSERT INTO cards (tree_id, front, back)
        VALUES ($1, $2, $3)
        RETURNING id",
        card_data.tree_id,
        card_data.front,
        card_data.back,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .id;

    Ok(card_id)
}

/// Remove a card from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `card_id` - The id of the card which should be removed.
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn remove_card(db: impl PgExecutor<'_>, card_id: i32) -> Result<(), ApiError> {
    // We only return the id to handle the case where the card does not exist with fetch_optional().
    query!("DELETE FROM cards WHERE id = $1 RETURNING id", card_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .ok_or(ApiError::MissingCard(card_id))?;

    Ok(())
}

/// Update a card in the database.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `card_data` - The data used to update the card with, as [`CardUpdateData`].
///
/// # Returns
/// Nothing.
///
/// # Error
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_card(
    db: &mut PgConnection,
    card_data: &CardUpdateData,
) -> Result<(), ApiError> {
    // We only return the id to handle the case where the card does not exist with fetch_optional().
    query!(
        "UPDATE cards SET (front, back, last_modified) = ($2, $3, CURRENT_TIMESTAMP) WHERE id = $1 RETURNING id",
        card_data.card_id,
        card_data.new_front,
        card_data.new_back,
    )
    .fetch_optional(&mut *db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .ok_or(ApiError::MissingCard(card_data.card_id))?;

    // Also update the `last_modified` field in the tree the card belongs to.
    query!(
        "UPDATE trees SET last_modified = CURRENT_TIMESTAMP
             WHERE id = (SELECT tree_id FROM cards where id = $1)",
        card_data.card_id,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

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
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_tree_cards(
    db: impl PgExecutor<'_>,
    tree_id: i32,
) -> Result<BTreeMap<i32, CardGetData>, ApiError> {
    let cards = query!(
        "SELECT id, front, back
        FROM cards
        WHERE tree_id = $1",
        tree_id
    )
    .fetch_all(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(cards
        .into_iter()
        .map(|card| {
            (
                card.id,
                CardGetData {
                    front: card.front,
                    back: card.back,
                },
            )
        })
        .collect())
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
/// # Error
/// Returns [`MissingCard`](ApiError::MissingCard) if no card with `card_id` exists.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_tree_of_card(db: impl PgExecutor<'_>, card_id: i32) -> Result<i32, ApiError> {
    // We retrieve the `tree_id` of the tree the card belongs to, and by doing so we simultaneously
    // check whether the card with `card_id` exists in the database.
    let tree_id = query!("SELECT tree_id FROM cards where id=$1", card_id,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .ok_or(ApiError::MissingCard(card_id))?
        .tree_id;

    Ok(tree_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test::{initialize_card, initialize_db};
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_add_card(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;

        let card_data = CardAddData {
            tree_id,
            front: "front".to_owned(),
            back: "back".to_owned(),
        };

        let card_id = add_card(&db, &card_data).await.unwrap();

        let found_card_data = query!("SELECT id, front, back FROM cards WHERE id=$1", card_id)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_card_data = CardAddData {
            tree_id: found_card_data.id,
            front: found_card_data.front,
            back: found_card_data.back,
        };

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
    }

    #[sqlx::test]
    async fn test_update_card(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;

        let new_card_data = CardUpdateData {
            card_id: 0,
            new_front: "FRONT".to_owned(),
            new_back: "BACK".to_owned(),
        };

        let error = update_card(get_connection_unwrap!(db), &new_card_data)
            .await
            .unwrap_err();
        assert_eq!(error, ApiError::MissingCard(new_card_data.card_id));

        let card_id = initialize_card(&db, tree_id).await;

        let new_card_data = CardUpdateData {
            card_id,
            new_front: new_card_data.new_front,
            new_back: new_card_data.new_back,
        };

        let void = update_card(get_connection_unwrap!(db), &new_card_data).await;
        assert!(void.is_ok());

        let found_card_data = query!("SELECT id, front, back FROM cards WHERE id=$1", card_id)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_card_data = CardUpdateData {
            card_id: found_card_data.id,
            new_front: found_card_data.front,
            new_back: found_card_data.back,
        };

        assert_eq!(found_card_data, new_card_data);
    }

    #[sqlx::test]
    async fn test_get_tree_cards(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_add_data1 = CardAddData {
            tree_id,
            front: "front1".to_owned(),
            back: "back1".to_owned(),
        };
        let card_get_data1 = CardGetData {
            front: card_add_data1.front.clone(),
            back: card_add_data1.back.clone(),
        };
        let card_add_data2 = CardAddData {
            tree_id,
            front: "front2".to_owned(),
            back: "back2".to_owned(),
        };
        let card_get_data2 = CardGetData {
            front: card_add_data2.front.clone(),
            back: card_add_data2.back.clone(),
        };

        let card_id1 = add_card(&db, &card_add_data1).await;
        assert!(card_id1.is_ok());
        let card_id1 = card_id1.unwrap();
        let card_id2 = add_card(&db, &card_add_data2).await;
        assert!(card_id2.is_ok());
        let card_id2 = card_id2.unwrap();

        let found_card_data = get_tree_cards(&db, tree_id).await;
        assert!(found_card_data.is_ok());
        let found_card_data = found_card_data.unwrap();

        let mut expected_card_data = BTreeMap::new();
        expected_card_data.insert(card_id1, card_get_data1);
        expected_card_data.insert(card_id2, card_get_data2);
        assert_eq!(found_card_data, expected_card_data);
    }

    #[sqlx::test]
    async fn test_get_tree_of_card(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let found_tree_id = get_tree_of_card(&db, card_id).await;
        assert!(found_tree_id.is_ok());
        let found_tree_id = found_tree_id.unwrap();
        assert_eq!(found_tree_id, tree_id);
    }
}
