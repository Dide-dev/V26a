/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use sqlx::{query, PgExecutor};
use crate::db_structs::cards::CardGetData;
use crate::errors::ApiError;

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
