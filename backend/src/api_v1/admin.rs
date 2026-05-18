/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use axum::{
    Extension, Json, Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use axum_extra::extract::Query;
use sqlx::PgPool;

use crate::db::feedback::{get_all_feedback, get_feedback_count};
use crate::db_structs::feedback::FeedbackListResponse;
use crate::{
    ArcState,
    db::users::{get_user_count, get_user_list_by_page},
    db_structs::pagination::OptionalPaginationOptions,
    db_structs::session::Claims,
    db_structs::users::{AccountType, UserListResponse},
    errors::ApiError,
};

/// Checks if the user is an admin. This is used as a middleware in the `admin` function below.
async fn check_if_admin(
    claims: Claims,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    match claims.account_type {
        AccountType::Admin => {
            let response = next.run(request).await;
            Ok(response)
        }
        AccountType::Author => Err(ApiError::NoPermission),
    }
}

/// Creates an `Axum` [`Router`] for the user category.
pub fn admin(state: ArcState) -> Router<ArcState> {
    Router::new()
        .route("/all-user-data", get(get_all_user_data))
        .route("/get-feedback", get(get_feedback))
        .route_layer(middleware::from_fn_with_state(state, check_if_admin))
}

#[utoipa::path(
    get,
    path = "/all-user-data",
    context_path = "/api/v1/admin",
    responses(
        (status = 200, description = "Successfully retrieved data.", body = UserListResponse),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("page" = OptionalPaginationOptions, Query),
    )
)]

/// Gets the data of all the users from the database.
///
/// # Arguments
/// `page` - The requested page, given as [`OptionalPaginationOptions`] through a query.
///
/// # Returns
/// the data form all users and the amount of pages as [`UserListResponse`].
///
/// # Errors
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to get
/// the data of all users.
///
/// Returns [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) if the limit on the database memory disallows the user count.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_all_user_data(
    Extension(db): Extension<PgPool>,
    Query(page): Query<OptionalPaginationOptions>,
) -> Result<Json<UserListResponse>, ApiError> {
    let page = page.to_some_with_default(1, 50, 1000, 100)?;

    let mut transaction = db.begin().await?;
    let users_count = get_user_count(&mut *transaction).await?;

    let users = get_user_list_by_page(&mut *transaction, &page).await?;
    transaction.commit().await?;

    let list = UserListResponse {
        total_users: i32::try_from(users_count).map_err(|_| ApiError::DatabaseMemoryCap)?,
        users,
    };

    Ok(Json(list))
}

#[utoipa::path(
    get,
    path = "/get-feedback",
    context_path = "/api/v1/admin",
    responses(
        (status = 200, description = "Feedback successfully retrieved.", body = FeedbackListResponse),
        (status = 400, description = "Requested page or page size has exceeded a limit."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("page" = OptionalPaginationOptions, Query),
    )
)]
/// Retrieve feedback from the database.
///
/// # Arguments
/// `page` - The requested page, given as [`OptionalPaginationOptions`] through a query.
///
/// # Returns
/// A list with the feedback at the requested page.
///
/// # Errors
/// Returns [`MaxPageSizeExceeded`](ApiError::MaxPageSizeExceeded) if the user requests more than
/// the defined number of items per page, currently 100.
///
/// Returns [`MaxPageNumberExceeded`](ApiError::MaxPageNumberExceeded) if the requested page number is greater than the given page number, currently configured to 1000.
///
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if  the user is not logged in.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to retrieve the feedback. Only admins have this permission.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) or
/// [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) instead.
async fn get_feedback(
    Extension(db): Extension<PgPool>,
    Query(page): Query<OptionalPaginationOptions>,
) -> Result<Json<FeedbackListResponse>, ApiError> {
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
