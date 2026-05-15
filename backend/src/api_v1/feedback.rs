/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::api_v1::session::Claims;
use crate::db::feedback::{add_feedback, get_all_feedback, get_feedback_count};
use crate::db_structs::feedback::{FeedbackData, FeedbackListResponse};
use crate::db_structs::pagination::OptionalPaginationOptions;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the feedback category.
pub fn feedback() -> Router<ArcState> {
    Router::new()
        .route("/provide", post(provide_feedback))
        .route("/get", get(get_feedback))
}

#[utoipa::path(
    post,
    path = "/provide",
    context_path = "/api/v1/feedback",
    request_body = FeedbackData,
    responses(
        (status = 200, description = "Feedback successfully received."),
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
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn provide_feedback(
    Extension(db): Extension<PgPool>,
    feedback_data: Json<FeedbackData>,
) -> Result<(), ApiError> {
    add_feedback(&db, &feedback_data).await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/get",
    context_path = "/api/v1/feedback",
    responses(
        (status = 200, description = "Feedback successfully retrieved.", body = FeedbackListResponse),
        (status = 400, description = "Requested page or page size has exceeded a limit."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Option<Claims>, Cookie),
        ("page" = OptionalPaginationOptions, Query),
    )
)]
/// Provide feedback on the website.
///
/// # Arguments
/// `feedback_data` - The data given as feedback, as [`FeedbackData`].
///
/// # Returns
/// A list with the feedback at the requested page.
///
/// # Errors
/// Returns [`MaxPageSizeExceeded`](ApiError::MaxPageSizeExceeded) if the user requests more than
/// 100 items per page.
///
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if  the user is not logged in.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to retrieve the feedback.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) or
/// [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) instead.
async fn get_feedback(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Query(page): Query<OptionalPaginationOptions>,
) -> Result<Json<FeedbackListResponse>, ApiError> {
    if claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }

    let page = page.to_some_with_default(1, 50, 1000, 100)?;

    let feedback_count = get_feedback_count(&db).await?;

    if feedback_count == 0 {
        return Ok(Json(FeedbackListResponse {
            feedback_total: 0,
            feedback: Vec::new(),
        }));
    }

    let feedback = get_all_feedback(&db, &page).await?;

    let list = FeedbackListResponse {
        feedback_total: i32::try_from(feedback_count).map_err(|_| ApiError::DatabaseMemoryCap)?,
        feedback,
    };

    Ok(Json(list))
}
