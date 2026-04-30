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

use crate::{
    api_v1::session::Claims,
    config::Config,
    db::users::{get_queried_count_users_pagination, get_queried_list_users_pagination,},
    db_structs::users::{AccountType, UserListResponse},
    errors::ApiError,
    db_structs::pagination::OptionalPaginationOptions,
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
pub fn admin(config: Config) -> Router<Config> {
    Router::new()
        .route("/all-user-data", get(get_all_user_data))
        .route_layer(middleware::from_fn_with_state(config, check_if_admin))
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
    )
)]

/// Gets the data of all the users from the database.
///
/// # Arguments
///
/// `page` - The requested page, given as [`OptionalPaginationOptions`] through a query.
///
/// # Returns
/// the data form all users and the amount of pages as [`UserListResponse>`].
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

    let mut transaction = db.begin().await.map_err(|_| ApiError::DatabaseException)?;
    let users_count = get_queried_count_users_pagination(&mut *transaction).await?;

    if users_count == 0 {
        return Ok(Json(UserListResponse {
            total_users: 0,
            users: Vec::new(),
        }));
    }
    let users = get_queried_list_users_pagination(&mut *transaction, &page).await?;
    transaction
        .commit()
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    let list = UserListResponse {
        total_users: i32::try_from(users_count).map_err(|_| ApiError::DatabaseMemoryCap)?,
        users,
    };

    Ok(Json(list))
}
