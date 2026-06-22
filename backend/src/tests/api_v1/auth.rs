/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::auth::LoginResponse;
use crate::get_connection_unwrap;
use crate::structs::auth::LoginRequestBody;
use crate::structs::session::{Claims, InfoCookieData};
use crate::structs::users::{
    AccountType, AdminUpdateData, RegisterData, UserData, UserGeneralData, UserLanguage,
};
use crate::tests::api_v1::{get_cookie_data, login, register, register_admin, setup};
use sqlx::PgPool;

#[sqlx::test]
async fn test_login_fail(db: PgPool) {
    let test_state = setup(db.clone());
    let register_data = RegisterData::test_values();
    register(&test_state, &register_data).await;

    let server = test_state.server;
    let response = server
        .post("/auth")
        .json(&LoginRequestBody {
            email: "Caesar@gmail.com".to_owned(),
            password: "banana".to_owned(),
        })
        .await;

    response.assert_status_ok();
    assert!(!response.json::<LoginResponse>().succeeded);

    // Checks if a cookie is missing.
    assert!(
        server
            .get("/any-route")
            .await
            .maybe_cookie("session")
            .is_none()
    );

    // Checks that login fails.
    server.get("/users/me").await.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_login_disabled_user(db: PgPool) {
    let state = setup(db.clone());

    let register_data1 = RegisterData::test_values();
    let register_data2 = RegisterData::test_values();
    let user_id = register(&state, &register_data1).await;
    register_admin(&state, get_connection_unwrap!(db.clone()), &register_data2).await;

    let server = state.server;
    login(&server, register_data2.email).await;

    let response = server
        .put(&format!("/admin/users/{user_id}"))
        .json(&AdminUpdateData {
            email: register_data1.email.clone(),
            data: UserGeneralData {
                name: register_data1.data.name,
                nickname: register_data1.data.nickname,
                about_me: register_data1.data.about_me,
            },
            enabled: false,
            account_type: AccountType::Author,
        })
        .await;
    response.assert_status_ok();

    let response = server
        .post("/auth")
        .json(&LoginRequestBody {
            email: register_data1.email,
            password: register_data1.password,
        })
        .await;
    response.assert_status_ok();

    assert!(!response.json::<LoginResponse>().succeeded);
    assert!(response.json::<LoginResponse>().disabled);
}

#[sqlx::test]
async fn test_logout_success(db: PgPool) {
    let test_state = setup(db.clone());
    let register_data = RegisterData::test_values();

    register(&test_state, &register_data).await;
    let server = test_state.server;
    let login_response = login(&server, register_data.email).await;
    let token = login_response.cookie("session").value().to_string();

    server.delete("/auth").await;

    // Check if a cookie is unset.
    assert!(
        server
            .get("/any-route")
            .await
            .maybe_cookie("session")
            .is_none()
    );

    // Check that the session is invalidated.
    let session = Claims::get_session(&db, &token).await.unwrap();
    assert!(session.is_none());
}

#[sqlx::test]
async fn test_logout_failure(db: PgPool) {
    let test_state = setup(db);
    let register_data = RegisterData::test_values();
    register(&test_state, &register_data).await;
    let server = test_state.server;

    // logout is not possible without a valid session.
    server.delete("/auth").await.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_check_user_info_cookie_data_login(db: PgPool) {
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    let user_id = register(&test_state, &register_data).await;
    let server = test_state.server;
    let response = login(&server, register_data.email).await;
    let cookie = response.cookie("user_info");
    let cookie_data = get_cookie_data::<InfoCookieData>(&cookie);

    assert!(cookie_data.name.starts_with("John_Test"));
    assert_eq!(cookie_data.nickname, "");
    assert_eq!(cookie_data.sub, user_id);
    assert_eq!(cookie_data.account_type, AccountType::Author);
}

#[sqlx::test]
async fn test_check_user_info_cookie_data_update(db: PgPool) {
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    register(&test_state, &register_data).await;
    let server = test_state.server;
    let response = login(&server, register_data.email).await;
    let original_cookie = response.cookie("user_info");
    let original_max_age = original_cookie.max_age().unwrap();
    let original_cookie_data = get_cookie_data::<InfoCookieData>(&original_cookie);

    let response = server
        .put("/users/me")
        .json(&UserData {
            name: "Sidney".to_string(),
            nickname: "Cookie Monster".to_string(),
            about_me: "I like cookies, yum".to_string(),
            email_visible: false,
            language: UserLanguage::En,
        })
        .await;
    response.assert_status_ok();
    let new_cookie = response.cookie("user_info");
    let new_max_age = new_cookie.max_age().unwrap();
    let new_cookie_data = get_cookie_data::<InfoCookieData>(&new_cookie);

    assert_eq!(new_cookie_data.name, "Sidney");
    assert_eq!(new_cookie_data.nickname, "Cookie Monster");
    assert_eq!(new_cookie_data.sub, original_cookie_data.sub);
    // The new duration can differ slightly, this is fine.
    assert!((original_max_age.whole_seconds() - new_max_age.whole_seconds()).abs() <= 2);
    assert_eq!(
        new_cookie_data.account_type,
        original_cookie_data.account_type
    );
}
