/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::auth::LoginResponse;
use crate::api_v1::users::{
    ChangePasswordResponse, PasswordResetResponse, UserResponse, VerificationCode,
};
use crate::config::Config;
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::{
    PasswordChangeRequestBody, PasswordResetRequestBody, RegisterData,
    RequestPasswordResetRequestBody, UserData, UserGetData, UserLanguage,
};
use crate::tests::api_v1::{TestState, login, register, setup, setup_config, try_verify};
use sqlx::{PgPool, query_scalar};

#[sqlx::test]
async fn test_get_and_update_own_user_data(db: PgPool) {
    let test_state = setup(db);

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

    register(&test_state, &register_data).await;
    let server = test_state.server;

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
        .post("/auth")
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

    let response = server.put("/users/me").json(&update_data).await;
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
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    register(&test_state, &register_data).await;
    let server = test_state.server;
    login(&server, register_data.email.clone()).await;

    let change_password_data = PasswordChangeRequestBody {
        old_password: register_data.password,
        new_password: "Pa$$word1".to_owned(),
    };

    let response = server
        .put("/users/me/password")
        .json(&change_password_data)
        .await;

    response.assert_status_ok();

    let body: ChangePasswordResponse = response.json();
    assert!(body.succeeded);

    let login_data = LoginRequestBody {
        email: register_data.email,
        password: change_password_data.new_password,
    };

    let response = server.post("/auth").json(&login_data).await;

    response.assert_status_ok();

    let login_body: LoginResponse = response.json();
    assert!(login_body.succeeded);
}

#[sqlx::test]
async fn test_change_password_incorrect_old_password(db: PgPool) {
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    register(&test_state, &register_data).await;
    let server = test_state.server;
    login(&server, register_data.email.clone()).await;

    // The default password set is `4$$wOrd!`, hence this request should fail.
    let change_password_data = PasswordChangeRequestBody {
        old_password: "Pa$$word".to_owned(),
        new_password: "NewPa$$word1".to_owned(),
    };
    let response = server
        .put("/users/me/password")
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

    let response = server.post("/auth").json(&login_data).await;

    response.assert_status_ok();

    let login_body: LoginResponse = response.json();
    assert!(login_body.succeeded);
}

#[sqlx::test]
async fn test_password_change_invalidates_other_sessions(db: PgPool) {
    let test_state = setup(db.clone());

    let register_data = RegisterData::test_values();
    let user_id = register(&test_state, &register_data).await;
    let server = test_state.server;

    // Create two sessions.
    login(&server, register_data.email.clone()).await;
    login(&server, register_data.email.clone()).await;

    // Check that two sessions exist before password change.
    let sessions = query_scalar!("SELECT COUNT(*) FROM sessions WHERE user_id = $1", user_id)
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(sessions, Some(2));

    // Change password.
    server
        .put("/users/me/password")
        .json(&PasswordChangeRequestBody {
            old_password: register_data.password,
            new_password: "P4$$word".to_owned(),
        })
        .await
        .assert_status_ok();

    // Checks that only one session is left. That means other sessions are invalidated.
    let sessions = query_scalar!("SELECT COUNT(*) FROM sessions WHERE user_id = $1", user_id)
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(sessions, Some(1));
}

#[sqlx::test]
async fn test_delete_user(db: PgPool) {
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    register(&test_state, &register_data).await;
    let server = test_state.server;
    login(&server, register_data.email).await;
    let response = server.delete("/users/me?deleteOwnedTrees=false").await;
    response.assert_status_ok();
    // Check if cookies are unset after deletion.
    assert!(
        server
            .get("/any-route")
            .await
            .maybe_cookie("session")
            .is_none()
    );

    // Check that sessions are unvalidated.
    server.get("/users/me").await.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_delete_own_user_invalidates_all_sessions(db: PgPool) {
    let test_state = setup(db.clone());

    let register_data = RegisterData::test_values();
    let user_id = register(&test_state, &register_data).await;
    let server = test_state.server;
    login(&server, register_data.email.clone()).await;
    login(&server, register_data.email.clone()).await;

    // Checks two sessions exist before deletion.
    let sessions = query_scalar!("SELECT COUNT(*) FROM sessions WHERE user_id = $1", user_id)
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(sessions, Some(2));

    server
        .delete("/users/me?deleteOwnedTrees=false")
        .await
        .assert_status_ok();

    // Check that all sessions are invalidated. The amount of sessions should be equal to 0 then.
    let sessions = query_scalar!("SELECT COUNT(*) FROM sessions WHERE user_id = $1", user_id)
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(sessions, Some(0));
}

#[sqlx::test]
async fn test_invalid_validation_code(db: PgPool) {
    let TestState { server, state } = setup(db);

    let register_data = RegisterData::test_values();
    let response = server.post("/users").json(&register_data).await;
    response.assert_status_ok();
    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::Pending);

    let before_pending_count = { state.pending_accounts.lock().await.len() };

    let response = server
        .post("/users/verification")
        .json(&VerificationCode {
            code: "1".to_string(),
        })
        .await;
    response.assert_status_ok();
    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::InvalidVerification);

    let after_pending_count = { state.pending_accounts.lock().await.len() };
    assert_eq!(before_pending_count, after_pending_count);
}

