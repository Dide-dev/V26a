/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::classrooms::verify_classroom_permissions;
use crate::db_structs::classrooms::{
    ClassroomData, ClassroomGeneratedData, ClassroomListResponse, LessonAddData,
};
use crate::db_structs::session::Claims;
use crate::db_structs::users::{AccountType, RegisterData};
use crate::errors::ApiError;
use crate::get_connection_unwrap;
use crate::tests::api_v1::{create_classroom, login, register, setup, setup_login};
use sqlx::PgPool;

#[sqlx::test]
async fn test_new_classroom(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server.post("/classrooms").json(&classroom_add_data).await;
    response.assert_status_ok();

    let classroom_created_data = response.json::<ClassroomGeneratedData>();

    let allowed_chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();

    assert_eq!(classroom_created_data.join_code.len(), 6);
    assert!(
        classroom_created_data
            .join_code
            .chars()
            .all(|c| allowed_chars.contains(&c))
    );
}

#[sqlx::test]
async fn test_new_classroom_forbidden(db: PgPool) {
    let server = setup(db).server;

    let classroom_add_data = ClassroomData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server.post("/classrooms").json(&classroom_add_data).await;
    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_get_classrooms(db: PgPool) {
    let test_state = setup(db);

    let account1 = RegisterData::test_values();
    let user_id = register(&test_state, &account1).await;
    let server = test_state.server;
    login(&server, account1.email).await;

    let classroom_add_data = ClassroomData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server.post("/classrooms").json(&classroom_add_data).await;
    response.assert_status_ok();

    let response2 = server.get(&format!("/users/{user_id}/classrooms")).await;
    response2.assert_status_ok();

    let classroom_list: ClassroomListResponse = response2.json();

    assert_eq!(classroom_list.total_classrooms, 1);
}

#[sqlx::test]
async fn test_get_classrooms_unauthorized(db: PgPool) {
    let server = setup(db).server;

    let response = server.get("/users/me/classrooms").await;
    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_new_lesson(db: PgPool) {
    let test_state = setup(db);

    let teacher_account = RegisterData::test_values();
    let other_account = RegisterData::test_values();
    register(&test_state, &teacher_account).await;
    register(&test_state, &other_account).await;
    // Mutable to be able to clear cookies later
    let mut server = test_state.server;
    login(&server, teacher_account.email).await;
    let classroom_id: i32 = i32::MAX;

    let lesson_add_data = LessonAddData {
        name: "Booleans 101".to_string(),
        description:
            "In today's lesson we will explore the exciting possibilities of the Boolean Data Type"
                .to_string(),
        published: false,
    };

    let response = server
        .post(&format!("/classrooms/{classroom_id}/lessons"))
        .json(&lesson_add_data)
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::MissingClassroom(classroom_id).to_string());

    let classroom_id: i32 = create_classroom(&server).await;
    let response = server
        .post(&format!("/classrooms/{classroom_id}/lessons"))
        .json(&lesson_add_data)
        .await;
    response.assert_status_ok();

    server.clear_cookies();
    let response = server
        .post(&format!("/classrooms/{classroom_id}/lessons"))
        .json(&lesson_add_data)
        .await;
    response.assert_status_unauthorized();
    response.assert_text(ApiError::MissingAuthentication.to_string());

    login(&server, other_account.email).await;
    let response = server
        .post(&format!("/classrooms/{classroom_id}/lessons"))
        .json(&lesson_add_data)
        .await;
    response.assert_status_forbidden();
    response.assert_text(ApiError::NoPermission.to_string());
}

#[sqlx::test]
async fn test_verify_classroom_permission_owner(db: PgPool) {
    let server = setup_login(db.clone()).await;
    let classroom_id: i32 = create_classroom(&server).await;

    let claims = Claims {
        sub: 1,
        exp: i64::MAX,
        account_type: AccountType::Author,
    };

    let result =
        verify_classroom_permissions(get_connection_unwrap!(db), classroom_id, &claims).await;
    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_verify_classroom_permission_admin(db: PgPool) {
    let server = setup_login(db.clone()).await;
    let classroom_id: i32 = create_classroom(&server).await;

    let claims = Claims {
        sub: 1,
        exp: i64::MAX,
        account_type: AccountType::Admin,
    };

    let result =
        verify_classroom_permissions(get_connection_unwrap!(db), classroom_id, &claims).await;
    assert!(result.is_ok());
}
