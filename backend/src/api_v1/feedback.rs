/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::Config;
use crate::db::feedback::add_feedback;
use crate::db_structs::feedback::FeedbackData;
use crate::errors::ApiError;
use axum::routing::post;
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the feedback category.
pub fn feedback() -> Router<Config> {
    Router::new().route("/provide", post(provide_feedback))
}

#[utoipa::path(
    post,
    path = "/provide",
    context_path = "/api/v1/feedback",
    request_body = FeedbackData,
    responses(
        (status = 200, description = "Feedback succesfully added"),
        (status = 500, description = "Internal server error."),
    ),
)]
/// Add feedback to the database.
///
/// # Arguments
/// `feedback_data` - The data given as feedback, as [`FeedbackData`].
///
/// # Returns
/// Nothing
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn provide_feedback(
    Extension(db): Extension<PgPool>,
    feedback_data: Json<FeedbackData>,
) -> Result<(), ApiError> {
    add_feedback(&db, &feedback_data).await?;

    Ok(())
}
