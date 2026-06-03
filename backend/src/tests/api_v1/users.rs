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
    PasswordChangeRequestBody, RegisterData, UserData, UserGetData, UserLanguage,
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
    let response = response.json::<UserResponse>();
    let UserResponse::Success { user_id: _ } = response else {
        // Adding a user should never fail in a test
        panic!("Something unexpected failed while adding a user")
    };

    let expected_user_data = UserGetData {
        email: register_data.email.clone(),
        data: UserData {
            name: register_data.data.name,
            nickname: register_data.data.nickname,
            about_me: register_data.data.about_me,
            email_visible: register_data.data.email_visible,
            language: register_data.data.language,
        },
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
    let update_data = UserData {
        name: "John_Smith".to_owned(),
        nickname: "John".to_owned(),
        about_me: String::new(),
        email_visible: false,
        language: UserLanguage::Nl,
    };

    let response = server.post("/users/update").json(&update_data).await;
    response.assert_status_ok();

    let expected_user_data = UserGetData {
        email: register_data.email.clone(),
        data: UserData {
            name: update_data.name,
            nickname: update_data.nickname,
            about_me: update_data.about_me,
            email_visible: update_data.email_visible,
            language: update_data.language,
        },
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

    let register_data = RegisterData::test_values();
    register(&server, &register_data).await;
    login(&server, register_data.email.clone()).await;

    let change_password_data = PasswordChangeRequestBody {
        old_password: register_data.password,
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
        email: register_data.email,
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

    let register_data = RegisterData::test_values();
    register(&server, &register_data).await;
    login(&server, register_data.email.clone()).await;

    // The default password set is `4$$wOrd!`, hence this request should fail.
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
        email: register_data.email,
        password: register_data.password,
    };

    let response = server.post("/auth/login").json(&login_data).await;

    response.assert_status_ok();

    let login_body: LoginResponse = response.json();
    assert!(login_body.succeeded);
}

#[sqlx::test]
async fn test_password_change_invalidates_other_sessions(db: PgPool) {
    let server = setup(db.clone());

    let register_data = RegisterData::test_values();
    let user_id = register(&server, &register_data).await;

    // Create two sessions.
    login(&server, register_data.email.clone()).await;
    login(&server, register_data.email.clone()).await;

    // Check that two sessions exist before password change.
    let sessions = sqlx::query!("SELECT token FROM sessions WHERE user_id = $1", user_id)
        .fetch_all(&db)
        .await
        .unwrap();

    assert_eq!(sessions.len(), 2);

    // Change password.
    server
        .post("/users/change-password")
        .json(&PasswordChangeRequestBody {
            old_password: register_data.password,
            new_password: "P4$$word".to_owned(),
        })
        .await
        .assert_status_ok();

    // Checks that only one session is left. That means other sessions are invalidated.
    let sessions = sqlx::query!("SELECT token FROM sessions WHERE user_id = $1", user_id)
        .fetch_all(&db)
        .await
        .unwrap();

    assert_eq!(sessions.len(), 1);
}

#[sqlx::test]
async fn test_delete_own_user(db: PgPool) {
    let server = setup(db);

    let register_data = RegisterData::test_values();
    register(&server, &register_data).await;
    login(&server, register_data.email).await;

    let response = server.delete("/users/delete?deleteOwnedTrees=false").await;

    response.assert_status_ok();

    // Check if cookies are unset after deletion.
    assert!(response.headers().contains_key("set-cookie"));

    // Check that sessions are unvalidated.
    server.get("/users/me").await.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_delete_own_user_invalidates_all_sessions(db: PgPool) {
    let server = setup(db.clone());

    let register_data = RegisterData::test_values();
    let user_id = register(&server, &register_data).await;
    login(&server, register_data.email.clone()).await;
    login(&server, register_data.email.clone()).await;

    // Checks two sessions exist before deletion.
    let sessions = sqlx::query!("SELECT token FROM sessions WHERE user_id = $1", user_id)
        .fetch_all(&db)
        .await
        .unwrap();
    assert_eq!(sessions.len(), 2);

    server
        .delete("/users/delete?deleteOwnedTrees=false")
        .await
        .assert_status_ok();

    // Check that all sessions are invalidated. The amount of sessions should be equal to 0 then.
    let sessions = sqlx::query!("SELECT token FROM sessions WHERE user_id = $1", user_id)
        .fetch_all(&db)
        .await
        .unwrap();
    assert_eq!(sessions.len(), 0);
}

#[sqlx::test]
async fn test_delete_nonexistent_user(db: PgPool) {
    let server = setup(db);

    let response = server.delete("/users/delete?deleteOwnedTrees=false").await;

    response.assert_status_unauthorized();
}
