#![cfg(test)]

mod demo;
mod trees;

use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::{UserData, UserLanguage};
use axum_test::TestServer;
use axum::Extension;
use sqlx::PgPool;

fn setup(db: PgPool) -> TestServer {
    let router = crate::api_v1::v1_api().layer(Extension(db)).with_state(crate::Config::load());
    TestServer::builder().save_cookies().build(router) 
}

async fn setup_login(db: PgPool) -> TestServer {
    let server = setup(db);

    let response = server.post("/users/add").json(&UserData {
        email: "testing@gmail.com".to_string(),
        password: "a$$word".to_string(),
        name: "John Test".to_string(),
        nickname: None,
        about_me: None,
        email_visible: false,
        language: UserLanguage::En,
    }).await;
    response.assert_status_ok();


    let response = server.post("/auth/login").json(&LoginRequestBody {
        email: "testing@gmail.com".to_string(),
        password: "a$$word".to_string(),
    }).await;
    response.assert_status_ok();

    server
}
