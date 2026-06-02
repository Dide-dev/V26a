/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::pagination::PaginationOptions;
use crate::db_structs::users::{
    AccountType, AdminUpdateData, AllUsersGetData, PublicUserData, RegisterData, UserData,
    UserGeneralData, UserGetData, UserLanguage,
};
use crate::errors::ApiError;
use bcrypt::hash;
use sqlx::{PgConnection, PgExecutor, query, query_as, query_scalar};
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
pub async fn add_user(
    db: &mut PgConnection,
    user_data: &RegisterData,
    cost: u32,
) -> Result<i32, ApiError> {
    // Hash the password using bcrypt. Anything beyond 72 bytes will be truncated.
    // Put a limit on password length.
    let hashed_password = hash(&user_data.password, cost).map_err(|_| ApiError::HashException)?;

    // Insert user into database.
    let user_id = query!(
        "INSERT INTO users (email, password_hash, name, nickname, about_me, email_visible, language_code)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id",
        user_data.email.to_lowercase(),
        hashed_password,
        user_data.data.name,
        user_data.data.nickname,
        user_data.data.about_me,
        user_data.data.email_visible,
        user_data.data.language.to_string(),
    )
    .fetch_one(db)
    .await?
    .id;

    Ok(user_id)
}

/// Query the private data for the given user id in the given database.
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
        "SELECT email, name, nickname, about_me, email_visible, language_code FROM users WHERE id = $1",
        id,
    )
    .fetch_one(db)
    .await?;

    Ok(UserGetData {
        email: record.email,
        data: UserData {
            name: record.name,
            nickname: record.nickname,
            about_me: record.about_me,
            email_visible: record.email_visible,
            language: UserLanguage::from_str(record.language_code.as_str())?,
        },
    })
}

/// Query the public data for the given user id in the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `id` - The id of the user.
///
/// # Returns
/// A [`PublicUserData`] object containing the user's data, minus their password.
///
/// Or if no user by that id exists, returns [`None`]
///
/// # Errors
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user_public_data(
    db: impl PgExecutor<'_>,
    id: i32,
) -> Result<Option<PublicUserData>, ApiError> {
    let record = query!(
        "SELECT name, nickname, about_me, email_visible, email, language_code, account_type FROM users WHERE id = $1",
        id,
    )
    .fetch_optional(db)
    .await?;

    let Some(record) = record else {
        return Ok(None);
    };

    Ok(Some(PublicUserData {
        name: record.name,
        nickname: record.nickname,
        about_me: record.about_me,
        language: UserLanguage::from_str(record.language_code.as_str())?,
        account_type: AccountType::from_str(record.account_type.as_str())?,
        email: if record.email_visible {
            Some(record.email)
        } else {
            None
        },
    }))
}

/// Query the data for the given user id in the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `id` - The id of the user.
///
/// # Returns
/// A [`AdminUpdateData`] object containing the user's data, minus their password.
///
/// # Errors
/// Returns [`InvalidDataFound`](ApiError::InvalidDataFound) if the user's data in the database is invalid or corrupted.
///
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user_all_data(
    db: impl PgExecutor<'_>,
    id: i32,
) -> Result<AdminUpdateData, ApiError> {
    let record = query!(
        "SELECT email, name, enabled, nickname, about_me, account_type FROM users WHERE id = $1",
        id,
    )
    .fetch_one(db)
    .await?;

    Ok(AdminUpdateData {
        email: record.email,
        data: UserGeneralData {
            name: record.name,
            nickname: record.nickname,
            about_me: record.about_me,
        },
        enabled: record.enabled,
        account_type: AccountType::from_str(record.account_type.as_str())?,
    })
}

