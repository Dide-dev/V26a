/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

mod admin;
mod auth;
mod cards;
mod cards_library;
mod classrooms;
mod feedback;
mod images;
mod lessons;
mod trees;
mod users;

use crate::ArcState;
use crate::api_v1::users::{UserResponse, VerificationCode};
use crate::config::Config;
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::classrooms::{ClassroomAddData, ClassroomGeneratedData};
use crate::db_structs::mail::StubMailer;
use crate::db_structs::trees::{RootNode, TreeAddData, TreeData, TreeStructure};
use crate::db_structs::users::RegisterData;
use axum::Extension;
use axum_extra::extract::cookie::Cookie;
use axum_test::{TestResponse, TestServer};
use lettre::transport::stub::AsyncStubTransport;
use percent_encoding::percent_decode_str;
use serde::de::DeserializeOwned;
use sqlx::{PgExecutor, PgPool, query};
use std::sync::Arc;
use tempfile::tempdir;

/// A struct for testing, containing the [`TestServer`] and [`ArcState`].
pub struct TestState {
    pub server: TestServer,
    pub state: ArcState,
}

/// Retrieve and decode the value of a cookie, as [`T: DeserializeOwned`](T)
///
/// # Arguments
/// `cookie` - The [`Cookie`] to retrieve the value from
///
/// # Returns
/// The retrieved value, decoded as type [`T: DeserializeOwned`](T)
fn get_cookie_data<T: DeserializeOwned>(cookie: &Cookie) -> T {
    let value = cookie.value();
    let decoded_value = percent_decode_str(value).decode_utf8().unwrap();
    serde_json::from_str(&decoded_value).unwrap()
}

/// Sets up an API test server with the given mock database, application config, and other settings.
/// The following settings will be overridden:
/// - `upload_dir`,
/// - `bcrypt_cost`,
/// - `mailer`
///
/// # Arguments
/// `db` - A [`PgPool`] to use for the [`TestServer`].
///
/// `config` - The [`Config`] options to use in the application.
///
/// `http_transport` - Configures the server to use real HTTP communication instead of mocking it.
/// It is more expensive to run this way, but for testing websockets it is required.
///
/// # Returns
/// A [`TestState`] containing a [`TestServer`] that can handle all the API endpoints and saves cookies.
fn setup_config(db: PgPool, config: &mut Config, http_transport: bool) -> TestState {
    // Apply some default settings during testing
    let temp_dir = tempdir().unwrap();
    let stub_mailer = AsyncStubTransport::new_ok();
    config.upload_dir = temp_dir.path().to_string_lossy().to_string();
    // Set a low bcrypt cost to speed up password hashing during testing.
    config.bcrypt_cost = 4;
    config.mailer = Arc::new(StubMailer {
        transport: stub_mailer,
    });

    let state = ArcState::new(config.to_owned());

    let transport = if http_transport {
        axum_test::Transport::HttpRandomPort
    } else {
        axum_test::Transport::default()
    };

    let router = crate::api_v1::v1_api(state.clone())
        .layer(Extension(db))
        .with_state(state.clone());
    let server = TestServer::builder()
        .transport(transport)
        .save_cookies()
        .build(router);
    TestState { server, state }
}

/// Sets up an API test server with the given mock database, with default config and settings.
///
/// # Arguments
/// `db` - A [`PgPool`] to use for the [`TestServer`].
///
/// # Returns
/// A [`TestState`] containing a [`TestServer`] that can handle all the API endpoints and saves cookies.
fn setup(db: PgPool) -> TestState {
    let mut config = Config::load();

    setup_config(db, &mut config, false)
}

/// Try to verify an account on the test server with the given email.
///
/// # Arguments
/// `test_state` - The [`TestState`] to use for making the API calls and extracting the verification code.
///
/// `email` - The email address of the account to be verified.
///
/// # Returns
/// The response of the verification request as a [`TestResponse`].
///
/// # Panics
/// Will panic if no pending account with the given e-mail could be found.
async fn try_verify(test_state: &TestState, email: String) -> TestResponse {
    let verification_code = {
        let pending_accounts = test_state.state.pending_accounts.lock().await;
        pending_accounts
            .iter()
            .find(|(_, p)| p.register_data.email == email)
            .unwrap()
            .0
            .clone()
    };

    test_state
        .server
        .post("/users/verification")
        .json(&VerificationCode {
            code: verification_code,
        })
        .await
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
    let response = test_state.server.post("/users").json(&register_data).await;
    response.assert_status_ok();

    let response = try_verify(test_state, register_data.email.clone()).await;
    response.assert_status_ok();

    let response = response.json::<UserResponse>();
    let UserResponse::Success { user_id } = response else {
        // Adding a user should never fail in a test
        panic!("Something unexpected failed while adding a user")
    };

    user_id
}

