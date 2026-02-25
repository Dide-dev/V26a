mod demo;
mod cards;

use crate::api_v1::demo::demo;
use crate::api_v1::cards::cards;
use axum::Router;

pub fn v1_api() -> Router {
    let demo_router = Router::new().nest("/demo", demo());
    let card_router = Router::new().nest("/cards", cards());

    Router::new().merge(demo_router).merge(card_router)
}
