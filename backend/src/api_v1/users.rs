use crate::db::users::add_user;
use crate::errors::ApiError;
use crate::users::UserData;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub fn users() -> Router {
    Router::new().route("/add-new-user", post(add_new_user))
}

async fn add_new_user(
    db: Extension<PgPool>,
    Json(user_data): Json<UserData>,
) -> Result<Json<i32>, ApiError> {
    add_user(&db, user_data).await.map(Json)
}
