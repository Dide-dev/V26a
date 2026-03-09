use crate::Config;
use crate::db::cards::add_card;
use crate::db_structs::cards::CardData;

use crate::api_v1::session::Claims;
use crate::errors::ApiError;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the cards category.
pub fn cards() -> Router<Config> {
    Router::new().route("/add", post(add_new_card))
}

/// Add a card to the database
///
/// # Returns
/// The index assigned to the new card
async fn add_new_card(
    claims: Claims,
    db: Extension<PgPool>,
    Json(card_data): Json<CardData>,
) -> Result<Json<i32>, ApiError> {
    let id = add_card(&db, card_data, claims.sub).await?;

    let json_id = Json(id);
    Ok(json_id)
}
