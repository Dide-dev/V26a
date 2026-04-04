/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::{RegisterData, UserData, UserLanguage};
use crate::tests::api_v1::setup;
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_own_user_data(db: PgPool) {
    let server = setup(db);

    let register_data = RegisterData {
        email: "johnsmith@gmail.com".to_owned(),
        password: "VeryStrongPassword1!".to_owned(),
        name: "name".to_owned(),
        nickname: Some("nickname".to_owned()),
        about_me: Some("about_me".to_owned()),
        email_visible: true,
        language: UserLanguage::En,
    };

    let response = server.post("/users/add").json(&register_data).await;
    response.assert_status_ok();

    let expected_user_data = UserData {
        email: register_data.email.clone(),
        name: register_data.name,
        nickname: register_data.nickname,
        about_me: register_data.about_me,
        email_visible: register_data.email_visible,
        language: register_data.language,
    };

    let response = server
        .post("/auth/login")
        .json(&LoginRequestBody {
            email: register_data.email,
            password: register_data.password,
        })
        .await;
    response.assert_status_ok();

    let response = server.get("/users/me").await;
    response.assert_status_ok();

    let user_data: UserData = response.json();
    assert_eq!(user_data, expected_user_data);
}
