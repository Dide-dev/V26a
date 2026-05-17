/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

mod cards;
mod classrooms;
mod feedback;
mod images;
mod trees;
mod users;

use crate::ArcState;
use crate::api_v1::users::UserResponse;
use crate::config::Mailer;
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::{RegisterData, UserLanguage};
use crate::errors::ApiError;
use async_trait::async_trait;
use axum::Extension;
use axum_test::TestServer;
use lettre::transport::stub::AsyncStubTransport;
use lettre::{AsyncTransport, Message};
use sqlx::PgPool;
use std::sync::Arc;
use tempfile::tempdir;

/// A struct for testing, containing the [`TestServer`] and [`ArcState`].
pub struct TestState {
    pub server: TestServer,
    pub state: ArcState,
}

/// Mailer to be used for tests. Does not mail.
pub struct StubMailer {
    transport: AsyncStubTransport,
}

#[async_trait]
impl Mailer for StubMailer {
    async fn send(&self, message: Message) -> Result<(), ApiError> {
        self.transport.send(message).await.unwrap();
        Ok(())
    }
}

/// Sets up an API test server with the given mock database.
///
/// # Arguments
/// `db` - A [`PgPool`] to use for the [`TestServer`].
///
/// # Returns
/// A [`TestServer`] that can handle all the API endpoints and saves cookies.
fn setup(db: PgPool) -> TestState {
    let temp_dir = tempdir().unwrap();
    let stub_mailer = AsyncStubTransport::new_ok();

    let mut config = crate::Config::load();
    config.upload_dir = temp_dir.path().to_string_lossy().to_string();
    // Set a low bcrypt cost to speed up password hashing during testing.
    config.bcrypt_cost = 4;
    config.mailer = Arc::new(StubMailer {
        transport: stub_mailer,
    });

    let state = ArcState::new(config);
    let router = crate::api_v1::v1_api(state.clone())
        .layer(Extension(db))
        .with_state(state.clone());
    TestState {
        server: TestServer::builder().save_cookies().build(router),
        state,
    }
}

/// Creates an account with the given [`TestState`]
///
/// # Arguments
/// `test_state` - The [`TestState`] to use for making the API calls and extracting the verification code.
///
/// `register_data` - The new user's registration data.
///
/// # Returns
/// An i32 representing the id of the user created.
async fn register(test_state: &TestState, register_data: &RegisterData) -> i32 {
    let response = test_state
        .server
        .post("/users/add")
        .json(&register_data)
        .await;
    response.assert_status_ok();

    let verification_code = {
        let pending_accounts = test_state.state.pending_accounts.lock().await;
        pending_accounts
            .iter()
            .find(|(_, p)| p.register_data.email == register_data.email)
            .unwrap()
            .0
            .clone()
    };

    let response = test_state
        .server
        .get(&format!("/users/verify?code={verification_code}"))
        .await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    let UserResponse::Success { user_id } = response else {
        // Adding a user should never fail in a test
        panic!("Something unexpected failed while adding a user")
    };
    user_id
}

/// A convenience function for registering in the testing environment, using only an e-mail.
/// Creates an account with the given [`TestState`]
///
/// # Arguments
/// `test_state` - The [`TestState`] to use for making the API calls and extracting the verification code.
///
/// `email` - The new user's email address.
///
/// # Returns
/// An i32 representing the id of the user created.
async fn register_email(test_state: &TestState, email: String) -> i32 {
    let register_data = RegisterData {
        email: email.clone(),
        password: "a$$word".to_string(),
        name: format!("John Test {}", uuid::Uuid::new_v4()),
        nickname: String::new(),
        about_me: String::new(),
        email_visible: false,
        language: UserLanguage::En,
    };

    register(test_state, &register_data).await
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
    let test_state = setup(db);

    let test_email = "testing@gmail.com".to_owned();
    register_email(&test_state, test_email.clone()).await;

    let server = test_state.server;
    login(&server, test_email).await;

    server
}
