/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db::classrooms::{add_classroom, add_lesson, generate_random_code, get_classroom_join_code_from_id, get_classroom_metadata_from_id, get_classroom_owner_from_id, get_lessons_with_tree_ids_from_classroom};
use crate::db_structs::Validated;
use crate::db_structs::classrooms::{LessonAddData, ClassroomAddData, ClassroomTeacherDashboardData, ClassroomGeneratedData};
use crate::db_structs::session::Claims;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::get_connection;

use axum::extract::Path;
use axum::{Extension, Json};
use sqlx::{PgExecutor, PgPool};

#[utoipa::path(
    post,
    path = "",
    context_path = "/api/v1/classrooms",
    request_body = ClassroomAddData,
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
pub(super) async fn add_new_classroom(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(classroom_add_data): Json<ClassroomAddData>,
) -> Result<Json<ClassroomGeneratedData>, ApiError> {
    classroom_add_data.validate_inputs()?;
    let join_code = generate_random_code(get_connection!(db)).await?;
    
    let id = add_classroom(&db, &classroom_add_data, claims.sub, &join_code).await?;

    Ok(Json(ClassroomGeneratedData {
        id,
        join_code,
    }))
}

#[utoipa::path(
    post,
    path = "/{classroom_id}/teacher-dash",
    context_path = "/api/v1/classrooms",
    responses(
        (status = 200, description = "Successfully retrieved classroom data", body = ClassroomTeacherDashboardData),
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
/// Retrieves all information necessary to construct the classroom student dashboard on the frontend.
///
/// # Arguments
/// `claims` - The session cookie of the teacher.
///
/// `classroom_id` - The classroom id retrieved from the path.
///
/// # Returns
/// Returns a [`ClassroomTeacherDashboardData`] if successful.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user cannot view this classroom.
///
/// May also return a [`DatabaseException`](ApiError::DatabaseException).
pub(super) async fn get_teacher_dashboard(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(classroom_id): Path<i32>,
) -> Result<Json<ClassroomTeacherDashboardData>, ApiError> {
    verify_classroom_permissions(&db, classroom_id, &claims).await?;

    let metadata = get_classroom_metadata_from_id(&db, classroom_id).await?;
    let lessons = get_lessons_with_tree_ids_from_classroom(&db, classroom_id).await?;
    let join_code = get_classroom_join_code_from_id(&db, classroom_id).await?;

    Ok(Json(ClassroomTeacherDashboardData {
        metadata,
        lessons,
        join_code,
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
pub(super) async fn add_new_lesson(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(classroom_id): Path<i32>,
    Json(lesson_add_data): Json<LessonAddData>,
) -> Result<Json<i32>, ApiError> {
    lesson_add_data.validate_inputs()?;
    // Check if the classroom exists and the user is permitted to manage it.
    verify_classroom_permissions(&db, classroom_id, &claims).await?;
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
    db: impl PgExecutor<'_>,
    classroom_id: i32,
    claims: &Claims,
) -> Result<(), ApiError> {
    let owner_id = get_classroom_owner_from_id(db, classroom_id)
        .await?
        .ok_or(ApiError::MissingClassroom(classroom_id))?;
    if claims.account_type == AccountType::Admin || claims.sub == owner_id {
        Ok(())
    } else {
        Err(ApiError::NoPermission)
    }
}
