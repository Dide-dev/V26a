/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use sqlx::{query, PgExecutor};
use crate::db_structs::cards::CardGetData;
use crate::errors::ApiError;

/// Add a card to the card library database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `owner_id` - The id of the owner as [`i32`].
///
/// `title` - The title of the card in the library as [`String`].
///
/// `tags` - The tags to find this card while filtering the cards library as [`&[String]`].
///
/// `card_text` - The front and back text of the card as [`CardGetData`].
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
    card_text: &CardGetData
) -> Result<i32, ApiError> {
    let new_card_id = query!(
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
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .id;

    Ok(new_card_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test::{initialize_card, initialize_db};
    use crate::get_connection_unwrap;
    use sqlx::PgPool;
    use crate::db::cards::get_card_text;
    use crate::db::trees::get_tree_owner;

    #[sqlx::test]
    async fn test_cards_library_add(db: PgPool) {
        let (_, tree_id) = initialize_db(get_connection_unwrap!(db)).await;
        let card_id = initialize_card(&db, tree_id).await;

        let owner_id = get_tree_owner(&db, tree_id).await.unwrap().unwrap();
        let title = String::from("title");
        let tags = [String::from("tag1"), String::from("tag2")];
        let card_text = get_card_text(&db, card_id).await.unwrap();

        let card_lib_id = cards_library_add(&db, &owner_id, &title, &tags, &card_text).await.unwrap();

        let found_card_data = query!("SELECT owner_id, title, tags, front, back FROM library_cards WHERE id=$1", card_lib_id)
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(found_card_data.owner_id, owner_id);
        assert_eq!(found_card_data.title, title);
        assert_eq!(found_card_data.tags.unwrap(), tags);
        assert_eq!(found_card_data.front, card_text.front);
        assert_eq!(found_card_data.back, card_text.back);
    }
}
