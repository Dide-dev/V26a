/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::users::UserResponse;
use crate::{
    db_structs::users::{
        AccountType, AdminPasswordChangeRequestBody, AdminUpdateData, RegisterData, UserData,
        UserGetAllData, UserLanguage, UserListResponse,
    },
    get_connection_unwrap,
    tests::api_v1::{login, register, register_admin, setup},
};
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_all_users_non_admin(db: PgPool) {
    let test_state = setup(db);

    // Create two accounts and log in as one of them.
    let account1 = RegisterData::test_values();
    let account2 = RegisterData::test_values();
    register(&test_state, &account1).await;
    register(&test_state, &account2).await;
    let server = test_state.server;
    login(&server, account2.email).await;

    let response = server.get("/admin/all-user-data").await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_get_all_users_admin(db: PgPool) {
    let test_state = setup(db.clone());

    // Create one admin account and one author account and log in to the admin account.
    let account1 = RegisterData::test_values();
    let account2 = RegisterData::test_values();
    register_admin(&test_state, &db, &account1).await;
    register(&test_state, &account2).await;
    let server = test_state.server;
    login(&server, account1.email).await;

    let response = server.get("/admin/all-user-data").await;
    // It should not fail because the user is an admin
    response.assert_status_ok();
    let user_list: UserListResponse = response.json();
    assert_eq!(user_list.total_users, 2);
}

#[sqlx::test]
async fn test_get_one_user_non_admin(db: PgPool) {
    let test_state = setup(db);

    // Create two accounts and log in as one of them.
    let account1 = RegisterData::test_values();
    let account2 = RegisterData::test_values();
    register(&test_state, &account1).await;
    register(&test_state, &account2).await;
    let server = test_state.server;
    login(&server, account2.email).await;

    let response = server.get("/admin/users/1").await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_get_one_user_admin(db: PgPool) {
    let test_state = setup(db.clone());

    // Create one admin account and one author account and log in to the admin account.
    let admin_account = RegisterData::test_values();
    register_admin(&test_state, &db, &admin_account).await;

    // Create an account
    let user_account = RegisterData::test_values();
    register(&test_state, &user_account).await;

    let server = test_state.server;
    login(&server, admin_account.email).await;

    let response = server.get("/admin/users/2").await;
    // It should not fail because the user is an admin
    response.assert_status_ok();
    let user_data: UserGetAllData = response.json();
    let expected_user_data = UserGetAllData {
        email: user_account.email.clone(),
        data: user_account.data,
        enabled: true,
        account_type: AccountType::Author,
    };
    assert_eq!(user_data, expected_user_data);
}

#[sqlx::test]
async fn test_update_user(db: PgPool) {
    let test_state = setup(db);

    let update_data = AdminUpdateData {
        email: "JohnSmith@gmail.com".to_owned(),
        data: UserData {
            name: "John Smith".to_owned(),
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: UserLanguage::Nl,
        },
        enabled: true,
        account_type: AccountType::Admin,
    };
    // Create two accounts and log in as one of them.
    let account1 = RegisterData::test_values();
    let account2 = RegisterData::test_values();
    register(&test_state, &account1).await;
    register(&test_state, &account2).await;
    let server = test_state.server;
    login(&server, account2.email).await;

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

    let admin_user = RegisterData::test_values();
    register_admin(&test_state, conn, &admin_user).await;

    let other_user = RegisterData::test_values();
    let other_user_id = register(&test_state, &other_user).await;

    let server = test_state.server;
    let pending_user = RegisterData::test_values();
    let response = server.post("/users/add").json(&pending_user).await;
    response.assert_status_ok();

    login(&server, admin_user.email).await;
    // Test username already used
    let updated_user = AdminUpdateData {
        email: other_user.email,
        enabled: true,
        account_type: AccountType::Admin,
        data: UserData {
            name: pending_user.data.name,
            nickname: "bob".to_string(),
            about_me: "I'm still bob".to_string(),
            email_visible: false,
            language: UserLanguage::En,
        },
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
        email: pending_user.email,
        enabled: true,
        account_type: AccountType::Admin,
        data: UserData {
            name: "first".to_string(),
            nickname: "bob".to_string(),
            about_me: "I'm still bob".to_string(),
            email_visible: false,
            language: UserLanguage::En,
        },
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
    let account1 = RegisterData::test_values();
    let account2 = RegisterData::test_values();
    register(&test_state, &account1).await;
    register(&test_state, &account2).await;
    let server = test_state.server;
    login(&server, account2.email).await;

    let response = server
        .post("/admin/users/1/update-password")
        .json(&new_password)
        .await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}