/// Updates a user in the database.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `new_data` - The new data to be assigned as [`UserData`], including the id of the user to be updated.
///
/// `id` - The id of the user that needs to be updated.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn update_user(
    db: impl PgExecutor<'_>,
    id: i32,
    new_data: &UserData,
) -> Result<(), ApiError> {
    query!(
        "UPDATE users SET name = $1, nickname = $2, about_me = $3, email_visible = $4, language_code = $5 WHERE id = $6",
        new_data.name,
        new_data.nickname,
        new_data.about_me,
        new_data.email_visible,
        new_data.language.to_string(),
        id,
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Updates a user in the database. Can also update protected fields.
///
/// # Arguments
/// `db` - A connection to a Postgres database.
///
/// `new_data` - The new data to be assigned as [`AdminUpdateData`], including the id of the user to be updated.
///
/// `id` - The id of the user that needs to be updated.
///
/// # Returns
/// Nothing.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn admin_update_user(
    db: impl PgExecutor<'_>,
    id: i32,
    new_data: &AdminUpdateData,
) -> Result<(), ApiError> {
    query!(
        "UPDATE users SET name = $1, nickname = $2, about_me = $3, account_type = $4, enabled = $5, email = $6 WHERE id = $7",
        new_data.data.name,
        new_data.data.nickname,
        new_data.data.about_me,
        new_data.account_type.to_string(),
        new_data.enabled,
        new_data.email.to_lowercase(),
        id,
    )
    .execute(db)
    .await?;

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
    cost: u32,
) -> Result<(), ApiError> {
    // Hash the password using bcrypt. Anything beyond 72 bytes will be truncated.
    // Put a limit on password length.
    let hashed_password = hash(new_password, cost).map_err(|_| ApiError::HashException)?;

    query!(
        "UPDATE users SET password_hash = $1 WHERE id = $2",
        hashed_password,
        user_id,
    )
    .execute(db)
    .await?;

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
            .await?;
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
    .await?;

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
pub async fn get_user_count(db: impl PgExecutor<'_>) -> Result<i64, ApiError> {
    let users_count: i64 = query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await?
        .ok_or(ApiError::DatabaseException)?;

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
pub async fn get_user_list_by_page(
    db: impl PgExecutor<'_>,
    page: &PaginationOptions,
) -> Result<Vec<AllUsersGetData>, ApiError> {
    let limit = i64::from(page.get_size());
    let offset = i64::from((page.get_page() - 1) * page.get_size());

    let users = query_as!(
        AllUsersGetData,
        "SELECT id, email, name, enabled FROM users
         ORDER BY id
         LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(db)
    .await?;

    if users.is_empty() {
        return Err(ApiError::InvalidPage);
    }

    Ok(users)
}

/// Checks if a user with a given e-mail exists in the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `email` - A reference to the email of the user being checked.
///
/// # Returns
/// The `user_id` corresponding to `email` if it is used, or `None` if it is not.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user_by_email(
    db: impl PgExecutor<'_>,
    email: &str,
) -> Result<Option<i32>, ApiError> {
    Ok(query!(
        "SELECT id FROM users WHERE email = $1",
        email.to_lowercase(),
    )
    .fetch_optional(db)
    .await?
    .map(|r| r.id))
}

/// Checks if a user with a given username exists in the given database.
///
/// # Arguments
/// `db` - Either a [`&PgPool`](sqlx::PgPool) or a [`&mut PgConnection`](PgConnection).
///
/// `name` - A reference to the username of the user being checked.
///
/// # Returns
/// The `user_id` corresponding to `name` if it is used, or `None` if it is not.
///
/// # Errors
/// May return a [`DatabaseException`](ApiError::DatabaseException) instead.
pub async fn get_user_by_name(
    db: impl PgExecutor<'_>,
    name: &str,
) -> Result<Option<i32>, ApiError> {
    Ok(query!(
        "SELECT id FROM users WHERE LOWER(name) = $1",
        name.to_lowercase(),
    )
    .fetch_optional(db)
    .await?
    .map(|r| r.id))
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
        .await?
        .exists
        .ok_or(ApiError::DatabaseException)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::auth::{get_user_credentials_by_email, get_user_credentials_by_id};
    use crate::db::test::{initialize_basic_user, initialize_user};
    use crate::db_structs::users::{AccountType, RegisterData, UserData, UserLanguage};
    use crate::get_connection_unwrap;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_add_user(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            data: UserData {
                name: "name".to_owned(),
                nickname: "nickname".to_owned(),
                about_me: "about_me".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
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
            data: UserData {
                name: all_found_user_data.name,
                nickname: all_found_user_data.nickname,
                about_me: all_found_user_data.about_me,
                email_visible: all_found_user_data.email_visible,
                language: all_found_user_data.language_code.parse().unwrap(),
            },
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
    async fn test_get_user_by_email(db: PgPool) {
        let email = "email".to_owned();

        let register_data = RegisterData {
            email: email.clone(),
            password: "password".to_owned(),
            data: UserData {
                name: "name".to_owned(),
                nickname: String::new(),
                about_me: String::new(),
                email_visible: false,
                language: UserLanguage::Nl,
            },
        };

        // Assert user does not exist before adding to the database.
        let found_user_id = get_user_by_email(&db, &email).await.unwrap();
        assert!(found_user_id.is_none());

        let user_id = initialize_user(&db, &register_data).await;

        // Assert user does exist after adding to the database.
        let found_user_id = get_user_by_email(&db, &email).await.unwrap();
        assert_eq!(found_user_id, Some(user_id));
    }

    #[sqlx::test]
    async fn test_admin_update_user(db: PgPool) {
        let register_data = RegisterData {
            email: "hoi@gmail.com".to_owned(),
            password: "password".to_owned(),
            data: UserData {
                name: "Not_John_Smith".to_owned(),
                nickname: String::new(),
                about_me: String::new(),
                email_visible: false,
                language: UserLanguage::Nl,
            },
        };

        let update_data = AdminUpdateData {
            email: "JohnSmith@gmail.com".to_owned(),
            enabled: true,
            data: UserGeneralData {
                name: "John_Smith".to_owned(),
                nickname: String::new(),
                about_me: String::new(),
            },
            account_type: AccountType::Admin,
        };

        let user_id = initialize_user(&db, &register_data).await;

        admin_update_user(&db, user_id, &update_data).await.unwrap();

        let found_user_id = get_user_by_name(&db, &update_data.data.name).await.unwrap();
        // Checks whether the users name changed from "Not John Smith" to "John Smith" by checking of the id's are the same.
        assert_eq!(found_user_id, Some(user_id));
    }

    #[sqlx::test]
    async fn test_get_user_by_name(db: PgPool) {
        let name = "name".to_owned();

        let register_data = RegisterData {
            email: "hoi@gmail.com".to_owned(),
            password: "password".to_owned(),
            data: UserData {
                name: name.clone(),
                nickname: String::new(),
                about_me: String::new(),
                email_visible: false,
                language: UserLanguage::Nl,
            },
        };

        // Assert user does not exist before adding to the database.
        let found_user_id = get_user_by_name(&db, &name).await.unwrap();
        assert!(found_user_id.is_none());

        let user_id = initialize_user(&db, &register_data).await;

        // Assert user does exist after adding to the database.
        let found_user_id = get_user_by_name(&db, &name).await.unwrap();
        assert_eq!(found_user_id, Some(user_id));
    }

    #[sqlx::test]
    async fn test_user_id_exists(db: PgPool) {
        // Assert user does not exist before adding to the database.
        let does_user_exist = user_id_exists(&db, 420).await.unwrap();
        assert!(!does_user_exist);

        let register_data = RegisterData {
            email: "test@test.com".to_owned(),
            password: "password".to_owned(),
            data: UserData {
                name: "name".to_owned(),
                nickname: String::new(),
                about_me: String::new(),
                email_visible: false,
                language: UserLanguage::Nl,
            },
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
            data: UserData {
                name: "name".to_owned(),
                nickname: "nickname".to_owned(),
                about_me: "about_me".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
        };

        let user_id = initialize_user(&db, &register_data).await;
        let user_data = get_user(&db, user_id).await.unwrap();

        let expected_user_data = UserGetData {
            email: register_data.email,
            data: UserData {
                name: register_data.data.name,
                nickname: register_data.data.nickname,
                about_me: register_data.data.about_me,
                email_visible: register_data.data.email_visible,
                language: register_data.data.language,
            },
        };

        assert_eq!(user_data, expected_user_data);
    }

    #[sqlx::test]
    async fn test_get_user_all_data(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            data: UserData {
                name: "name".to_owned(),
                nickname: "nickname".to_owned(),
                about_me: "about_me".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
        };

        let user_id = initialize_user(&db, &register_data).await;
        let user_data = get_user_all_data(&db, user_id).await.unwrap();

        let expected_user_data = AdminUpdateData {
            email: register_data.email,
            data: UserGeneralData {
                name: register_data.data.name,
                nickname: register_data.data.nickname,
                about_me: register_data.data.about_me,
            },
            enabled: true,
            account_type: AccountType::Author,
        };

        assert_eq!(user_data, expected_user_data);
    }

    #[sqlx::test]
    async fn test_get_all_users(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            data: UserData {
                name: "name".to_owned(),
                nickname: "nickname".to_owned(),
                about_me: "about_me".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
        };
        let register_data2 = RegisterData {
            email: "email2".to_owned(),
            password: "password2".to_owned(),
            data: UserData {
                name: "name2".to_owned(),
                nickname: "nickname2".to_owned(),
                about_me: "about_me2".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
        };
        let pagination_options = PaginationOptions::new(1, 5, 1000, 500).unwrap();

        initialize_user(&db, &register_data).await;
        initialize_user(&db, &register_data2).await;
        let all_user_data = get_user_list_by_page(&db, &pagination_options)
            .await
            .unwrap();

        let expected_user_data = vec![
            AllUsersGetData {
                id: 1,
                email: register_data.email,
                name: register_data.data.name,
                enabled: true,
            },
            AllUsersGetData {
                id: 2,
                email: register_data2.email,
                name: register_data2.data.name,
                enabled: true,
            },
        ];

        assert_eq!(all_user_data, expected_user_data);
    }

    #[sqlx::test]
    async fn test_get_user_count(db: PgPool) {
        assert_eq!(get_user_count(&db).await.unwrap(), 0);
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            data: UserData {
                name: "name".to_owned(),
                nickname: "nickname".to_owned(),
                about_me: "about_me".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
        };
        let register_data2 = RegisterData {
            email: "email2".to_owned(),
            password: "password2".to_owned(),
            data: UserData {
                name: "name2".to_owned(),
                nickname: "nickname2".to_owned(),
                about_me: "about_me2".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
        };
        initialize_user(&db, &register_data).await;

        assert_eq!(get_user_count(&db).await.unwrap(), 1);
        initialize_user(&db, &register_data2).await;
        assert_eq!(get_user_count(&db).await.unwrap(), 2);
    }

    #[sqlx::test]
    async fn test_update_user(db: PgPool) {
        let user_id = initialize_basic_user(&db).await;

        // Test not updating the account type and the enabled flag
        let new_user_data = UserData {
            name: "name".to_owned(),
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: UserLanguage::Nl,
        };
        update_user(&db, user_id, &new_user_data).await.unwrap();
        let all_found_user_data = query!("SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();
        let found_update_data = UserData {
            name: all_found_user_data.name,
            nickname: all_found_user_data.nickname,
            about_me: all_found_user_data.about_me,
            email_visible: all_found_user_data.email_visible,
            language: all_found_user_data.language_code.parse().unwrap(),
        };
        assert_eq!(found_update_data, new_user_data);

        // Test updating the account type and enabled flag
        let new_user_data = UserData {
            name: "john smith".to_owned(),
            nickname: "john".to_owned(),
            about_me: "Nothing interesting".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        };
        update_user(&db, user_id, &new_user_data).await.unwrap();
        let all_found_user_data = query!("SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();
        let found_update_data = UserData {
            name: all_found_user_data.name,
            nickname: all_found_user_data.nickname,
            about_me: all_found_user_data.about_me,
            email_visible: all_found_user_data.email_visible,
            language: all_found_user_data.language_code.parse().unwrap(),
        };
        assert_eq!(found_update_data, new_user_data);
    }

    #[sqlx::test]
    async fn test_get_user_account_type(db: PgPool) {
        let email = "test123@gmail.com".to_owned();

        let register_data = RegisterData {
            email: email.clone(),
            password: "password".to_string(),
            data: UserData {
                name: "name".to_string(),
                nickname: String::new(),
                about_me: String::new(),
                email_visible: false,
                language: UserLanguage::En,
            },
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
            data: UserData {
                name: "name".to_owned(),
                nickname: "nickname".to_owned(),
                about_me: "about_me".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
        };

        let user_id = initialize_user(&db, &register_data).await;
        let new_password = "P@ssword";
        let cost = 4;

        update_password(&db, user_id, new_password, cost)
            .await
            .unwrap();

        let password_in_db = get_user_credentials_by_id(&db, user_id)
            .await
            .unwrap()
            .unwrap()
            .password_hash;

        assert!(password_in_db.starts_with(&format!("$2b${cost:02}$")));

        let password_valid = bcrypt::verify(new_password, password_in_db.as_str()).unwrap();

        assert!(password_valid);

        let new_password = "P@ssword!!!!";
        let cost = 10;

        update_password(&db, user_id, new_password, cost)
            .await
            .unwrap();

        let password_in_db = get_user_credentials_by_id(&db, user_id)
            .await
            .unwrap()
            .unwrap()
            .password_hash;

        assert!(password_in_db.starts_with(&format!("$2b${cost:02}$")));

        let password_valid = bcrypt::verify(new_password, password_in_db.as_str()).unwrap();

        assert!(password_valid);
    }

    #[sqlx::test]
    async fn test_delete_user(db: PgPool) {
        let register_data = RegisterData {
            email: "email".to_owned(),
            password: "password".to_owned(),
            data: UserData {
                name: "name".to_owned(),
                nickname: "nickname".to_owned(),
                about_me: "about_me".to_owned(),
                email_visible: true,
                language: UserLanguage::En,
            },
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
            data: UserData {
                name: all_found_user_data.name,
                nickname: all_found_user_data.nickname,
                about_me: all_found_user_data.about_me,
                email_visible: all_found_user_data.email_visible,
                language: all_found_user_data.language_code.parse().unwrap(),
            },
        };

        let expected_user_data = RegisterData {
            email: ("deleted").to_string(),
            password: String::new(),
            data: UserData {
                name: format!("deleted user_{user_id}"),
                nickname: String::new(),
                about_me: String::new(),
                email_visible: false,
                language: register_data.data.language,
            },
        };

        assert!(!all_found_user_data.enabled);
        assert_eq!(found_update_data, expected_user_data);
    }
}
