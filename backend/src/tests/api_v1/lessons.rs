/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::lessons::verify_lesson_permissions;
use crate::db_structs::classrooms::LessonAddData;
use crate::db_structs::session::Claims;
use crate::db_structs::users::{AccountType, RegisterData};
use crate::errors::ApiError;
use crate::get_connection_unwrap;
use crate::tests::api_v1::{create_classroom, create_tree, login, register, setup_login};
use sqlx::PgPool;

#[sqlx::test]
async fn test_add_tree_to_lesson(db: PgPool) {
    let mut server = setup_login(db.clone()).await;
    let classroom_id = create_classroom(&server).await;
    let lesson_data = LessonAddData {
        name: "Lesson".to_owned(),
        description: "Test".to_owned(),
        published: true,
    };
    let response = server
        .post(&format!("/classrooms/{classroom_id}/lessons"))
        .json(&lesson_data)
        .await;
    response.assert_status_ok();
    let lesson_id: i32 = response.json();

    let tree_id = i32::MAX;
    let response = server
        .post(&format!("/lessons/{lesson_id}/trees/{tree_id}"))
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::MissingTree(tree_id).to_string());

    let tree_id = create_tree(&server).await;
    let response = server
        .post(&format!("/lessons/{lesson_id}/trees/{tree_id}"))
        .await;
    response.assert_status_ok();

    let response = server
        .post(&format!("/lessons/{lesson_id}/trees/{tree_id}"))
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::TreeAlreadyExistsWithinLesson.to_string());

    server.clear_cookies();
    let response = server
        .post(&format!("/lessons/{lesson_id}/trees/{tree_id}"))
        .await;
    response.assert_status_unauthorized();
    response.assert_text(ApiError::MissingAuthentication.to_string());

    let register_data = RegisterData::test_values();
    register(&server, &register_data).await;
    login(&server, register_data.email).await;
    let response = server
        .post(&format!("/lessons/{lesson_id}/trees/{tree_id}"))
        .await;
    response.assert_status_forbidden();
    response.assert_text(ApiError::NoPermission.to_string());

    let lesson_id = i32::MAX;
    let response = server
        .post(&format!("/lessons/{lesson_id}/trees/{tree_id}"))
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::MissingLesson(lesson_id).to_string());
}
#[sqlx::test]
async fn test_verify_lesson_permissions(db: PgPool) {
    let server = setup_login(db.clone()).await;
    let classroom_id = create_classroom(&server).await;
    let lesson_data = LessonAddData {
        name: "Lesson".to_owned(),
        description: "Test".to_owned(),
        published: true,
    };
    let response = server
        .post(&format!("/classrooms/{classroom_id}/lessons"))
        .json(&lesson_data)
        .await;
    let lesson_id: i32 = response.json();

    let claims = Claims {
        sub: 1,
        exp: i64::MAX,
        account_type: AccountType::Author,
    };

    let result = verify_lesson_permissions(get_connection_unwrap!(db), lesson_id, &claims).await;
    assert!(result.is_ok());

    let claims = Claims {
        sub: 1,
        exp: i64::MAX,
        account_type: AccountType::Admin,
    };

    let result = verify_lesson_permissions(get_connection_unwrap!(db), lesson_id, &claims).await;
    assert!(result.is_ok());
}
