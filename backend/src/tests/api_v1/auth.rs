/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::auth::LoginResponse;
use crate::db::session::get_session;
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::session::InfoCookieData;
use crate::db_structs::users::{AccountType, RegisterData, UserData, UserLanguage};
use crate::tests::api_v1::{get_cookie_data, login, register, setup};
use sqlx::PgPool;

#[sqlx::test]
async fn test_login_fail(db: PgPool) {
    let server = setup(db.clone());
    let register_data = RegisterData::test_values();
    register(&server, &register_data).await;

    let response = server
        .post("/auth/login")
        .json(&LoginRequestBody {
            email: "Caesar@gmail.com".to_owned(),
            password: "banana".to_owned(),
        })
        .await;

    response.assert_status_ok();
    assert!(!response.json::<LoginResponse>().succeeded);

    // Checks if a cookie is not set.
    assert!(!response.headers().contains_key("set-cookie"));

    // Checks that there is no valid session present.
    let session = get_session(&db, "expected_token").await.unwrap();
    assert!(session.is_none());
    server.get("/users/me").await.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_logout_success(db: PgPool) {
    let server = setup(db.clone());

    let register_data = RegisterData::test_values();
    register(&server, &register_data).await;

    login(&server, register_data.email.clone()).await;

    let response = server.post("/auth/logout").await;

    let cookie = response.headers().get("set-cookie").unwrap().clone();

    // extract token
    let token = cookie
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .strip_prefix("session=")
        .unwrap();

    // Checks if a header exists that will unset the cookie.
    assert!(response.headers().contains_key("set-cookie"));

    // Check that the session is invalidated.
    let session = get_session(&db, token).await.unwrap();
    assert!(session.is_none());
}

#[sqlx::test]
async fn test_logout_failure(db: PgPool) {
    let server = setup(db);
    let register_data = RegisterData::test_values();
    register(&server, &register_data).await;

    // logout is not possible without a valid session.
    server
        .post("/auth/logout")
        .await
        .assert_status_unauthorized();
}

#[sqlx::test]
async fn test_check_user_info_cookie_data_login(db: PgPool) {
    let server = setup(db);
    let register_data = RegisterData::test_values();
    let user_id = register(&server, &register_data).await;
    let response = login(&server, register_data.email.clone()).await;
    let cookie = response.cookie("user_info");
    let cookie_data = get_cookie_data::<InfoCookieData>(&cookie);

    assert!(cookie_data.name.starts_with("John_Test"));
    assert_eq!(cookie_data.nickname, "");
    assert_eq!(cookie_data.sub, user_id);
    assert_eq!(cookie_data.account_type, AccountType::Author);
}

#[sqlx::test]
async fn test_check_user_info_cookie_data_update(db: PgPool) {
    let server = setup(db);
    let register_data = RegisterData::test_values();
    register(&server, &register_data).await;
    let response = login(&server, register_data.email.clone()).await;
    let original_cookie = response.cookie("user_info");
    let original_max_age = original_cookie.max_age().unwrap();
    let original_cookie_data = get_cookie_data::<InfoCookieData>(&original_cookie);

    let response = server
        .post("/users/update")
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
