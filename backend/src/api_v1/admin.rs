/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use axum::{
    Extension, Json, Router,
    extract::Request,
    extract::{Path, State},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use axum_extra::extract::Query;
use sqlx::PgPool;

use crate::db::feedback::{get_all_feedback, get_feedback_count};
use crate::db_structs::{feedback::FeedbackListResponse, session::Claims, users::AdminUpdateData};

use crate::{
    ArcState,
    api_v1::users::{ChangePasswordResponse, UserResponse},
    db::users::{
        admin_update_user, get_user_all_data, get_user_by_name, get_user_count,
        get_user_list_by_page, update_password, user_id_exists,
    },
    db_structs::pagination::OptionalPaginationOptions,
    db_structs::users::{
        AccountType, AdminPasswordChangeRequestBody, UserGetAllData, UserListResponse,
    },
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
        .route("/users/{userId}", get(get_user_data))
        .route("/users/{user_id}/update-data", post(update_user_data))
        .route("/users/{user_id}/update-password", post(change_password))
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
/// The data from all users and the amount of pages as [`UserListResponse`].
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
/// Update a user's data in the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `user_id` - The user for whom to update their data.
///
/// `new_data` - The new data of the user, as [`AdminUpdateData`].
///
/// # Returns
/// A response as [`UserResponse`], which can be of the form `Success { user_id: i32 }`, `EmailAlreadyUsed` or `UsernameAlreadyUsed`.
///
/// # Errors
/// Returns [`MissingUser`](ApiError::MissingUser) if the given user id does not exist.
///
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) when there is no session.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user is not an admin.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_user_data(
    Extension(db): Extension<PgPool>,
    Path(user_id): Path<i32>,
    Json(new_data): Json<AdminUpdateData>,
) -> Result<Json<UserResponse>, ApiError> {
    if !user_id_exists(&db, user_id).await? {
        return Err(ApiError::MissingUser(user_id));
    }

    let found_user_id = get_user_by_name(&db, &new_data.name).await?;

    if let Some(id) = found_user_id
        && id != user_id
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
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
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
/// A response as [`ChangePasswordResponse`], which has a variable succeeded of the type bool.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) when there is no session.
///
/// Returns [`NoPermission`](ApiError::NoPermission) if the user is not an admin.
///
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn change_password(
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Path(user_id): Path<i32>,
    Json(new_password): Json<AdminPasswordChangeRequestBody>,
) -> Result<Json<ChangePasswordResponse>, ApiError> {
    update_password(
        &db,
        user_id,
        &new_password.new_password,
        state.config.bcrypt_cost,
    )
    .await?;

    Ok(Json(ChangePasswordResponse { succeeded: true }))
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
/// `user_id` - The user from whom we want to get the data.
///
/// # Returns
/// The data of a user as [`UserGetAllData`].
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
        ("session" = Claims, Cookie),
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
