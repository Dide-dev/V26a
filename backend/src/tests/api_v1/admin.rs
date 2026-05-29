/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::{
    db_structs::users::{
        AccountType, AdminPasswordChangeRequestBody, AdminUpdateData, RegisterData, UserData,
        UserGeneralData, UserLanguage, UserListResponse,
    },
    tests::api_v1::{login, register, register_admin, setup},
};
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_all_users_non_admin(db: PgPool) {
    let server = setup(db);

    // Create two accounts and log in as one of them.
    register(&server, "1@gmail.com".to_owned()).await;
    register(&server, "2@gmail.com".to_owned()).await;
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
        data: UserData {
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        },
    };

    let response = server.post("/users/add").json(&register_data).await;
    response.assert_status_ok();

    login(&server, "1@gmail.com".to_owned()).await;

    let response = server.get("/admin/users/2").await;
    // It should not fail because the user is an admin
    response.assert_status_ok();
    let user_data: AdminUpdateData = response.json();
    let expected_user_data = AdminUpdateData {
        email: register_data.email.clone(),
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
async fn test_update_user(db: PgPool) {
    let server = setup(db);

    let update_data = AdminUpdateData {
        email: "JohnSmith@gmail.com".to_owned(),
        data: UserGeneralData {
            name: "John Smith".to_owned(),
            nickname: String::new(),
            about_me: String::new(),
        },
        enabled: true,
        account_type: AccountType::Admin,
    };
    // Create two accounts and log in as one of them.
    register(&server, "1@gmail.com".to_owned()).await;
    register(&server, "2@gmail.com".to_owned()).await;
    login(&server, "2@gmail.com".to_owned()).await;

    let response = server
        .post("/admin/users/1/update-data")
        .json(&update_data)
        .await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_change_password(db: PgPool) {
    let server = setup(db);

    let new_password = AdminPasswordChangeRequestBody {
        new_password: "Banaan123!".to_owned(),
    };
    // Create two accounts and log in as one of them.
    register(&server, "1@gmail.com".to_owned()).await;
    register(&server, "2@gmail.com".to_owned()).await;
    login(&server, "2@gmail.com".to_owned()).await;

    let response = server
        .post("/admin/users/1/update-password")
        .json(&new_password)
        .await;
    // It should fail because the user is not an admin
    response.assert_status_forbidden();
}
