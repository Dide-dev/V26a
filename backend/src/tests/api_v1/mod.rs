mod demo;

use axum::{Extension, Router};
use sqlx::PgPool;

fn setup(db: PgPool) -> Router {
    crate::api_v1::v1_api().layer(Extension(db)).with_state(crate::Config::load())
}
