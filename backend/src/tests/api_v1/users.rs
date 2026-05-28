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
use crate::tests::api_v1::{TestState, login, register, register_email, setup};
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_and_update_own_user_data(db: PgPool) {
    let test_state = setup(db);

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

    register(&test_state, &register_data).await;
    let server = test_state.server;

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
async fn test_get_all_users(db: PgPool) {
    let test_state = setup(db);

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
async fn test_update_other_user(db: PgPool) {
    let test_state = setup(db);

    // Create two accounts and log in as one of them.
    let user_id_1 = register_email(&test_state, "1@gmail.com".to_owned()).await;
    register_email(&test_state, "2@gmail.com".to_owned()).await;
    let server = test_state.server;
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
    let test_state = setup(db);
    let user_id = register_email(&test_state, "test@gmail.com".to_owned()).await;
    let server = test_state.server;
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
    let test_state = setup(db);
    let user_id = register_email(&test_state, "test@gmail.com".to_owned()).await;
    let server = test_state.server;
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

#[sqlx::test]
async fn test_successful_password_change(db: PgPool) {
    let test_state = setup(db);

    let email = "test@test.nl";
    register_email(&test_state, email.to_owned()).await;
    let server = test_state.server;
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
    let test_state = setup(db);

    let email = "test@test.nl";
    register_email(&test_state, email.to_owned()).await;
    let server = test_state.server;
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
    let test_state = setup(db);
    let email = "monkaS@gmail.com".to_owned();
    let user_id = register_email(&test_state, email.clone()).await;
    let server = test_state.server;
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
    let test_state = setup(db);

    let email1 = "user1@gmail.com".to_owned();
    let email2 = "user2@gmail.com".to_owned();
    let user_id_1 = register_email(&test_state, email1.clone()).await;
    register_email(&test_state, email2.clone()).await;

    let server = test_state.server;
    login(&server, email2.clone()).await;

    let response = server
        .delete(&format!("/users/{user_id_1}/delete?deleteOwnedTrees=false"))
        .await;

    response.assert_status_forbidden();
}

#[sqlx::test]
async fn test_delete_nonexistent_user(db: PgPool) {
    let test_state = setup(db);

    let email = "swag@gmail.com".to_owned();
    register_email(&test_state, email.clone()).await;
    let server = test_state.server;
    login(&server, email.clone()).await;

    let response = server
        .delete("/users/99999/delete?deleteOwnedTrees=false")
        .await;

    response.assert_status_bad_request();
}

#[sqlx::test]
async fn test_invalid_validation_code(db: PgPool) {
    let TestState { server, state } = setup(db);

    let response = server
        .post("/users/add")
        .json(&RegisterData {
            email: "johnsmith@gmail.com".to_owned(),
            password: "VeryStrongPassword1!".to_owned(),
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        })
        .await;
    response.assert_status_ok();
    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::Pending);

    let before_pending_count = { state.pending_accounts.lock().await.len() };

    let response = server.get("/users/verify?code=1").await;
    response.assert_status_ok();
    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::InvalidVerification);

    let after_pending_count = { state.pending_accounts.lock().await.len() };
    assert_eq!(before_pending_count, after_pending_count);
}

#[sqlx::test]
async fn test_register_pending_user(db: PgPool) {
    let server = setup(db).server;

    let response = server
        .post("/users/add")
        .json(&RegisterData {
            email: "johnsmith@gmail.com".to_owned(),
            password: "VeryStrongPassword1!".to_owned(),
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        })
        .await;
    response.assert_status_ok();
    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::Pending);

    // Name already used
    let response = server
        .post("/users/add")
        .json(&RegisterData {
            email: "janedoe@gmail.com".to_owned(),
            password: "VeryStrongPassword1!".to_owned(),
            name: "name".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        })
        .await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::UsernameAlreadyUsed);

    // E-mail already used
    let response = server
        .post("/users/add")
        .json(&RegisterData {
            email: "johnsmith@gmail.com".to_owned(),
            password: "VeryStrongPassword1!".to_owned(),
            name: "name2".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        })
        .await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::EmailAlreadyUsed);
}

#[sqlx::test]
async fn test_update_pending_user(db: PgPool) {
    let test_state = setup(db);

    let verified_user = RegisterData {
        email: "johnsmith@localhost".to_string(),
        password: "a$$word".to_string(),
        name: "first".to_string(),
        nickname: "bob".to_string(),
        about_me: "I'm bob".to_string(),
        email_visible: false,
        language: UserLanguage::En,
    };
    let real_user_id = register(&test_state, &verified_user).await;

    let server = test_state.server;
    let response = server
        .post("/users/add")
        .json(&RegisterData {
            email: "janedoe@gmail.com".to_owned(),
            password: "VeryStrongPassword1!".to_owned(),
            name: "second".to_owned(),
            nickname: "nickname".to_owned(),
            about_me: "about_me".to_owned(),
            email_visible: true,
            language: UserLanguage::En,
        })
        .await;
    response.assert_status_ok();

    login(&server, verified_user.email).await;
    let updated_user = UserUpdateData {
        name: "second".to_string(),
        nickname: "bob".to_string(),
        about_me: "I'm still bob".to_string(),
        email_visible: false,
        language: UserLanguage::En,
        account_type: None,
        enabled: None,
    };
    let response = server
        .post(&format!("/users/update/{real_user_id}"))
        .json(&updated_user)
        .await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::UsernameAlreadyUsed);
}
