/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::users::{AccountType, AdminUpdateData, UserLanguage};
use crate::tests::api_v1::{login, register, setup};
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_all_users(db: PgPool) {
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
async fn test_update_user(db: PgPool) {
    let server = setup(db);

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
