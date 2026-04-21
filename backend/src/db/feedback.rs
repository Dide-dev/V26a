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
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `feedback_data` - The feedback given as [`FeedbackData`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_feedback(
    db: impl PgExecutor<'_>,
    feedback_data: &FeedbackData,
) -> Result<(), ApiError> {
    query!(
        "INSERT INTO feedback (email, feedback)
        VALUES ($1, $2)",
        feedback_data.email,
        feedback_data.feedback,
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
            email: "test@test.nl".to_owned(),
            feedback: "feedback".to_owned(),
        };

        add_feedback(&db, &feedback_data).await.unwrap();

        let found_feedback_data = query!("SELECT id, email, feedback FROM feedback WHERE id=$1", 1)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_feedback_data = FeedbackData {
            email: found_feedback_data.email,
            feedback: found_feedback_data.feedback,
        };

        assert_eq!(found_feedback_data, feedback_data);
    }
}
