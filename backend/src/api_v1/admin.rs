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
    routing::{get, post},
};
use sqlx::PgPool;

use crate::db_structs::users::AdminUpdateData;
use axum::extract::{Path, State};

use crate::{
    api_v1::session::Claims,
    api_v1::users::{UserResponse, ChangePasswordResponse},
    config::Config,
    db::users::{admin_update_user, get_all_users, get_user_by_name, user_id_exists, update_password},
    db_structs::users::{AccountType, AllUsersGetData, AdminPasswordChangeRequestBody},
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
        .route("/users/{user_id}/update-data", post(update_user_data))
        .route("/users/{user_id}/update-password", post(change_password))
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
    post,
    path = "/users/{user_id}/update-data",
    context_path = "/api/v1/admin",
    request_body = AdminUpdateData,
    responses(
        (status = 200, description = "Successfully updated user with the returned id.", body = UserResponse, content_type = "application/json"),
        (status = 400, description = "User with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
        ("user_id" = i32, Path),
    )
)]
/// Update a user's data in the database..
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `user_id` - The user for whom to update their data.
///
/// `new_data` - The new data of the user, as [`AdminUpdateData`].
///
/// # Returns
/// Nothing.
///
/// # Errors
/// Returns [`MissingUser`](ApiError::MissingUser) if the given user id does not exist.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user does not have permission to change
/// the given user's data, or to update one of the specified fields.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_user_data(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Path(user_id): Path<i32>,
    Json(new_data): Json<AdminUpdateData>,
) -> Result<Json<UserResponse>, ApiError> {
    if !user_id_exists(&db, user_id).await? {
        return Err(ApiError::MissingUser(user_id));
    }

    let found_user_id = get_user_by_name(&db, &new_data.name).await?;

    if let Some(id) = found_user_id
        && id != claims.sub
    {
        return Ok(Json(UserResponse::UsernameAlreadyUsed));
    }

    admin_update_user(&db, user_id, &new_data).await?;

    Ok(Json(UserResponse::Success { user_id }))
}

#[utoipa::path(
    post,
    path = "/users/{user_id}/update-password",
    context_path = "/api/v1/admin",
    request_body = AdminPasswordChangeRequestBody,
    responses(
        (status = 200, description = "Password changing attempt has been finalized.", body = ChangePasswordResponse),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
        ("user_id" = i32, Path),
    )
)]
/// Changes a password in the database.
///
/// # Arguments
/// `user_id` - The user for whom to update their password.
///
/// `new_password` - A struct containing the new password.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn change_password(
    Extension(db): Extension<PgPool>,
    State(config): State<Config>,
    Path(user_id): Path<i32>,
    Json(new_password): Json<AdminPasswordChangeRequestBody>,
) -> Result<Json<ChangePasswordResponse>, ApiError> {

    update_password(
        &db,
        user_id,
        &new_password.new_password,
        config.bcrypt_cost,
    )
    .await?;

    Ok(Json(ChangePasswordResponse { succeeded: true }))
}
