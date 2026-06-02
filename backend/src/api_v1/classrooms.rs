/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db::classrooms::{add_classroom, generate_random_code};
use crate::db_structs::Validated;
use crate::db_structs::classrooms::{ClassroomAddData, ClassroomAddReturnData, ClassroomAllData};
use crate::db_structs::session::Claims;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::{ArcState, get_connection};
use axum::extract::{Path};

use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the classrooms category.
pub fn classrooms() -> Router<ArcState> {
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
/// The id assigned to the new classroom and the generated join code in a [`ClassroomAddReturnData`].
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_classroom(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(classroom_add_data): Json<ClassroomAddData>,
) -> Result<Json<ClassroomAddReturnData>, ApiError> {
    classroom_add_data.validate_inputs()?;
    let join_code = generate_random_code(get_connection!(db)).await?;

    let id = add_classroom(&db, &classroom_add_data, claims.sub, &join_code).await?;

    Ok(Json(ClassroomAddReturnData { id, join_code }))
}

#[utoipa::path(
    get,
    path = "/get-by-user/{id}",
    context_path = "/api/v1/classrooms",
    responses(
        (status = 200, description = "Successfully retrieved all classrooms"),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("user_id" = i32, Path),
    )
)]
/// Adds a classroom to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `user_id` - The user for whom to get their classrooms.
///
/// # Returns
/// All the classrooms of a specific user as [`Vec<ClassroomAllData>`].
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_all_owned_classrooms(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<ClassroomAllData>>, ApiError> {
    if claims.account_type == AccountType::Admin || claims.sub == user_id{
        let classrooms = get_all_classrooms(user_id).await?;
        Ok(Json(classrooms))
    }
    else {
        Err(ApiError::NoPermission)
    }
}
