use crate::db::cards::add_card;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Router};
use sqlx::PgPool;

pub fn cards() -> Router {
    Router::new()
        .route("/add_new_card", post(add_new_card))
}

#[derive(serde::Deserialize)]
pub struct Card {
    owner_id: i32,
    front: String,
    back: String,
}

/// Add a tree to the database
/// Returns the index assigned to the new tree
async fn add_new_card(
    db: Extension<PgPool>,
    axum::Json(card): axum::Json<Card>,
) -> Result<(), StatusCode> {
    let _id = add_card(&db, card.owner_id, &card.front, &card.back)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}