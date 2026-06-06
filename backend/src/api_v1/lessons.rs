/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::api_v1::classrooms::verify_classroom_permissions;
use crate::api_v1::trees::verify_tree_permissions;
use crate::db::classrooms::{add_existing_tree_to_lesson, get_classroom_from_lesson};
use crate::db_structs::session::Claims;
use crate::errors::ApiError;
use crate::get_connection;
use crate::state::ArcState;
use axum::extract::Path;
use axum::routing::post;
use axum::{Extension, Router};
use sqlx::{PgConnection, PgPool};

/// Creates an `Axum` [`Router`] for the classrooms category.
pub fn lessons() -> Router<ArcState> {
    Router::new().route("/{lesson_id}/trees/{tree_id}", post(add_tree_to_lesson))
}

#[utoipa::path(
    post,
    path = "/{lesson_id}/trees/{tree_id}",
    context_path = "/api/v1/lessons",
    responses(
        (status = 200, description = "Successfully added tree to lesson."),
        (status = 400, description = "Lesson or tree with given id cannot be found, or tree already exists in the lesson."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("lesson_id" = i32, Path),
        ("tree_id" = i32, Path),
    )
)]
/// Adds an existing tree to a lesson.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `lesson_id` - The id of the lesson to which the tree needs to be added.
///
/// `tree_id` - The id of the tree which is to be added to the lesson.
///
/// # Returns
/// Nothing
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// Returns [`MissingLesson`](ApiError::MissingLesson) if the lesson to which the tree should be added does not exist.
///
/// Returns [`MissingTree`](ApiError::MissingTree) if the tree which should be added to the lesson does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have the permission to add edit the tree or classroom.
///
/// Returns [`TreeAlreadyExistsWithinLesson`](ApiError::TreeAlreadyExistsWithinLesson) if the tree already exists in the lesson.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_tree_to_lesson(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path((lesson_id, tree_id)): Path<(i32, i32)>,
) -> Result<(), ApiError> {
    verify_lesson_permissions(get_connection!(db), lesson_id, &claims).await?;
    verify_tree_permissions(get_connection!(db), tree_id, &claims).await?;
    add_existing_tree_to_lesson(get_connection!(db), lesson_id, tree_id).await?;
    Ok(())
}

/// Checks whether the lesson exists and if it belongs to a classroom. It checks edit permission through classroom ownership, or if the user is an admin.
/// As of now this function does not take into account eventual collaborators as this is currently not implemented for classrooms or lessons,
/// this function would need to be updated if this were the case in the future.
///
/// This function uses the `verify_classroom_permissions` function internally.
///
/// # Arguments
/// `lesson_id` - The id of the classroom whose ownership needs to be verified.
///
/// `claims` - The session cookie of the user, used to verify ownership.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to modify the lesson for the given `lesson_id`.
///
/// Returns [`MissingLesson`](ApiError::MissingTree) if there is no lesson with `lesson_id`.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn verify_lesson_permissions(
    db: &mut PgConnection,
    lesson_id: i32,
    claims: &Claims,
) -> Result<(), ApiError> {
    let classroom_id = get_classroom_from_lesson(&mut *db, lesson_id)
        .await?
        .ok_or(ApiError::MissingLesson(lesson_id))?;
    verify_classroom_permissions(&mut *db, classroom_id, claims).await?;
    Ok(())
}
