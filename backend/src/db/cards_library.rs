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
    owner_id: i32,
    title: String,
    tags: &[String],
    card_text: CardGetData
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
