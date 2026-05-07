/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::session::Claims;
use crate::db::classrooms::{add_classroom, generate_random_code};
use crate::db_structs::classrooms::{ClassroomAddData, ClassroomCreatedData};
use crate::errors::ApiError;
use crate::{Config, get_connection};

use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the classrooms category.
pub fn classrooms() -> Router<Config> {
    Router::new().route("/add", post(add_new_classroom))
}

#[utoipa::path(
    post,
    path = "/add",
    context_path = "/api/v1/classrooms",
    responses(
        (status = 200, description = "Successfully added new classroom."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie)
    )
)]
/// Adds a classroom to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `classroom_add_data` - The data used to create the classroom, as [`ClassroomAddData`].
///
/// # Returns
/// The id assigned to the new classroom and the generated join code in a [`ClassroomCreatedData`].
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user isn't logged in.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_classroom(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(classroom_add_data): Json<ClassroomAddData>,
) -> Result<Json<ClassroomCreatedData>, ApiError> {
    let join_code = generate_random_code(get_connection!(db)).await?;

    let id = add_classroom(&db, &classroom_add_data, claims.sub, &join_code).await?;

    Ok(Json(ClassroomCreatedData { id, join_code }))
}
