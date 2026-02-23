use crate::db::users::add_user;
use crate::errors::ApiError;
use crate::users::UserData;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

pub fn users() -> Router {
    Router::new().route("/add_new_user", post(add_new_user))
}

async fn add_new_user(
    db: Extension<PgPool>,
    axum::Json(user_data): axum::Json<UserData>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let res = add_user(&db, user_data).await;

    match res {
        Ok(id) => Ok(Json(id)),
        Err(ApiError::UserAlreadyExistsError) => {
            Err((StatusCode::BAD_REQUEST, "User already exists".to_string()))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unexpected error occured".to_string(),
        )),
    }
}
