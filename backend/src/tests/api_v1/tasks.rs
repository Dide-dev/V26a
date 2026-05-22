/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::users::UserResponse;
use crate::config::Config;
use crate::db_structs::users::{RegisterData, UserLanguage};
use crate::tests::api_v1::{setup_config, try_verify};
use sqlx::PgPool;

#[sqlx::test]
async fn test_verify_unpruned_account(db: PgPool) {
    let mut config = Config::load();
    // Set verification timer to 0 so a pending account is considered expired immediately.
    // The task is not running so it will not be pruned from the state, but it should still be
    // considered as expired.
    config.verification_timer = 0;

    let test_state = setup_config(db, &mut config);

    let register_data = RegisterData {
        email: "a@a".to_string(),
        password: "a$$word".to_string(),
        name: "John Test".to_string(),
        nickname: "Johnny".to_string(),
        about_me: "John".to_string(),
        email_visible: false,
        language: UserLanguage::En,
    };
    let response = test_state
        .server
        .post("/users/add")
        .json(&register_data)
        .await;
    response.assert_status_ok();

    let response = try_verify(&test_state, register_data.email).await;
    response.assert_status_ok();
    let response = response.json::<UserResponse>();
    assert_eq!(response, UserResponse::InvalidVerification);
}
