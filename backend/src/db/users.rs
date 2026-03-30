/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::users::UserData;
use crate::errors::ApiError;
use bcrypt::{DEFAULT_COST, hash};
use sqlx::{PgConnection, PgExecutor, query};

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
pub async fn add_user(db: &mut PgConnection, user_data: &UserData) -> Result<i32, ApiError> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::test::initialize_user;
    use crate::db_structs::users::{UserData, UserLanguage};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_add_user(db: PgPool) {
        let user_data = UserData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: Some("nickname".to_owned()),
            about_me: Some("about_me".to_owned()),
            email_visible: true,
            language: UserLanguage::En,
        };

        let user_id = initialize_user(&db, &user_data).await;

        let all_found_user_data = query!("SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_user_data = UserData {
            email: all_found_user_data.email,
            // We test the password separately, as it has been hashed.
            password: user_data.password.clone(),
            name: all_found_user_data.name,
            nickname: all_found_user_data.nickname,
            about_me: all_found_user_data.about_me,
            email_visible: all_found_user_data.email_visible,
            language: all_found_user_data.language.parse().unwrap(),
        };
        assert_eq!(found_user_data, user_data);

        let password_matches = bcrypt::verify(
            user_data.password,
            all_found_user_data.password_hash.as_str(),
        )
        .unwrap();

        assert!(password_matches);
    }

    #[sqlx::test]
    async fn test_user_exists(db: PgPool) {
        let email = "email".to_owned();

        let user_data = UserData {
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

        initialize_user(&db, &user_data).await;

        // Assert user does exist after adding to the database.
        let does_user_exist = user_exists(&db, &email).await.unwrap();
        assert!(does_user_exist);
    }
}
