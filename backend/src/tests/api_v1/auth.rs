/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::session::InfoCookieData;
use crate::db_structs::auth::LoginRequestBody;
use crate::db_structs::users::{AccountType, UserLanguage, UserUpdateData};
use crate::tests::api_v1::{register, setup};
use percent_encoding::percent_decode_str;
use sqlx::PgPool;

#[sqlx::test]
async fn test_check_user_info_cookie_data_login(db: PgPool) {
    let server = setup(db);
    let email = "KoekieMonster@sesamstraat.nl".to_owned();
    register(&server, email.clone()).await;
    let response = server
        .post("/auth/login")
        .json(&LoginRequestBody {
            email,
            password: "a$$word".to_string(),
        })
        .await;
    let cookies = response.cookies();
    let user_info_cookie = cookies.get("user_info");
    let data = user_info_cookie.unwrap().value();
    let data = percent_decode_str(data).decode_utf8().unwrap();
    let cookie_data: InfoCookieData = serde_json::from_str(&data).unwrap();

    // Fields have left out data as that is time-sensitive
    assert!(cookie_data.name.starts_with("John Test"));
    assert_eq!(cookie_data.nickname, "");
    assert!(cookie_data.sub > 0);
    assert!(cookie_data.exp > 0);
    assert_eq!(cookie_data.account_type, AccountType::Author);
}

#[sqlx::test]
async fn test_check_user_info_cookie_data_update(db: PgPool) {
    let server = setup(db);
    let email = "SidneyMonster@sesamestreet.com".to_owned();
    let user_id = register(&server, email.clone()).await;
    let response = server
        .post("/auth/login")
        .json(&LoginRequestBody {
            email,
            password: "a$$word".to_string(),
        })
        .await;
    let cookies = response.cookies();
    let user_info_cookie = cookies.get("user_info");
    let data = user_info_cookie.unwrap().value();
    let data = percent_decode_str(data).decode_utf8().unwrap();
    let original_cookie_data: InfoCookieData = serde_json::from_str(&data).unwrap();

    let response = server
        .post(&format!("/users/update/{user_id}"))
        .json(&UserUpdateData {
            name: "Sidney".to_string(),
            nickname: "Cookie Monster".to_string(),
            about_me: "I like cookies, yum".to_string(),
            email_visible: false,
            language: UserLanguage::En,
            account_type: None,
            enabled: None,
        })
        .await;
    response.assert_status_ok();
    let cookies = response.cookies();
    let user_info_cookie = cookies.get("user_info");
    let data = user_info_cookie.unwrap().value();
    let data = percent_decode_str(data).decode_utf8().unwrap();
    let new_cookie_data: InfoCookieData = serde_json::from_str(&data).unwrap();

    assert_eq!(new_cookie_data.name, "Sidney");
    assert_eq!(new_cookie_data.nickname, "Cookie Monster");
    assert_eq!(new_cookie_data.sub, original_cookie_data.sub);
    assert_eq!(new_cookie_data.exp, original_cookie_data.exp);
    assert_eq!(
        new_cookie_data.account_type,
        original_cookie_data.account_type
    );
}
