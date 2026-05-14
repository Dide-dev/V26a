/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::auth::LoginResponse;
use crate::api_v1::users::{ChangePasswordResponse, UserResponse};
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::{
    PasswordChangeRequestBody, RegisterData, UserGetData, UserLanguage, UserUpdateData,
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
    let response = response.json::<UserResponse>();
    let UserResponse::Success { user_id: _ } = response else {
        // Adding a user should never fail in a test
        panic!("Something unexpected failed while adding a user")
    };

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

    let response = server.post("/users/update").json(&update_data).await;
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
async fn test_successful_password_change(db: PgPool) {
    let server = setup(db);

    let email = "test@test.nl";

    register(&server, email.to_owned()).await;
    login(&server, email.to_owned()).await;

    let change_password_data = PasswordChangeRequestBody {
        old_password: "a$$word".to_owned(),
        new_password: "Pa$$word".to_owned(),
    };

    let response = server
        .post("/users/change-password")
        .json(&change_password_data)
        .await;

    response.assert_status_ok();

    let body: ChangePasswordResponse = response.json();
    assert!(body.succeeded);

    let login_data = LoginRequestBody {
        email: email.to_owned(),
        password: change_password_data.new_password,
    };

    let response = server.post("/auth/login").json(&login_data).await;

    response.assert_status_ok();

    let login_body: LoginResponse = response.json();
    assert!(login_body.succeeded);
}

#[sqlx::test]
async fn test_change_password_incorrect_old_password(db: PgPool) {
    let server = setup(db);

    let email = "test@test.nl";

    register(&server, email.to_owned()).await;
    login(&server, email.to_owned()).await;

    // The default password set is `a$$word`, hence this request should fail.
    let change_password_data = PasswordChangeRequestBody {
        old_password: "Pa$$word".to_owned(),
        new_password: "NewPa$$word".to_owned(),
    };
    let response = server
        .post("/users/change-password")
        .json(&change_password_data)
        .await;

    response.assert_status_ok();

    let body: ChangePasswordResponse = response.json();
    assert!(!body.succeeded);

    // Password change request should not have succeeded, therefore the old password should enable a user to log in.
    let login_data = LoginRequestBody {
        email: email.to_owned(),
        password: "a$$word".to_owned(),
    };

    let response = server.post("/auth/login").json(&login_data).await;

    response.assert_status_ok();

    let login_body: LoginResponse = response.json();
    assert!(login_body.succeeded);
}

#[sqlx::test]
async fn test_delete_own_user(db: PgPool) {
    let server = setup(db);
    let email = "monkaS@gmail.com".to_owned();
    let user_id = register(&server, email.clone()).await;

    login(&server, email.clone()).await;

    let response = server
        .delete(&format!("/users/{user_id}/delete?deleteOwnedTrees=false"))
        .await;

    response.assert_status_ok();

    // Check if user is logged out after deletion.
    let response = server.get("/users/me").await;
    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_delete_other_user(db: PgPool) {
    let server = setup(db);
    let email1 = "user1@gmail.com".to_owned();
    let user_id_1 = register(&server, email1.clone()).await;
    let email2 = "user2@gmail.com".to_owned();
    register(&server, email2.clone()).await;
    login(&server, email2.clone()).await;

    let response = server
        .delete(&format!("/users/{user_id_1}/delete?deleteOwnedTrees=false"))
        .await;

    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_delete_nonexistent_user(db: PgPool) {
    let server = setup(db);
    let email = "swag@gmail.com".to_owned();
    register(&server, email.clone()).await;
    login(&server, email.clone()).await;

    let response = server
        .delete("/users/99999/delete?deleteOwnedTrees=false")
        .await;

    response.assert_status_bad_request();
}
