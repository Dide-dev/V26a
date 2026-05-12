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
use std::sync::Arc;
use std::time::Duration;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;

/// Creates an `Axum` [`Router`] for the feedback category.
pub fn feedback() -> Router<Config> {
    // Ratelimiter for endpoint feedback
    let feedback_ratelimiter = Arc::new(
        GovernorConfigBuilder::default()
            .period(Duration::from_millis(60_000 / 5))
            .burst_size(10)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    Router::new().route(
        "/provide",
        post(provide_feedback).layer(GovernorLayer::new(feedback_ratelimiter)),
    )
}

#[utoipa::path(
    post,
    path = "/provide",
    context_path = "/api/v1/feedback",
    request_body = FeedbackData,
    responses(
        (status = 200, description = "Feedback successfully received."),
        (status = 429, description = "Ratelimit exceeded"),
        (status = 500, description = "Internal server error."),
    ),
)]
/// Provide feedback on the website.
///
/// # Arguments
/// `feedback_data` - The data given as feedback, as [`FeedbackData`].
///
/// # Returns
/// Nothing.
///
/// # Errors
///
/// May return a [`RateLimitExceeded`](ApiError::RateLimitExceeded) if the rate limit is exceeded.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn provide_feedback(
    Extension(db): Extension<PgPool>,
    feedback_data: Json<FeedbackData>,
) -> Result<(), ApiError> {
    add_feedback(&db, &feedback_data).await?;

    Ok(())
}
