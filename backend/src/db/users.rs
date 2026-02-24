use crate::errors::ApiError;
use crate::users::UserData;

use sqlx::{PgPool, query};

pub async fn add_user(db: &PgPool, user_data: UserData) -> Result<i32, ApiError> {
    let exists = query!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)",
        user_data.email,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .exists
    .unwrap();

    if exists {
        return Err(ApiError::UserAlreadyExistsError);
    }

    let id_record = query!(
        "INSERT INTO users (email, password_hash, name, nickname, about_me, email_visible, language)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id",
        user_data.email,
        user_data.password_hash,
        user_data.name,
        user_data.nickname,
        user_data.about_me,
        user_data.email_visible,
        user_data.language,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(id_record.id)
}
