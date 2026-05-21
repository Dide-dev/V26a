/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

mod admin;
mod auth;
mod cards;
mod classrooms;
mod feedback;
mod images;
mod trees;
mod users;

use crate::ArcState;
use crate::api_v1::users::UserResponse;
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::trees::{RootNode, TreeAddData, TreeData, TreeStructure};
use crate::db_structs::users::{RegisterData, UserLanguage};
use axum::Extension;
use axum_extra::extract::cookie::Cookie;
use axum_test::{TestResponse, TestServer};
use percent_encoding::percent_decode_str;
use serde::de::DeserializeOwned;
use sqlx::{PgExecutor, PgPool, query};
use tempfile::tempdir;

fn get_cookie_data<T: DeserializeOwned>(cookie: &Cookie) -> T {
    let value = cookie.value();
    let decoded_value = percent_decode_str(value).decode_utf8().unwrap();
    serde_json::from_str(&decoded_value).unwrap()
}

/// Sets up an API test server with the given mock database.
///
/// # Arguments
/// `db` - A [`PgPool`] to use for the [`TestServer`].
///
/// # Returns
/// A [`TestServer`] that can handle all the API endpoints and saves cookies.
fn setup(db: PgPool) -> TestServer {
    let temp_dir = tempdir().unwrap();

    let mut config = crate::Config::load();
    config.upload_dir = temp_dir.path().to_string_lossy().to_string();
    // Set a low bcrypt cost to speed up password hashing during testing.
    config.bcrypt_cost = 4;

    let state = ArcState::new(config);
    let router = crate::api_v1::v1_api(state.clone())
        .layer(Extension(db))
        .with_state(state);
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
            name: format!("John Test {}", uuid::Uuid::new_v4()),
            nickname: String::new(),
            about_me: String::new(),
            email_visible: false,
            language: UserLanguage::En,
        })
        .await;
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
async fn register_admin(test_server: &TestServer, db: impl PgExecutor<'_>, email: String) -> i32 {
    let user_id = register(test_server, email).await;

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
        .post("/auth/login")
        .json(&LoginRequestBody {
            email,
            password: "a$$word".to_string(),
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
    let server = setup(db);

    let test_email = "testing@gmail.com".to_owned();
    register(&server, test_email.clone()).await;
    login(&server, test_email).await;

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
    let response = test_server.post("/trees/add").json(&tree_add_data).await;
    response.json()
}

#[cfg(test)]
mod tests {
    use crate::db_structs::session::InfoCookieData;
    use crate::db_structs::users::AccountType;
    use crate::get_connection_unwrap;
    use crate::tests::api_v1::{get_cookie_data, login, register, register_admin, setup};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_normal_login(db: PgPool) {
        let server = setup(db);

        let user_email = "user@localhost";
        register(&server, user_email.to_owned()).await;
        let res = login(&server, user_email.to_owned()).await;
        let cookie = res.cookie("user_info");
        let user_info = get_cookie_data::<InfoCookieData>(&cookie);
        assert_eq!(user_info.account_type, AccountType::Author);
    }

    #[sqlx::test]
    async fn test_admin_login(db: PgPool) {
        let conn = get_connection_unwrap!(db);
        let server = setup(db);

        let admin_email = "admin@localhost";
        register_admin(&server, conn, admin_email.to_owned()).await;
        let res = login(&server, admin_email.to_owned()).await;
        let cookie = res.cookie("user_info");
        let user_info = get_cookie_data::<InfoCookieData>(&cookie);
        assert_eq!(user_info.account_type, AccountType::Admin);
    }
}
