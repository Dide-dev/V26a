/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::feedback::FeedbackData;
use crate::errors::ApiError;
use sqlx::{PgExecutor, query};

/// Add feedback to the database.
///
/// # Arguments
/// `db` - a [`&PgPool`](sqlx::PgPool).
///
/// `feedback_data` - The feedback given as [`FeedbackData`].
///
/// # Returns
/// Nothing
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_feedback(
    db: impl PgExecutor<'_>,
    feedback_data: &FeedbackData,
) -> Result<(), ApiError> {
    query!(
        "INSERT INTO feedback (email, positive_feedback, negative_feedback, additional, grade)
        VALUES ($1, $2, $3, $4, $5)",
        feedback_data.email,
        feedback_data.positive,
        feedback_data.negative,
        feedback_data.additional,
        feedback_data.grade,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}