/// Creates an admin account with the given [`TestServer`]
///
/// # Arguments
/// `test_server` - The [`TestServer`] to use for making the API calls.
///
/// `db` - A [`PgConnection`] to the database used to make the [`TestServer`]
///
/// `email` - The new admin's email address.
///
/// # Returns
/// An i32 representing the id of the admin created.
async fn register_admin(
    test_state: &TestState,
    db: impl PgExecutor<'_>,
    register_data: &RegisterData,
) -> i32 {
    let user_id = register(test_state, register_data).await;

    query!(
        r"UPDATE users SET account_type='admin' WHERE id = $1",
        user_id
    )
    .execute(db)
    .await
    .unwrap();

    user_id
}

/// Log in as a user on the [`TestServer`], setting the proper cookie for usage with protected endpoints.
///
/// # Arguments
/// `test_server` - The [`TestServer`] to use for making the API calls.
///
/// `email` - The user's email address.
async fn login(test_server: &TestServer, email: String) -> TestResponse {
    let response = test_server
        .post("/auth")
        .json(&LoginRequestBody {
            email,
            password: RegisterData::test_values().password,
        })
        .await;
    response.assert_status_ok();

    response
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

    let register_data = RegisterData::test_values();
    register(&test_state, &register_data).await;

    let server = test_state.server;
    login(&server, register_data.email).await;

    server
}

/// Sets up an API test server with the given mock database, creates an admin account and logs in.
///
/// # Arguments
/// `db` - A [`PgPool`] to use to create the [`TestServer`].
///
/// # Returns
/// A [`TestServer`] that can handle all the API endpoints, including ones
/// that require being logged in as admin, and saves cookies.
async fn setup_admin(db: PgPool) -> TestServer {
    let conn = crate::get_connection_unwrap!(db);
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    register_admin(&test_state, &mut *conn, &register_data).await;

    let server = test_state.server;
    login(&server, register_data.email).await;

    server
}

/// Creates a basic tree that can be used in tests.
///
/// # Arguments
/// `test_server` - The [`TestServer`] to use for making the API calls.
///
/// # Returns
/// An i32 representing the tree's id.
async fn create_tree(test_server: &TestServer) -> i32 {
    let tree_data = TreeData::default();
    let tree_structure = TreeStructure {
        root_nodes: vec![RootNode {
            label: "RootNode".to_string(),
            children: vec![],
        }],
    };
    let tree_add_data = TreeAddData {
        tree_data,
        tree_structure,
    };
    let response = test_server.post("/trees").json(&tree_add_data).await;
    response.json()
}

/// Creates a basic classroom that can be used in tests.
///
/// # Arguments
/// `test_server` - The [`TestServer`] to use for making the API calls.
///
/// # Returns
/// An i32 representing the classroom's id.
async fn create_classroom(test_server: &TestServer) -> i32 {
    let classroom_data = ClassroomAddData {
        name: "Classroom".to_string(),
        description: "Classroom description".to_string(),
    };
    let response = test_server.post("/classrooms").json(&classroom_data).await;
    response.json::<ClassroomGeneratedData>().id
}

#[cfg(test)]
mod tests {
    use crate::db_structs::session::InfoCookieData;
    use crate::db_structs::users::{AccountType, RegisterData};
    use crate::get_connection_unwrap;
    use crate::tests::api_v1::{get_cookie_data, login, register, register_admin, setup};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_normal_login(db: PgPool) {
        let test_state = setup(db);

        let register_data = RegisterData::test_values();
        register(&test_state, &register_data).await;
        let server = test_state.server;
        let res = login(&server, register_data.email).await;
        let cookie = res.cookie("user_info");
        let user_info = get_cookie_data::<InfoCookieData>(&cookie);
        assert_eq!(user_info.account_type, AccountType::Author);
    }

    #[sqlx::test]
    async fn test_admin_login(db: PgPool) {
        let conn = get_connection_unwrap!(db);
        let test_state = setup(db);

        let register_data = RegisterData::test_values();
        register_admin(&test_state, conn, &register_data).await;
        let server = test_state.server;
        let res = login(&server, register_data.email).await;
        let cookie = res.cookie("user_info");
        let user_info = get_cookie_data::<InfoCookieData>(&cookie);
        assert_eq!(user_info.account_type, AccountType::Admin);
    }
}
