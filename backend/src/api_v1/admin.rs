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
use sqlx::PgPool;

use crate::{
    api_v1::session::Claims,
    config::Config,
    db::users::get_all_users,
    db_structs::users::{AccountType, UserGetAllData},
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
        (status = 200, description = "Successfully retrieved data.", body = Vec<UserGetAllData>),
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
/// `claims` - The session cookie of the user.
///
/// # Returns
/// the data form all users as [`Vec<UserGetAllData>`].
///
/// # Errors
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to get
/// the data of all users.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_all_user_data(
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<UserGetAllData>>, ApiError> {
    get_all_users(&db).await.map(Json)
}
