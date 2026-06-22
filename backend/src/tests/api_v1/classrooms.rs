/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::api_v1::classrooms::teachers::verify_classroom_permissions;
use crate::errors::ApiError;
use crate::get_connection_unwrap;
use crate::structs::classrooms::{
    ClassroomAddData, ClassroomGeneratedData, ClassroomListResponse, ClassroomLoginResponse,
    ClassroomUserLoginData, LessonAddData,
};
use crate::structs::session::{AllClassroomSessions, Session};
use crate::structs::users::{AccountType, RegisterData};
use crate::tests::api_v1::{
    create_classroom, get_cookie_data, login, register, setup, setup_login,
};
use sqlx::PgPool;

#[sqlx::test]
async fn test_new_classroom(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomAddData {
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

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server.post("/classrooms").json(&classroom_add_data).await;
    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_classroom_login_same_classroom(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server.post("/classrooms").json(&classroom_add_data).await;

    response.assert_status_ok();
    let classroom_created_data = response.json::<ClassroomGeneratedData>();

    // ----- No cookie present, new user ----- //

    let classroom_login_data = ClassroomUserLoginData {
        join_code: classroom_created_data.join_code,
        username: "Gelicenseerd radiozendamateur".to_owned(),
    };

    let response = server
        .post("/classrooms/auth")
        .json(&classroom_login_data)
        .await;

    response.assert_status_ok();

    let classroom_login_response = response.json::<ClassroomLoginResponse>();
    let classroom_login_response = match classroom_login_response {
        ClassroomLoginResponse::Success(login_data) => login_data,
        ClassroomLoginResponse::JoinCodeIncorrect { join_code } => {
            panic!("Join code {join_code} incorrect")
        }
        ClassroomLoginResponse::MissingUser {
            user_number: user_num,
        } => {
            panic!("User {user_num} does not exist in classroom")
        }
    };

    let cookie = response.cookie("classroom_user_infos");
    let cookie_data = get_cookie_data::<AllClassroomSessions>(&cookie);

    assert_eq!(cookie_data.sessions.len(), 1);

    // ----- Cookie with same classroom id present, new user ----- //

    let classroom_login_data2 = ClassroomUserLoginData {
        join_code: classroom_login_data.join_code,
        username: "Gelicenseerd radiozendamateur 2".to_owned(),
    };

    let response = server
        .post("/classrooms/auth")
        .json(&classroom_login_data2)
        .await;

    response.assert_status_ok();

    let cookie = response.cookie("classroom_user_infos");
    let cookie_data = get_cookie_data::<AllClassroomSessions>(&cookie);

    assert_eq!(cookie_data.sessions.len(), 1);

    // ----- Cookie with same classroom id present, existing user ----- //

    let classroom_login_data = ClassroomUserLoginData {
        join_code: classroom_login_data2.join_code,
        username: classroom_login_response.num.to_string(),
    };

    let response = server
        .post("/classrooms/auth")
        .json(&classroom_login_data)
        .await;

    response.assert_status_ok();

    let cookie = response.cookie("classroom_user_infos");
    let cookie_data = get_cookie_data::<AllClassroomSessions>(&cookie);

    assert_eq!(cookie_data.sessions.len(), 1);
}

#[sqlx::test]
async fn test_classroom_login_multiple_sessions(db: PgPool) {
    let server = setup_login(db).await;

    // ----- First classroom ----- //

    let classroom_add_data = ClassroomAddData {
        name: "Classroom1".to_owned(),
        description: String::new(),
    };

    let response = server.post("/classrooms").json(&classroom_add_data).await;

    response.assert_status_ok();
    let classroom_created_data = response.json::<ClassroomGeneratedData>();

    let classroom_login_data = ClassroomUserLoginData {
        join_code: classroom_created_data.join_code,
        username: "Gelicenseerd radiozendamateur".to_owned(),
    };

    let response = server
        .post("/classrooms/auth")
        .json(&classroom_login_data)
        .await;

    response.assert_status_ok();

    let cookie = response.cookie("classroom_user_infos");
    let cookie_data = get_cookie_data::<AllClassroomSessions>(&cookie);

    assert_eq!(cookie_data.sessions.len(), 1);

    // ----- Create a number more classrooms ----- //

    for i in 2..5 {
        let classroom_add_data = ClassroomAddData {
            name: format!("Classroom{i}"),
            description: String::new(),
        };

        let response = server.post("/classrooms").json(&classroom_add_data).await;

        response.assert_status_ok();
        let classroom_created_data = response.json::<ClassroomGeneratedData>();

        let classroom_login_data = ClassroomUserLoginData {
            join_code: classroom_created_data.join_code,
            username: "Gelicenseerd radiozendamateur".to_owned(),
        };

        let response = server
            .post("/classrooms/auth")
            .json(&classroom_login_data)
            .await;

        response.assert_status_ok();

        let cookie = response.cookie("classroom_user_infos");
        let cookie_data = get_cookie_data::<AllClassroomSessions>(&cookie);

        assert_eq!(cookie_data.sessions.len(), i);
    }
}

#[sqlx::test]
async fn test_get_classrooms_teacher(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server.post("/classrooms").json(&classroom_add_data).await;
    response.assert_status_ok();

    let response2 = server.get("/classrooms/teacher/list").await;
    response2.assert_status_ok();

    let classroom_list: ClassroomListResponse = response2.json();

    assert_eq!(classroom_list.total_classrooms, 1);
}

#[sqlx::test]
async fn test_get_classrooms_teacher_unauthorized(db: PgPool) {
    let server = setup(db).server;

    let response = server.get("/classrooms/teacher/list").await;
    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_get_classrooms_student(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server.post("/classrooms").json(&classroom_add_data).await;
    response.assert_status_ok();
    let classroom_generated_data = response.json::<ClassroomGeneratedData>();

    let classroom_login = ClassroomUserLoginData {
        join_code: classroom_generated_data.join_code,
        username: "Gelicenseerd radiozendamateur".to_owned(),
    };

    let response2 = server.post("/classrooms/auth").json(&classroom_login).await;
    response2.assert_status_ok();

    let response3 = server.get("/classrooms/student/list").await;
    response3.assert_status_ok();

    let classroom_list: ClassroomListResponse = response3.json();

    assert_eq!(classroom_list.total_classrooms, 1);
}

#[sqlx::test]
async fn test_get_classrooms_student_unauthorized(db: PgPool) {
    let server = setup(db).server;

    let response = server.get("/classrooms/student/list").await;
    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_get_classroom_dashboard_teacher(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };
    let response = server.post("/classrooms").json(&classroom_add_data).await;
    response.assert_status_ok();
    let classroom_generated_data = response.json::<ClassroomGeneratedData>();

    let response = server
        .get(&format!(
            "/classrooms/{0}/teacher/dashboard",
            classroom_generated_data.id
        ))
        .await;
    response.assert_status_ok();
}

#[sqlx::test]
async fn test_get_classroom_dashboard_teacher_unauthorized(db: PgPool) {
    let server = setup(db).server;

    let response = server.get("/classrooms/0/teacher/dashboard").await;
    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_get_classroom_dashboard_student(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };
    let response = server.post("/classrooms").json(&classroom_add_data).await;
    response.assert_status_ok();
    let classroom_generated_data = response.json::<ClassroomGeneratedData>();

    let classroom_login = ClassroomUserLoginData {
        join_code: classroom_generated_data.join_code,
        username: "Gelicenseerd radiozendamateur".to_owned(),
    };

    let response = server.post("/classrooms/auth").json(&classroom_login).await;
    response.assert_status_ok();

    let response = server
        .get(&format!(
            "/classrooms/{0}/student/dashboard",
            classroom_generated_data.id
        ))
        .await;
    response.assert_status_ok();
}

#[sqlx::test]
async fn test_get_classroom_dashboard_student_unauthorized(db: PgPool) {
    let server = setup(db).server;

    let response = server.get("/classrooms/0/student/dashboard").await;
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

    let session = Session {
        user_id: 1,
        account_type: AccountType::Author,
    };

    let result =
        verify_classroom_permissions(get_connection_unwrap!(db), classroom_id, &session).await;
    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_verify_classroom_permission_admin(db: PgPool) {
    let server = setup_login(db.clone()).await;
    let classroom_id: i32 = create_classroom(&server).await;

    let session = Session {
        user_id: 1,
        account_type: AccountType::Admin,
    };

    let result =
        verify_classroom_permissions(get_connection_unwrap!(db), classroom_id, &session).await;
    assert!(result.is_ok());
}
