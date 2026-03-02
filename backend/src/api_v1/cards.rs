use crate::Config;
use crate::db::cards::add_card;
use crate::db_structs::cards::CardData;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the cards category.
pub fn cards() -> Router<Config> {
    Router::new().route("/add-new-card", post(add_new_card))
}

/// Add a card to the database
/// Returns the index assigned to the new tree
async fn add_new_card(
    db: Extension<PgPool>,
    Json(card_data): Json<CardData>,
) -> Result<Json<i32>, StatusCode> {
    let id = add_card(&db, card_data, -1)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json_id = Json(id);
    Ok(json_id)
}