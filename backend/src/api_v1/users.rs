/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::ArcState;
use crate::api_v1::{extract_token, rate_limit_layer};
use crate::db::auth::get_user_credentials_by_id;
use crate::db::classrooms::{get_all_classrooms, get_classroom_count};
use crate::db::session::{delete_all_sessions_except, delete_all_sessions_user};
use crate::db::users::{
    add_user, delete_user, get_user, get_user_by_email, get_user_by_name, update_password,
    update_user, user_id_exists,
};
use crate::db_structs::classrooms::ClassroomListResponse;
use crate::db_structs::pagination::OptionalPaginationOptions;
use crate::db_structs::session::{Claims, InfoCookieData};
use crate::db_structs::users::{
    DeleteOwnedTrees, PasswordChangeRequestBody, RegisterData, UserData, UserGetData,
};
use crate::errors::ApiError;

use crate::db_structs::Validated;
use axum::extract::State;
use axum::routing::{get, post, put};
use axum::{Extension, Json, Router};
use axum_extra::extract::Query;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Duration;
use utoipa::ToSchema;

/// Creates an `Axum` [`Router`] for the user category.
pub fn users() -> Router<ArcState> {
    Router::new()
        .route("/", post(add_new_user).layer(rate_limit_layer(5, 20)))
        .route(
            "/me",
            get(get_own_user_data)
                .put(update_user_data)
                .delete(delete_user_data),
        )
        .route("/me/password", put(change_password))
        .route("/me/classrooms", get(get_all_owned_classrooms))
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
#[serde(rename_all_fields = "camelCase")]
pub enum UserResponse {
    /// Return body signaling the user has been successfully added/updated with the contained `user_id`.
    Success { user_id: i32 },
    /// Return body signaling the user could not be added/updated, as the e-mail was already in use.
    EmailAlreadyUsed,
    /// Return body signaling the user could not be added/updated, as the username was already in use.
    UsernameAlreadyUsed,
}

#[utoipa::path(
    post,
    path = "",
    context_path = "/api/v1/users",
    request_body = RegisterData,
    responses(
        (status = 200, description = "Successfully registered user with the returned id.", body = UserResponse, content_type = "application/json"),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 403, description = ApiError::NoPermission.to_string()),
        (status = 429, description = "Ratelimit exceeded."),
        (status = 500, description = "Internal server error."),
    )
)]
/// Add a user to the database.
///
/// # Arguments
/// `user_data` - The data used to register the user, as [`RegisterData`].
///
/// # Returns
/// [`Success`](UserResponse::Success) with the id assigned to the new user
/// if the email has not yet been used, or [`EmailAlreadyUsed`](UserResponse::EmailAlreadyUsed) otherwise.
/// if the username has not yet been used, or [`UsernameAlreadyUsed`](UserResponse::UsernameAlreadyUsed) otherwise.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn add_new_user(
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(user_data): Json<RegisterData>,
) -> Result<Json<UserResponse>, ApiError> {
    user_data.validate_inputs()?;

    let mut transaction = db.begin().await?;

    let found_user_id = get_user_by_email(&mut *transaction, &user_data.email).await?;

    if found_user_id.is_some() {
        return Ok(Json(UserResponse::EmailAlreadyUsed));
    }

    let found_user_id = get_user_by_name(&mut *transaction, &user_data.data.name).await?;

    if found_user_id.is_some() {
        return Ok(Json(UserResponse::UsernameAlreadyUsed));
    }

    let user_id = add_user(&mut transaction, &user_data, state.config.bcrypt_cost).await?;

    transaction.commit().await?;

    Ok(Json(UserResponse::Success { user_id }))
}

