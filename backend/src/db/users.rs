/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

extern crate bcrypt;

use crate::db_structs::users::{RegisterData, UserData, UserLanguage};
use crate::errors::ApiError;

use bcrypt::{DEFAULT_COST, hash};
use sqlx::{PgConnection, PgExecutor, query};
use std::str::FromStr;

/// Add a new user to the database, returning its assigned id.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `user_data` - The data assigned to the new user.
///
/// # Returns
/// The id assigned to the new user.
///
/// # Error
/// Returns [`EmailAlreadyUsed`](ApiError::EmailAlreadyUsed) if the email was already used in the database.
///
/// May return a [`HashException`](ApiError::HashException) instead.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_user(db: &mut PgConnection, user_data: &RegisterData) -> Result<i32, ApiError> {
    // Check if email already exists. This check is not strictly necessary, as a `UNIQUE`
    // constraint is placed on the `email` field.
    // In case of two concurrent requests with same e-mail, a `ApiError::DatabaseException`
    // will be returned instead of an `ApiError::EmailAlreadyUsed`.
    let exists = user_exists(&mut *db, &user_data.email).await?;

    if exists {
        return Err(ApiError::EmailAlreadyUsed);
    }

    // Hash the password using bcrypt. Anything beyond 72 bytes will be truncated.
    // Put a limit on password length.
    let hashed_password =
        hash(&user_data.password, DEFAULT_COST).map_err(|_| ApiError::HashException)?;

    // Insert user into database.
    let user_id = query!(
        "INSERT INTO users (email, password_hash, name, nickname, about_me, email_visible, language)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id",
        user_data.email,
        hashed_password,
        user_data.name,
        user_data.nickname,
        user_data.about_me,
        user_data.email_visible,
        user_data.language.to_string(),
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .id;

    Ok(user_id)
}

/// Get a user from the database, return its assigned id and account type.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
///
/// `user_id` - The id of the user.
///
/// # Returns
/// The type assigned to the user.
///
/// # Error
/// Returns [`WrongCredentials`](ApiError::WrongCredentials) if credentials are wrong.
///
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user_account_type(
    db: impl PgExecutor<'_>,
    userid: i32,
) -> Result<String, ApiError> {
    // Get account type
    let record = query!("SELECT account_type FROM users WHERE id = $1", userid,)
        .fetch_optional(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?
        .ok_or(ApiError::WrongCredentials)?;

    Ok(record.account_type)
}

/// Checks if a user with a given e-mail exists in the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `email` - A reference to the email of the user being checked.
///
/// # Returns
/// `true` if the user exists or `false` if they do not.
///
/// # Error
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn user_exists(db: impl PgExecutor<'_>, email: &String) -> Result<bool, ApiError> {
    query!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)",
        email,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?
    .exists
    .ok_or(ApiError::DatabaseException)
}

/// Query the data for the given user id in the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `id` - The id of the user.
///
/// # Returns
/// A [`UserData`] object containing the user's data, minus their password.
///
/// # Error
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user(db: impl PgExecutor<'_>, id: i32) -> Result<UserData, ApiError> {
    let record = query!(
        "SELECT email, name, nickname, about_me, email_visible, language FROM users WHERE id = $1",
        id,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(UserData {
        email: record.email,
        name: record.name,
        nickname: record.nickname,
        about_me: record.about_me,
        email_visible: record.email_visible,
        language: UserLanguage::from_str(record.language.as_str())?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test::initialize_user;
    use crate::db_structs::users::{RegisterData, UserLanguage};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_add_user(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: Some("nickname".to_owned()),
            about_me: Some("about_me".to_owned()),
            email_visible: true,
            language: UserLanguage::En,
        };

        let user_id = initialize_user(&db, &register_data).await;

        let all_found_user_data = query!("SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_register_data = RegisterData {
            email: all_found_user_data.email,
            // We test the password separately, as it has been hashed.
            password: register_data.password.clone(),
            name: all_found_user_data.name,
            nickname: all_found_user_data.nickname,
            about_me: all_found_user_data.about_me,
            email_visible: all_found_user_data.email_visible,
            language: all_found_user_data.language.parse().unwrap(),
        };
        assert_eq!(found_register_data, register_data);

        let password_matches = bcrypt::verify(
            register_data.password,
            all_found_user_data.password_hash.as_str(),
        )
        .unwrap();

        assert!(password_matches);
    }

    #[sqlx::test]
    async fn test_user_exists(db: PgPool) {
        let email = "email".to_owned();

        let register_data = RegisterData {
            email: email.clone(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: None,
            about_me: None,
            email_visible: false,
            language: UserLanguage::Nl,
        };

        // Assert user does not exist before adding to the database.
        let does_user_exist = user_exists(&db, &email).await.unwrap();
        assert!(!does_user_exist);

        initialize_user(&db, &register_data).await;

        // Assert user does exist after adding to the database.
        let does_user_exist = user_exists(&db, &email).await.unwrap();
        assert!(does_user_exist);
    }

     #[sqlx::test]
    async fn test_get_user(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: Some("nickname".to_owned()),
            about_me: Some("about_me".to_owned()),
            email_visible: true,
            language: UserLanguage::En,
        };

        let user_id = initialize_user(&db, &register_data).await;
        let user_data = get_user(&db, user_id).await.unwrap();

        let expected_user_data = UserData {
            email: register_data.email,
            name: register_data.name,
            nickname: register_data.nickname,
            about_me: register_data.about_me,
            email_visible: register_data.email_visible,
            language: register_data.language,
        };

        assert_eq!(user_data, expected_user_data);
    }

    #[sqlx::test]
    async fn test_get_user_account_type(db: PgPool) {
        let register_data = RegisterData {
            email: "test123@gmail.com".to_string(),
            password: "password".to_string(),
            name: "name".to_string(),
            nickname: None,
            about_me: None,
            email_visible: false,
            language: UserLanguage::En,
        };

        // Add an user
        let mut conn = db.acquire().await.unwrap();
        let user_id = add_user(&mut conn, &register_data).await.unwrap();

        // Assert user does have a (default) account type.
        let account_type = get_user_account_type(&db, user_id).await.unwrap();
        assert_eq!(account_type, "author");
    }

    #[sqlx::test]
    async fn test_get_user_account_type_not_found(db: PgPool) {
        let result = get_user_account_type(&db, 999).await;
        assert!(matches!(result, Err(ApiError::WrongCredentials)));
    }
}
