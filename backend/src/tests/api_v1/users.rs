/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::{
    AccountType, RegisterData, UserGetData, UserLanguage, UserUpdateData,
};
use crate::tests::api_v1::{login, register, setup};
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_and_update_own_user_data(db: PgPool) {
    let server = setup(db);

    // Create an account
    let register_data = RegisterData {
        email: "johnsmith@gmail.com".to_owned(),
        password: "VeryStrongPassword1!".to_owned(),
        name: "name".to_owned(),
        nickname: "nickname".to_owned(),
        about_me: "about_me".to_owned(),
        email_visible: true,
        language: UserLanguage::En,
    };

    let response = server.post("/users/add").json(&register_data).await;
    response.assert_status_ok();
    let user_id: i32 = response.json();

    let expected_user_data = UserGetData {
        email: register_data.email.clone(),
        name: register_data.name,
        nickname: register_data.nickname,
        about_me: register_data.about_me,
        email_visible: register_data.email_visible,
        language: register_data.language,
    };

    // Log in
    let response = server
        .post("/auth/login")
        .json(&LoginRequestBody {
            email: register_data.email.clone(),
            password: register_data.password,
        })
        .await;
    response.assert_status_ok();

    // Get our own user data
    let response = server.get("/users/me").await;
    response.assert_status_ok();

    let user_data: UserGetData = response.json();
    assert_eq!(user_data, expected_user_data);

    // Update our own user data
    let update_data = UserUpdateData {
        name: "John Smith".to_owned(),
        nickname: "John".to_owned(),
        about_me: String::new(),
        email_visible: false,
        language: UserLanguage::Nl,
        account_type: None,
        enabled: None,
    };

    let response = server
        .post(&format!("/users/update/{user_id}"))
        .json(&update_data)
        .await;
    response.assert_status_ok();

    let expected_user_data = UserGetData {
        email: register_data.email.clone(),
        name: update_data.name,
        nickname: update_data.nickname,
        about_me: update_data.about_me,
        email_visible: update_data.email_visible,
        language: update_data.language,
    };

    // Get our own user data again
    let response = server.get("/users/me").await;
    response.assert_status_ok();

    let user_data: UserGetData = response.json();
    assert_eq!(user_data, expected_user_data);
}

#[sqlx::test]
async fn test_update_other_user(db: PgPool) {
    let server = setup(db);

    // Create two accounts and log in as one of them.
    let user_id_1 = register(&server, "1@gmail.com".to_owned()).await;
    register(&server, "2@gmail.com".to_owned()).await;
    login(&server, "2@gmail.com".to_owned()).await;

    // Update the other user's data.
    let update_data = UserUpdateData {
        name: "John Smith".to_owned(),
        nickname: String::new(),
        about_me: String::new(),
        email_visible: false,
        language: UserLanguage::Nl,
        account_type: None,
        enabled: None,
    };

    let response = server
        .post(&format!("/users/update/{user_id_1}"))
        .json(&update_data)
        .await;
    // Changing the other user's data should fail.
    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_change_account_type(db: PgPool) {
    let server = setup(db);
    let user_id = register(&server, "test@gmail.com".to_owned()).await;
    login(&server, "test@gmail.com".to_owned()).await;

    // Update our own user data
    let update_data = UserUpdateData {
        name: "John Smith".to_owned(),
        nickname: String::new(),
        about_me: String::new(),
        email_visible: false,
        language: UserLanguage::Nl,
        account_type: Some(AccountType::Admin),
        enabled: None,
    };

    let response = server
        .post(&format!("/users/update/{user_id}"))
        .json(&update_data)
        .await;
    // Making ourselves admin should fail.
    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_change_enabled_flag(db: PgPool) {
    let server = setup(db);
    let user_id = register(&server, "test@gmail.com".to_owned()).await;
    login(&server, "test@gmail.com".to_owned()).await;

    // Update our own user data
    let update_data = UserUpdateData {
        name: "John Smith".to_owned(),
        nickname: String::new(),
        about_me: String::new(),
        email_visible: false,
        language: UserLanguage::Nl,
        account_type: None,
        enabled: Some(false),
    };

    let response = server
        .post(&format!("/users/update/{user_id}"))
        .json(&update_data)
        .await;
    // Disabling our own account should fail.
    response.assert_status_forbidden();
}