#[utoipa::path(
    get,
    path = "/me",
    context_path = "/api/v1/users",
    responses(
        (status = 200, description = "Successfully retrieved own data.", body = UserGetData),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Get a user's own data from the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// # Returns
/// The user's data as [`UserGetData`].
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_own_user_data(
    claims: Claims,
    Extension(db): Extension<PgPool>,
) -> Result<Json<UserGetData>, ApiError> {
    get_user(&db, claims.sub).await.map(Json)
}

#[utoipa::path(
    put,
    path = "/me",
    context_path = "/api/v1/users",
    request_body = UserData,
    responses(
        (status = 200, description = "Successfully updated user with the returned id.", body = UserResponse, content_type = "application/json"),
        (status = 400, description = "User with given id cannot be found."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Update a user's own data in the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `new_data` - The new data of the user, as [`UserData`].
///
/// # Returns
/// [`Success`](UserResponse::Success) with the id assigned to the new user, if successful,
/// or [`UsernameAlreadyUsed`](UserResponse::UsernameAlreadyUsed) if the username is already in use.
/// Also updates the cookies.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn update_user_data(
    mut jar: CookieJar,
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Json(new_data): Json<UserData>,
) -> Result<(CookieJar, Json<UserResponse>), ApiError> {
    new_data.validate_inputs()?;
    let found_user_id = get_user_by_name(&db, &new_data.name).await?;

    if let Some(id) = found_user_id
        && id != claims.sub
    {
        return Ok((jar, Json(UserResponse::UsernameAlreadyUsed)));
    }

    update_user(&db, claims.sub, &new_data).await?;

    let new_user_info_cookie = InfoCookieData {
        name: new_data.name,
        nickname: new_data.nickname,
        sub: claims.sub,
        account_type: claims.account_type,
    };
    let new_user_info = serde_json::to_string(&new_user_info_cookie)?;
    let time = Duration::seconds(claims.exp - Utc::now().timestamp());
    let user_cookie = Cookie::build(("user_info", new_user_info))
        .path("/")
        .secure(true)
        .max_age(time)
        .http_only(false);
    // `add` automatically replaces the cookie values, `remove` sets the cookie to an empty value and the maxAge to 0.
    jar = jar.add(user_cookie);

    Ok((
        jar,
        Json(UserResponse::Success {
            user_id: claims.sub,
        }),
    ))
}

/// Return body, where the `succeeded`-field signals whether the request was successful.
/// - If the flag is `false`, the wrong credentials were passed.
/// - If the flag is `true`, the right credentials were passed and the password change was successful.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordResponse {
    pub succeeded: bool,
}

#[utoipa::path(
    put,
    path = "/me/password",
    context_path = "/api/v1/users",
    request_body = PasswordChangeRequestBody,
    responses(
        (status = 200, description = "Password changing attempt has been finalized.", body = ChangePasswordResponse),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("session" = Claims, Cookie),
    )
)]
/// Changes a user's own password in the database.
///
/// # Arguments
/// `password_change_request` - The struct containing the user's old password and the new password,
/// as [`PasswordChangeRequestBody`].
///
/// # Returns
/// A boolean whether the password change was successful, as [`ChangePasswordResponse`].
///
/// # Errors
/// May return a [`HashException`](ApiError::HashException) or
/// [`DatabaseException`](ApiError::DatabaseException) instead.
async fn change_password(
    claims: Claims,
    jar: CookieJar,
    Extension(db): Extension<PgPool>,
    State(state): State<ArcState>,
    Json(password_change_request): Json<PasswordChangeRequestBody>,
) -> Result<Json<ChangePasswordResponse>, ApiError> {
    password_change_request.validate_inputs()?;

    let credentials = get_user_credentials_by_id(&db, claims.sub)
        .await?
        .ok_or(ApiError::DatabaseException)?;

    let password_valid = bcrypt::verify(
        password_change_request.old_password,
        &credentials.password_hash,
    )
    .map_err(|_| ApiError::HashException)?;

    if !password_valid {
        return Ok(Json(ChangePasswordResponse { succeeded: false }));
    }

    update_password(
        &db,
        claims.sub,
        &password_change_request.new_password,
        state.config.bcrypt_cost,
    )
    .await?;

    let token = extract_token(&jar)?;

    // All other sessions of the account are invalidated if password is changed.
    delete_all_sessions_except(&db, claims.sub, &token).await?;

    Ok(Json(ChangePasswordResponse { succeeded: true }))
}

#[utoipa::path(
    delete,
    path = "/me",
    context_path = "/api/v1/users",
    responses(
        (status = 200, description = "User data successfully deleted."),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error.")
    ),
    params(
        ("deleteOwnedTrees" = DeleteOwnedTrees, Query),
        ("session" = Claims, Cookie),
    )
)]
/// Delete the logged-in user from the database.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `deleteOwnedTrees` - Delete all the trees of the user.
///
/// # Returns
/// The cookie jar with the session cookies removed.
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if you're not logged in.
///
/// Returns [`MissingUser`](ApiError::MissingUser) if the given user id does not exist.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn delete_user_data(
    jar: CookieJar,
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Query(delete): Query<DeleteOwnedTrees>,
) -> Result<CookieJar, ApiError> {
    let mut transaction = db.begin().await?;

    if !user_id_exists(&mut *transaction, claims.sub).await? {
        return Err(ApiError::MissingUser(claims.sub));
    }

    delete_user(&mut transaction, claims.sub, delete.delete_owned_trees).await?;

    // Invalidate all the sessions based on userid.
    delete_all_sessions_user(&mut *transaction, claims.sub).await?;

    transaction.commit().await?;

    // Unset the cookies by changing the date to a time in the past.
    Ok(jar
        .remove(Cookie::build("session").path("/api/"))
        .remove(Cookie::build("user_info").path("/")))
}

#[utoipa::path(
    get,
    path = "/{user_id}/classrooms",
    context_path = "/api/v1/users",
    responses(
        (status = 200, description = "Successfully retrieved all classrooms", body = ClassroomListResponse),
        (status = 401, description = ApiError::MissingAuthentication.to_string()),
        (status = 500, description = "Internal server error."),
    ),
    params(
        ("session" = Claims, Cookie),
        ("page" = OptionalPaginationOptions, Query),
    )
)]
/// Get the list of all classrooms a user has on a given page.
///
/// # Arguments
/// `claims` - The session cookie of the user.
///
/// `page` - The requested page, given as [`OptionalPaginationOptions`] through a query.
///
/// # Returns
/// All the user's classrooms and the number of classrooms as [`ClassroomListResponse`].
///
/// # Errors
/// Returns [`MissingAuthentication`](ApiError::MissingAuthentication) if the user isn't logged in.
///
/// Returns [`DatabaseMemoryCap`](ApiError::DatabaseMemoryCap) if the limit on the database memory disallows the classroom count.
///
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the classrooms data in the database is
/// invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
async fn get_all_owned_classrooms(
    claims: Claims,
    Extension(db): Extension<PgPool>,
    Query(page): Query<OptionalPaginationOptions>,
) -> Result<Json<ClassroomListResponse>, ApiError> {
    let page = page.to_some_with_default(1, 50, 1000, 100)?;

    let mut transaction = db.begin().await?;
    let classroom_count = get_classroom_count(&mut *transaction, claims.sub).await?;

    let classrooms = get_all_classrooms(&mut *transaction, claims.sub, &page).await?;
    transaction.commit().await?;
    let list = ClassroomListResponse {
        total_classrooms: i32::try_from(classroom_count)
            .map_err(|_| ApiError::DatabaseMemoryCap)?,
        classrooms,
    };
    Ok(Json(list))
}
