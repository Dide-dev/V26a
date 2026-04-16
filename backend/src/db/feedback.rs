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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test::initialize_db;
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_add_feedback(db: PgPool) {
        initialize_db(get_connection_unwrap!(db)).await;

        let feedback_data = FeedbackData {
            email: Some("test@test.nl".to_owned()),
            positive: Some("Tops".to_owned()),
            negative: Some("Tips".to_owned()),
            additional: Some("Very cool".to_owned()),
            grade: Some(7),
        };

        add_feedback(&db, &feedback_data).await.unwrap();

        let found_feedback_data = query!("SELECT id, email, positive_feedback, negative_feedback, additional, grade FROM feedback WHERE id=$1", 1)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_feedback_data = FeedbackData {
            email: found_feedback_data.email,
            positive: found_feedback_data.positive_feedback,
            negative: found_feedback_data.negative_feedback,
            additional: found_feedback_data.additional,
            grade: found_feedback_data.grade,
        };

        assert_eq!(found_feedback_data, feedback_data);
    }
}