#[sqlx::test]
async fn test_verify_unpruned_account(db: PgPool) {
    let mut config = Config::load();
    // Set verification timer to -1 seconds so a pending account is considered expired immediately.
    // The task is not running so it will not be pruned from the state, but it should still be
    // considered as expired.
    config.verification_timer = -1;

    let test_state = setup_config(db, &mut config, false);

    let register_data = RegisterData::test_values();
    let response = test_state.server.post("/users").json(&register_data).await;
    response.assert_status_ok();

    let response = try_verify(&test_state, register_data.email).await;
    response.assert_status_ok();
    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::InvalidVerification);
}

#[sqlx::test]
async fn test_register_pending_user(db: PgPool) {
    let server = setup(db).server;

    let pending_user_data = RegisterData::test_values();
    let response = server.post("/users").json(&pending_user_data).await;
    response.assert_status_ok();
    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::Pending);

    let name_already_used_data = RegisterData {
        data: UserData {
            name: pending_user_data.data.name,
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: UserLanguage::Nl,
        },
        ..RegisterData::test_values()
    };
    let response = server.post("/users").json(&name_already_used_data).await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::UsernameAlreadyUsed);

    let email_already_used_data = RegisterData {
        email: pending_user_data.email,
        ..RegisterData::test_values()
    };
    let response = server.post("/users").json(&email_already_used_data).await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::EmailAlreadyUsed);
}

#[sqlx::test]
async fn test_update_pending_user(db: PgPool) {
    let test_state = setup(db);

    let verified_user_data = RegisterData::test_values();
    register(&test_state, &verified_user_data).await;

    let server = test_state.server;
    let pending_user_data = RegisterData::test_values();
    let response = server.post("/users").json(&pending_user_data).await;
    response.assert_status_ok();

    login(&server, verified_user_data.email).await;
    let updated_user = UserData {
        name: pending_user_data.data.name,
        nickname: "bob".to_string(),
        about_me: "I'm still bob".to_string(),
        email_visible: false,
        language: UserLanguage::En,
    };
    let response = server.put("/users/me").json(&updated_user).await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::UsernameAlreadyUsed);
}

#[sqlx::test]
async fn test_password_reset(db: PgPool) {
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    let user_id = register(&test_state, &register_data).await;
    let response = login(&test_state.server, register_data.email.clone())
        .await
        .json::<LoginResponse>();
    assert!(response.succeeded);

    let response = test_state
        .server
        .post("/users/reset-password-requests")
        .json(&RequestPasswordResetRequestBody {
            email: register_data.email.clone(),
        })
        .await;
    response.assert_status_ok();

    let reset_code = {
        let reset_password_requests = test_state.state.reset_password_requests.lock().await;
        reset_password_requests
            .iter()
            .find(|(_, r)| r.user_id == user_id)
            .unwrap()
            .0
            .clone()
    };

    let server = test_state.server;
    let new_password = "NewP4sswordThatIWon'tForget!".to_owned();
    let response = server
        .put("/users/reset-password-requests")
        .json(&PasswordResetRequestBody {
            code: reset_code,
            new_password: new_password.clone(),
        })
        .await;
    response.assert_status_ok();
    let response = response.json::<PasswordResetResponse>();
    assert_eq!(response, PasswordResetResponse::Success);

    // Log in with old password
    let response = login(&server, register_data.email.clone())
        .await
        .json::<LoginResponse>();
    assert!(!response.succeeded);

    let response = server
        .post("/auth")
        .json(&LoginRequestBody {
            email: register_data.email,
            password: new_password,
        })
        .await;
    response.assert_status_ok();
}

#[sqlx::test]
async fn test_invalid_password_reset_code(db: PgPool) {
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    register(&test_state, &register_data).await;

    let response = test_state
        .server
        .post("/users/reset-password-requests")
        .json(&RequestPasswordResetRequestBody {
            email: register_data.email,
        })
        .await;
    response.assert_status_ok();

    let before_reset_count = { test_state.state.reset_password_requests.lock().await.len() };

    let response = test_state
        .server
        .put("/users/reset-password-requests")
        .json(&PasswordResetRequestBody {
            code: "1".to_string(),
            new_password: register_data.password,
        })
        .await;
    response.assert_status_ok();
    let response = response.json::<PasswordResetResponse>();
    assert_eq!(response, PasswordResetResponse::InvalidCode);

    let after_pending_count = { test_state.state.reset_password_requests.lock().await.len() };
    assert_eq!(before_reset_count, after_pending_count);
}

#[sqlx::test]
async fn test_password_reset_unpruned(db: PgPool) {
    let mut config = Config::load();
    // Set reset password timer to -1 seconds so a pending reset is considered expired immediately.
    // The task is not running so it will not be pruned from the state, but it should still be
    // considered as expired.
    config.reset_password_timer = -1;

    let test_state = setup_config(db, &mut config, false);

    let register_data = RegisterData::test_values();
    let user_id = register(&test_state, &register_data).await;

    let response = test_state
        .server
        .post("/users/reset-password-requests")
        .json(&RequestPasswordResetRequestBody {
            email: register_data.email,
        })
        .await;
    response.assert_status_ok();

    let reset_code = {
        let reset_password_requests = test_state.state.reset_password_requests.lock().await;
        reset_password_requests
            .iter()
            .find(|(_, r)| r.user_id == user_id)
            .unwrap()
            .0
            .clone()
    };

    let response = test_state
        .server
        .put("/users/reset-password-requests")
        .json(&PasswordResetRequestBody {
            code: reset_code,
            new_password: register_data.password,
        })
        .await;
    response.assert_status_ok();
    let response = response.json::<PasswordResetResponse>();
    assert_eq!(response, PasswordResetResponse::InvalidCode);
}
