/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

mod cards;
mod images;
mod trees;
mod users;

use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::{RegisterData, UserLanguage};
use axum::Extension;
use axum_test::TestServer;
use sqlx::PgPool;

/// Sets up an API test server with the given mock database.
///
/// # Arguments
/// `db` - A [`PgPool`] to use for the [`TestServer`].
///
/// # Returns
/// A [`TestServer`] that can handle all the API endpoints and saves cookies.
fn setup(db: PgPool) -> TestServer {
    let router = crate::api_v1::v1_api()
        .layer(Extension(db))
        .with_state(crate::Config::load());
    TestServer::builder().save_cookies().build(router)
}

/// Creates an account with the given [`TestServer`]
///
/// # Arguments
/// `test_server` - The [`TestServer`] to use for making the API calls.
///
/// `email` - The new user's email address.
///
/// # Returns
/// An i32 representing the id of the user created.
async fn register(test_server: &TestServer, email: String) -> i32 {
    let response = test_server
        .post("/users/add")
        .json(&RegisterData {
            email,
            password: "a$$word".to_string(),
            name: "John Test".to_string(),
            nickname: None,
            about_me: None,
            email_visible: false,
            language: UserLanguage::En,
        })
        .await;
    response.assert_status_ok();

    response.json::<i32>()
}

/// Log in as a user on the [`TestServer`], setting the proper cookie for usage with protected endpoints.
///
/// # Arguments
/// `test_server` - The [`TestServer`] to use for making the API calls.
///
/// `email` - The new user's email address.
async fn login(test_server: &TestServer, email: String) {
    let response = test_server
        .post("/auth/login")
        .json(&LoginRequestBody {
            email,
            password: "a$$word".to_string(),
        })
        .await;
    response.assert_status_ok();
}

/// Sets up an API test server with the given mock database, creates an account and logs in.
///
/// # Arguments
/// `db` - A [`PgPool`] to use to create the [`TestServer`].
///
/// # Returns
/// A [`TestServer`] that can handle all the API endpoints, including ones
/// that require being logged in, and saves cookies.
async fn setup_login(db: PgPool) -> TestServer {
    let server = setup(db);

    let test_email = "testing@gmail.com".to_owned();
    register(&server, test_email.clone()).await;
    login(&server, test_email).await;

    server
}
