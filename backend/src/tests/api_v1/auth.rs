/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::auth::LoginResponse;
use crate::db_structs::auth::LoginRequestBody;
use crate::tests::api_v1::{login, register, setup};
use sqlx::PgPool;

#[sqlx::test]
async fn test_login_success(db: PgPool) {
    let server = setup(db);
    register(&server, "gorilla@gmail.com".to_owned()).await;

    let response = server
        .post("/auth/login")
        .json(&LoginRequestBody {
            email: "gorilla@gmail.com".to_owned(),
            password: "a$$word".to_owned(),
        })
        .await;

    response.assert_status_ok();
    assert!(response.json::<LoginResponse>().succeeded);

    // Checks if cookie is set after log in.
    assert!(response.headers().contains_key("set-cookie"));

    // Checks if there is a valid session present.
    server.get("/users/me").await.assert_status_ok();
}

#[sqlx::test]
async fn test_login_fail(db: PgPool) {
    let server = setup(db);
    register(&server, "Caesar@gmail.com".to_owned()).await;

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
    server.get("/users/me").await.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_logout_success(db: PgPool) {
    let server = setup(db);
    let user_id = register(&server, "Caesar@gmail.com".to_owned()).await;
    login(&server, "Caesar@gmail.com".to_owned()).await;

    let response = server.post(&format!("/auth/{user_id}/logout")).await;

    response.assert_status_ok();
    // Checks if a cookie is unset.
    assert!(response.headers().contains_key("set-cookie"));

    // Check that the session is unvalidated.
    server.get("/users/me").await.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_logout_failure(db: PgPool) {
    let server = setup(db);
    let user_id = register(&server, "Caesar@gmail.com".to_owned()).await;

    // logout is not possible without a valid session.
    server
        .post(&format!("/auth/{user_id}/logout"))
        .await
        .assert_status_unauthorized();
}
