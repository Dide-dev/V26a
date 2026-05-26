/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::session::InfoCookieData;
use crate::db_structs::users::{AccountType, UserData, UserLanguage};
use crate::tests::api_v1::{get_cookie_data, login, register, setup};
use sqlx::PgPool;

#[sqlx::test]
async fn test_check_user_info_cookie_data_login(db: PgPool) {
    let server = setup(db);
    let email = "KoekieMonster@sesamstraat.nl".to_owned();
    let user_id = register(&server, email.clone()).await;
    let response = login(&server, email).await;
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
    let email = "SidneyMonster@sesamestreet.com".to_owned();
    register(&server, email.clone()).await;
    let response = login(&server, email).await;
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
