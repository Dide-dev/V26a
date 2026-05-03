/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::users::{
    RegisterData, UserGetAllData, UserGetData, UserLanguage, UserUpdateData,
};
use crate::db_structs::pagination::PaginationOptions;
use crate::errors::ApiError;

use bcrypt::{DEFAULT_COST, hash};
use sqlx::{PgConnection, QueryBuilder, Postgres, PgExecutor, query, query_as};
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
/// # Errors
/// May return a [`HashException`](ApiError::HashException) instead.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn add_user(db: &mut PgConnection, user_data: &RegisterData) -> Result<i32, ApiError> {
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
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn user_email_exists(db: impl PgExecutor<'_>, email: &str) -> Result<bool, ApiError> {
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

/// Checks if a user with a given id exists in the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `id` - The id of the user being checked.
///
/// # Returns
/// `true` if the user exists or `false` if they do not.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn user_id_exists(db: impl PgExecutor<'_>, id: i32) -> Result<bool, ApiError> {
    query!("SELECT EXISTS (SELECT 1 FROM users WHERE id = $1)", id,)
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
/// A [`UserGetData`] object containing the user's data, minus their password.
///
/// # Errors
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user(db: impl PgExecutor<'_>, id: i32) -> Result<UserGetData, ApiError> {
    let record = query!(
        "SELECT email, name, nickname, about_me, email_visible, language FROM users WHERE id = $1",
        id,
    )
    .fetch_one(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(UserGetData {
        email: record.email,
        name: record.name,
        nickname: record.nickname,
        about_me: record.about_me,
        email_visible: record.email_visible,
        language: UserLanguage::from_str(record.language.as_str())?,
    })
}

/// Updates a user in the database.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `new_data` - The new data to be assigned as [`UserUpdateData`], including the id of the user to be updated.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_user(
    db: impl PgExecutor<'_>,
    id: i32,
    new_data: &UserUpdateData,
) -> Result<(), ApiError> {
    // COALESCE is used here to update account_type and enabled only if they are not None.
    query!(
        "UPDATE users SET name = $1, nickname = $2, about_me = $3, email_visible = $4, language = $5, account_type = COALESCE($6, account_type), enabled = COALESCE($7, enabled) WHERE id = $8",
        new_data.name,
        new_data.nickname,
        new_data.about_me,
        new_data.email_visible,
        new_data.language.to_string(),
        new_data.account_type.map(|t| t.to_string()),
        new_data.enabled,
        id,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// Updates a user's password in the database.
///
/// # Arguments
/// `db` - either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `user_id` - The user whose password should be updated.
///
/// `new_password`- The new password.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`HashException`](ApiError::HashException) or a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_password(
    db: impl PgExecutor<'_>,
    user_id: i32,
    new_password: &str,
) -> Result<(), ApiError> {
    // Hash the password using bcrypt. Anything beyond 72 bytes will be truncated.
    // Put a limit on password length.
    let hashed_password = hash(new_password, DEFAULT_COST).map_err(|_| ApiError::HashException)?;

    query!(
        "UPDATE users SET password_hash = $1 WHERE id = $2",
        hashed_password,
        user_id,
    )
    .execute(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// Delete a user from the database.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `user_id` - The id of the user.
///
/// `delete_owned_trees` - Whether the user wants to also delete their trees.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn delete_user(
    db: &mut PgConnection,
    user_id: i32,
    delete_owned_trees: bool,
) -> Result<(), ApiError> {
    if delete_owned_trees {
        query!("DELETE FROM trees WHERE owner_id = $1", user_id,)
            .execute(&mut *db)
            .await
            .map_err(|_| ApiError::DatabaseException)?;
    }

    // Make all fields of the user empty (soft delete) and disable the account making logging in impossible.
    query!(
        r#"
        UPDATE users
        SET enabled = false,
            email = 'deleted',
            name = CONCAT('deleted user_', id),
            nickname = '',
            about_me = '',
            email_visible = false,
            password_hash = ''
        WHERE id = $1
        "#,
        user_id,
    )
    .execute(&mut *db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    Ok(())
}

/// Get the amount of users in the databse.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// # Returns
/// The amount of users in the database,
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_count_users_pagination(
    db: impl PgExecutor<'_>,
) -> Result<i64, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM users");

    let users_count: i64 = query
        .build_query_scalar()
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::DatabaseException)?;

    Ok(users_count)
}

/// Get the list of all users at a given page.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](sqlx::PgConnection).
///
/// `page` - Which page is requested.
///
/// # Returns
/// A list with the users at the requested page,
///
/// # Errors
/// Returns [`InvalidPage`](ApiError::InvalidPage) if the requested page is empty.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_queried_list_users_pagination(
    db: impl PgExecutor<'_>,
    page: &PaginationOptions,
) -> Result<Vec<UserGetAllData>, ApiError> {
    let limit = i64::from(page.get_size());
    let offset = i64::from((page.get_page() - 1) * page.get_size());

    let users = query_as!(
        UserGetAllData,
        "SELECT id, email, name, enabled FROM users
         ORDER BY id
         LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(|_| ApiError::DatabaseException)?;

    if users.is_empty() {
        return Err(ApiError::InvalidPage);
    }

    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::auth::{get_user_credentials_by_email, get_user_credentials_by_id};
    use crate::db::test::{initialize_basic_user, initialize_user};
    use crate::db_structs::users::{AccountType, RegisterData, UserLanguage, UserUpdateData};
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_add_user(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
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
    async fn test_user_email_exists(db: PgPool) {
        let email = "email".to_owned();

        let register_data = RegisterData {
            email: email.clone(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: UserLanguage::Nl,
        };

        // Assert user does not exist before adding to the database.
        let does_user_exist = user_email_exists(&db, &email).await.unwrap();
        assert!(!does_user_exist);

        initialize_user(&db, &register_data).await;

        // Assert user does exist after adding to the database.
        let does_user_exist = user_email_exists(&db, &email).await.unwrap();
        assert!(does_user_exist);
    }

    #[sqlx::test]
    async fn test_user_id_exists(db: PgPool) {
        // Assert user does not exist before adding to the database.
        let does_user_exist = user_id_exists(&db, 420).await.unwrap();
        assert!(!does_user_exist);

        let register_data = RegisterData {
            email: "test@test.com".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: UserLanguage::Nl,
        };
        let user_id = initialize_user(&db, &register_data).await;

        // Assert user does exist after adding to the database.
        let does_user_exist = user_id_exists(&db, user_id).await.unwrap();
        assert!(does_user_exist);
    }

    #[sqlx::test]
    async fn test_get_user(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        };

        let user_id = initialize_user(&db, &register_data).await;
        let user_data = get_user(&db, user_id).await.unwrap();

        let expected_user_data = UserGetData {
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
    async fn test_get_all_users(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        };
        let register_data2 = RegisterData {
            email: "email2".to_owned(),
            password: "password2".to_owned(),
            name: "name2".to_owned(),
            nickname: "nickname2".to_owned(),
            about_me: "about_me2".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        };
        let pagination_options= PaginationOptions::new(1, 5, 1000, 500).unwrap();

        initialize_user(&db, &register_data).await;
        initialize_user(&db, &register_data2).await;
        let all_user_data = get_queried_list_users_pagination(&db, &pagination_options).await.unwrap();

        let expected_user_data = vec![
            UserGetAllData {
                id: 1,
                email: register_data.email,
                name: register_data.name,
                enabled: true,
            },
            UserGetAllData {
                id: 2,
                email: register_data2.email,
                name: register_data2.name,
                enabled: true,
            },
        ];

        assert_eq!(all_user_data, expected_user_data);
    }

    #[sqlx::test]
    async fn test_update_user(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

        // Test not updating the account type and the enabled flag
        let new_user_data = UserUpdateData {
            name: "name".to_owned(),
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: UserLanguage::Nl,
            account_type: None,
            enabled: None,
        };
        update_user(&db, user_id, &new_user_data).await.unwrap();
        let all_found_user_data = query!("SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();
        let found_update_data = UserUpdateData {
            name: all_found_user_data.name,
            nickname: all_found_user_data.nickname,
            about_me: all_found_user_data.about_me,
            email_visible: all_found_user_data.email_visible,
            language: all_found_user_data.language.parse().unwrap(),
            // These need to be checked separately as they are both None in new_user_data.
            account_type: new_user_data.account_type,
            enabled: new_user_data.enabled,
        };
        assert_eq!(found_update_data, new_user_data);
        assert_eq!(
            all_found_user_data.account_type,
            AccountType::Author.to_string()
        );
        assert!(all_found_user_data.enabled);

        // Test updating the account type and enabled flag
        let new_user_data = UserUpdateData {
            name: "john smith".to_owned(),
            nickname: "john".to_owned(),
            about_me: "Nothing interesting".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
            account_type: Some(AccountType::Admin),
            enabled: Some(false),
        };
        update_user(&db, user_id, &new_user_data).await.unwrap();
        let all_found_user_data = query!("SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();
        let found_update_data = UserUpdateData {
            name: all_found_user_data.name,
            nickname: all_found_user_data.nickname,
            about_me: all_found_user_data.about_me,
            email_visible: all_found_user_data.email_visible,
            language: all_found_user_data.language.parse().unwrap(),
            account_type: Some(AccountType::from_str(&all_found_user_data.account_type).unwrap()),
            enabled: Some(all_found_user_data.enabled),
        };
        assert_eq!(found_update_data, new_user_data);
    }

    #[sqlx::test]
    async fn test_get_user_account_type(db: PgPool) {
        let email = "test123@gmail.com".to_owned();

        let register_data = RegisterData {
            email: email.clone(),
            password: "password".to_string(),
            name: "name".to_string(),
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: UserLanguage::En,
        };

        initialize_user(get_connection_unwrap!(db), &register_data).await;

        // Assert user has the default account type.
        let account_type = get_user_credentials_by_email(&db, email)
            .await
            .unwrap()
            .unwrap()
            .account_type;
        assert_eq!(account_type, AccountType::Author);
    }

    #[sqlx::test]
    async fn test_get_user_account_type_not_found(db: PgPool) {
        let result = get_user_credentials_by_email(&db, "abc@localhost".to_owned()).await;
        assert!(result.unwrap().is_none());
    }

    #[sqlx::test]
    async fn test_update_password(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        };

        let user_id = initialize_user(&db, &register_data).await;
        let new_password = "P@ssword";

        update_password(&db, user_id, new_password).await.unwrap();

        let password_in_db = get_user_credentials_by_id(&db, user_id)
            .await
            .unwrap()
            .unwrap()
            .password_hash;

        let password_valid = bcrypt::verify(new_password, password_in_db.as_str())
            .map_err(|_| ApiError::HashException)
            .unwrap();

        assert!(password_valid);
    }

    #[sqlx::test]
    async fn test_delete_user(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        };

        let user_id = initialize_user(&db, &register_data).await;

        delete_user(&mut db.acquire().await.unwrap(), user_id, true)
            .await
            .unwrap();

        let all_found_user_data = query!("SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();

        let found_update_data = RegisterData {
            email: all_found_user_data.email,
            password: all_found_user_data.password_hash,
            name: all_found_user_data.name,
            nickname: all_found_user_data.nickname,
            about_me: all_found_user_data.about_me,
            email_visible: all_found_user_data.email_visible,
            language: all_found_user_data.language.parse().unwrap(),
        };

        let expected_user_data = RegisterData {
            email: ("deleted").to_string(),
            password: String::new(),
            name: format!("deleted user_{user_id}"),
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: register_data.language,
        };

        assert!(!all_found_user_data.enabled);
        assert_eq!(found_update_data, expected_user_data);
    }
}
