/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::{
    db_structs::users::{
        AccountType, AdminPasswordChangeRequestBody, AdminUpdateData, RegisterData, UserGetAllData,
        UserLanguage, UserListResponse,
    },
    tests::api_v1::{login, register, register_admin, setup},
};
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_all_users_non_admin(db: PgPool) {
    let server = setup(db);

    // Create two accounts and log in as one of them.
    register_email(&test_state, "1@gmail.com".to_owned()).await;
    register_email(&test_state, "2@gmail.com".to_owned()).await;
    let server = test_state.server;
    login(&server, "2@gmail.com".to_owned()).await;

    let response = server.get("/admin/all-user-data").await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_get_all_users_admin(db: PgPool) {
    let server = setup(db.clone());

    // Create one admin account and one author account and log in to the admin account.
    register_admin(&server, &db, "1@gmail.com".to_owned()).await;
    register(&server, "2@gmail.com".to_owned()).await;
    login(&server, "1@gmail.com".to_owned()).await;

    let response = server.get("/admin/all-user-data").await;
    // It should not fail because the user is an admin
    response.assert_status_ok();
    let user_list: UserListResponse = response.json();
    assert_eq!(user_list.total_users, 2);
}

#[sqlx::test]
async fn test_get_one_user_non_admin(db: PgPool) {
    let server = setup(db);

    // Create two accounts and log in as one of them.
    register(&server, "1@gmail.com".to_owned()).await;
    register(&server, "2@gmail.com".to_owned()).await;
    login(&server, "2@gmail.com".to_owned()).await;

    let response = server.get("/admin/users/1").await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_get_one_user_admin(db: PgPool) {
    let server = setup(db.clone());

    // Create one admin account and one author account and log in to the admin account.
    register_admin(&server, &db, "1@gmail.com".to_owned()).await;

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

    login(&server, "1@gmail.com".to_owned()).await;

    let response = server.get("/admin/users/2").await;
    // It should not fail because the user is an admin
    response.assert_status_ok();
    let user_data: UserGetAllData = response.json();
    let expected_user_data = UserGetAllData {
        email: register_data.email.clone(),
        name: register_data.name,
        enabled: true,
        nickname: register_data.nickname,
        about_me: register_data.about_me,
        email_visible: register_data.email_visible,
        account_type: AccountType::Author,
        language: register_data.language,
    };
    assert_eq!(user_data, expected_user_data);
}

#[sqlx::test]
async fn test_update_user(db: PgPool) {
    let test_state = setup(db);

    let update_data = AdminUpdateData {
        email: "JohnSmith@gmail.com".to_owned(),
        name: "John Smith".to_owned(),
        enabled: true,
        nickname: String::new(),
        about_me: String::new(),
        email_visible: false,
        account_type: AccountType::Admin,
        language: UserLanguage::Nl,
    };
    // Create two accounts and log in as one of them.
    register_email(&test_state, "1@gmail.com".to_owned()).await;
    register_email(&test_state, "2@gmail.com".to_owned()).await;
    let server = test_state.server;
    login(&server, "2@gmail.com".to_owned()).await;

    let response = server
        .post("/admin/users/1/update-data")
        .json(&update_data)
        .await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_admin_update_pending_user(db: PgPool) {
    let conn = get_connection_unwrap!(db);
    let test_state = setup(db);

    let admin_email = "admin@localhost".to_owned();
    register_admin(&test_state, conn, admin_email.clone()).await;

    let other_user_email = "johnsmith@localhost".to_string();
    let other_user = RegisterData {
        email: other_user_email.clone(),
        password: "a$$word".to_string(),
        name: "first".to_string(),
        nickname: "bob".to_string(),
        about_me: "I'm bob".to_string(),
        email_visible: false,
        language: UserLanguage::En,
    };
    let other_user_id = register(&test_state, &other_user).await;

    let server = test_state.server;
    let pending_user_email = "janedoe@gmail.com".to_owned();
    let response = server
        .post("/users/add")
        .json(&RegisterData {
            email: pending_user_email.clone(),
            password: "VeryStrongPassword1!".to_owned(),
            name: "second".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        })
        .await;
    response.assert_status_ok();

    login(&server, admin_email.clone()).await;
    // Test username already used
    let updated_user = AdminUpdateData {
        email: other_user_email.clone(),
        name: "second".to_string(),
        enabled: true,
        nickname: "bob".to_string(),
        about_me: "I'm still bob".to_string(),
        email_visible: false,
        account_type: AccountType::Admin,
        language: UserLanguage::En,
    };
    let response = server
        .post(&format!("/admin/users/{other_user_id}/update-data"))
        .json(&updated_user)
        .await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::UsernameAlreadyUsed);

    // Test email already used
    let updated_user = AdminUpdateData {
        email: pending_user_email.clone(),
        name: "first".to_string(),
        enabled: true,
        nickname: "bob".to_string(),
        about_me: "I'm still bob".to_string(),
        email_visible: false,
        account_type: AccountType::Admin,
        language: UserLanguage::En,
    };
    let response = server
        .post(&format!("/admin/users/{other_user_id}/update-data"))
        .json(&updated_user)
        .await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::EmailAlreadyUsed);
}

#[sqlx::test]
async fn test_change_password(db: PgPool) {
    let test_state = setup(db);

    let new_password = AdminPasswordChangeRequestBody {
        new_password: "Banaan123!".to_owned(),
    };
    // Create two accounts and log in as one of them.
    register_email(&test_state, "1@gmail.com".to_owned()).await;
    register_email(&test_state, "2@gmail.com".to_owned()).await;
    let server = test_state.server;
    login(&server, "2@gmail.com".to_owned()).await;

    let response = server
        .post("/admin/users/1/update-password")
        .json(&new_password)
        .await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}
