/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use axum::{
    Extension, Json, Router,
    extract::Path,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use sqlx::PgPool;

use crate::{
    api_v1::session::Claims,
    config::Config,
    db::users::{get_all_users, get_user_all_data, user_id_exists},
    db_structs::users::{AccountType, AllUsersGetData, UserGetAllData},
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
        .route("/users/{userId}", get(get_user_data))
        .route_layer(middleware::from_fn_with_state(config, check_if_admin))
}

#[utoipa::path(
    get,
    path = "/all-user-data",
    context_path = "/api/v1/admin",
    responses(
        (status = 200, description = "Successfully retrieved data.", body = Vec<AllUsersGetData>),
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
/// the data form all users as [`Vec<AllUsersGetData>`].
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
) -> Result<Json<Vec<AllUsersGetData>>, ApiError> {
    get_all_users(&db).await.map(Json)
}

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    context_path = "/api/v1/admin",
    responses(
        (status = 200, description = "Successfully retrieved data.", body = UserGetAllData),
        (status = 400, description = "User with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("user_id" = i32, Path),
    )
)]
/// Gets the data of one user from the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// # Returns
/// The data form of a user as [`UserGetAllData`].
///
/// # Errors
/// Returns [`MissingUser`](ApiError::MissingUser) if the requested user does not exist in the database.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to get
/// the data of all users.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_user_data(
    Extension(db): Extension<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserGetAllData>, ApiError> {
    if !user_id_exists(&db, user_id).await? {
        return Err(ApiError::MissingUser(user_id));
    }
    get_user_all_data(&db, user_id).await.map(Json)
}
