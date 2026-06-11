/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db::classrooms::{add_classroom, add_lesson, generate_random_code, get_classroom_owner};
use crate::db_structs::Validated;
use crate::db_structs::classrooms::{ClassroomData, ClassroomGeneratedData, LessonAddData};
use crate::db_structs::session::Claims;
use crate::errors::ApiError;
use crate::{ArcState, get_connection};

use crate::db_structs::users::AccountType;
use axum::extract::Path;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::{PgConnection, PgPool};

/// Creates an `Axum` [`Router`] for the classrooms category.
pub fn classrooms() -> Router<ArcState> {
    Router::new()
        .route("/", post(add_new_classroom))
        .route("/{classroom_id}/lessons", post(add_new_lesson))
}

#[utoipa::path(
    post,
    path = "",
    context_path = "/api/v1/classrooms",
    request_body = ClassroomData,
    responses(
        (status = 200, description = "Successfully added new classroom.", body = ClassroomGeneratedData),
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
/// `classroom_add_data` - The data used to create the classroom, as [`ClassroomData`].
///
/// # Returns
/// The id assigned to the new classroom and the generated join code in a [`ClassroomGeneratedData`].
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_classroom(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(classroom_add_data): Json<ClassroomData>,
) -> Result<Json<ClassroomGeneratedData>, ApiError> {
    classroom_add_data.validate_inputs()?;
    let join_code = generate_random_code(get_connection!(db)).await?;

    let (id, created_at) = add_classroom(&db, &classroom_add_data, claims.sub, &join_code).await?;

    Ok(Json(ClassroomGeneratedData {
        id,
        join_code,
        created_at,
        owner_id: claims.sub,
    }))
}

#[utoipa::path(
    post,
    path = "/{classroom_id}/lessons",
    context_path = "/api/v1/classrooms",
    request_body = LessonAddData,
    responses(
        (status = 200, description = "Successfully added new lesson."),
        (status = 400, description = "Classroom with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("classroom_id" = i32, Path),
    )
)]
/// Adds a lesson to the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `lesson_add_data` - The data for the lesson in the form of [`LessonAddData`].
///
/// `classroom_id` - The id of the classroom to which the lesson needs to belong.
///
/// # Returns
/// The id assigned to the new lesson.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// Returns [`MissingClassroom`](ApiError::MissingClassroom) if the classroom to which the lesson should be added does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have the permission to add lessons to the classroom.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_lesson(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(classroom_id): Path<i32>,
    Json(lesson_add_data): Json<LessonAddData>,
) -> Result<Json<i32>, ApiError> {
    lesson_add_data.validate_inputs()?;
    // Check if the classroom exists and the user is permitted to manage it.
    verify_classroom_permissions(get_connection!(db), classroom_id, &claims).await?;
    let id = add_lesson(get_connection!(db), &lesson_add_data, classroom_id).await?;

    Ok(Json(id))
}

/// Checks whether the classroom exists and if it is owned by the user, or if the user is an admin.
/// As of now this function does not take into account eventual collaborators as this is currently not implemented for classrooms,
/// this function would need to be updated if this were the case in the future.
///
/// # Arguments
/// `classroom_id` - The id of the classroom whose ownership needs to be verified.
///
/// `claims` - The session cookie of the user, used to verify ownership.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to modify the classroom for the given `classroom_id`.
///
/// Returns [`MissingClassroom`](ApiError::MissingClassroom) if there is no classroom with `classroom_id`.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn verify_classroom_permissions(
    db: &mut PgConnection,
    classroom_id: i32,
    claims: &Claims,
) -> Result<(), ApiError> {
    let owner_id = get_classroom_owner(&mut *db, classroom_id)
        .await?
        .ok_or(ApiError::MissingClassroom(classroom_id))?;
    if claims.account_type == AccountType::Admin || claims.sub == owner_id {
        Ok(())
    } else {
        Err(ApiError::NoPermission)
    }
}
