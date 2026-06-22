/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::errors::ApiError;
use crate::structs::feedback::{FeedbackData, FeedbackGetData};
use crate::structs::pagination::PaginationOptions;
use sqlx::{PgExecutor, query, query_as, query_scalar};

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
    .await?;

    Ok(())
}

/// Get feedback from the database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `page` - Which page is requested, and its size, as [`PaginationOptions`].
///
/// # Returns
/// A list with the feedback.
///
/// # Errors
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_all_feedback(
    db: impl PgExecutor<'_>,
    page: &PaginationOptions,
) -> Result<Vec<FeedbackGetData>, ApiError> {
    let limit = i64::from(page.get_size());
    let offset = i64::from((page.get_page() - 1) * page.get_size());

    let feedback = query_as!(
        FeedbackGetData,
        "SELECT id AS feedback_id, email, feedback, created_at
        FROM feedback
        ORDER BY created_at DESC \
        LIMIT $1
        OFFSET $2",
        limit,
        offset
    )
    .fetch_all(db)
    .await?;

    // User requested a page that yields no results. This should only occur if the user requests a
    // page that is too high, as get_queried_count should be used to confirm results exist.
    if feedback.is_empty() {
        return Err(ApiError::InvalidPage);
    }

    Ok(feedback)
}

/// Get the number of feedback entries.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// # Returns
/// The number of feedback entries.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_feedback_count(db: impl PgExecutor<'_>) -> Result<i64, ApiError> {
    // `!` forces non-nullability. `COUNT(*)` is guaranteed to return a non-null value, so sqlx can safely refer the count as i64, instead of Option<i64>.
    let feedback_count = query_scalar!(r#"SELECT COUNT(*) AS "count!" FROM feedback"#)
        .fetch_one(db)
        .await?;

    Ok(feedback_count)
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

    #[sqlx::test]
    async fn test_get_feedback(db: PgPool) {
        initialize_db(get_connection_unwrap!(db)).await;

        let feedback_data = FeedbackData {
            email: "test@test.nl".to_owned(),
            feedback: "feedback".to_owned(),
        };

        add_feedback(&db, &feedback_data).await.unwrap();

        let page = PaginationOptions::new(1, 50, 1000, 500).unwrap();
        let feedback_count = get_feedback_count(&db).await.unwrap();
        assert_eq!(feedback_count, 1);

        let found_feedback_data = get_all_feedback(&db, &page).await.unwrap();

        assert!(found_feedback_data.len() == 1);

        let found_feedback = &found_feedback_data[0];

        assert_eq!(found_feedback.email, feedback_data.email);
        assert_eq!(found_feedback.feedback, feedback_data.feedback);
    }
}
