/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::classrooms::verify_classroom_permissions;
use crate::db_structs::classrooms::{ClassroomAddData, ClassroomGeneratedData, LessonAddData};
use crate::db_structs::session::Claims;
use crate::db_structs::users::AccountType;
use crate::errors::ApiError;
use crate::get_connection_unwrap;
use crate::tests::api_v1::{create_classroom, login, register, setup, setup_login};
use sqlx::PgPool;

#[sqlx::test]
async fn test_new_classroom(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server
        .post("/classrooms/add")
        .json(&classroom_add_data)
        .await;
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
    let server = setup(db);

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server
        .post("/classrooms/add")
        .json(&classroom_add_data)
        .await;
    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_new_lesson(db: PgPool) {
    let mut server = setup_login(db.clone()).await;
    let classroom_id: i32 = i32::MAX;

    let lesson_add_data = LessonAddData {
        name: "Booleans 101".to_string(),
        description:
            "In today's lesson we will explore the exciting possibilities of the Boolean Data Type"
                .to_string(),
        published: false,
    };

    let response = server
        .post(&format!("/classrooms/{classroom_id}/add-lesson"))
        .json(&lesson_add_data)
        .await;
    response.assert_status_bad_request();
    response.assert_text(ApiError::MissingClassroom(classroom_id).to_string());

    let classroom_id: i32 = create_classroom(&server).await;
    let response = server
        .post(&format!("/classrooms/{classroom_id}/add-lesson"))
        .json(&lesson_add_data)
        .await;
    response.assert_status_ok();

    server.clear_cookies();
    let response = server
        .post(&format!("/classrooms/{classroom_id}/add-lesson"))
        .json(&lesson_add_data)
        .await;
    response.assert_status_unauthorized();
    response.assert_text(ApiError::MissingAuthentication.to_string());

    let other_user_email = "ClassroomBandit@ThievesGuildOfAmerica.org";
    register(&server, other_user_email.to_owned()).await;
    login(&server, other_user_email.to_owned()).await;
    let response = server
        .post(&format!("/classrooms/{classroom_id}/add-lesson"))
        .json(&lesson_add_data)
        .await;
    response.assert_status_forbidden();
    response.assert_text(ApiError::NoPermission.to_string());
}

#[sqlx::test]
async fn test_verify_tree_permission_owner(db: PgPool) {
    let server = setup_login(db.clone()).await;
    let classroom_id: i32 = create_classroom(&server).await;

    let claims = Claims {
        sub: 1,
        exp: i64::MAX,
        account_type: AccountType::Author,
    };

    let result =
        verify_classroom_permissions(get_connection_unwrap!(db), classroom_id, claims).await;
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
        verify_classroom_permissions(get_connection_unwrap!(db), classroom_id, claims).await;
    assert!(result.is_ok());
}
