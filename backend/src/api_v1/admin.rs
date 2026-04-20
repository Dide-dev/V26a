use axum::{Extension, Json, Router, routing::get};
use sqlx::PgPool;

use crate::{api_v1::session::Claims, config::Config, db::users::get_all_users, db_structs::users::{AccountType, UserGetAllData}, errors::ApiError};

/// Creates an `Axum` [`Router`] for the user category.
pub fn admin() -> Router<Config> {
    Router::new()
        .route("/all-user-data", get(get_all_user_data))
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
    claims: Claims,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<UserGetAllData>>, ApiError> {
    if claims.account_type != AccountType::Admin {
        return Err(ApiError::NoPermission);
    }
    get_all_users(&db).await.map(Json)
}
