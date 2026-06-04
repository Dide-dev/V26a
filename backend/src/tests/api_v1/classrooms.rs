/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::classrooms::{
    ClassroomAddData, ClassroomGeneratedData, ClassroomLoginResponse, ClassroomUserLoginData,
};
use crate::db_structs::session::AllClassroomClaims;
use crate::tests::api_v1::{get_cookie_data, setup, setup_login};
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
    let server = setup(db);

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
        .post("/classrooms/login")
        .json(&classroom_login_data)
        .await;

    response.assert_status_ok();

    let classroom_login_response = response.json::<ClassroomLoginResponse>();

    let cookie = response.cookie("classroom_user_infos");
    let cookie_data = get_cookie_data::<AllClassroomClaims>(&cookie);

    assert_eq!(cookie_data.sessions.len(), 1);
    assert_eq!(cookie_data.sessions[0].id, classroom_created_data.id);
    assert_eq!(cookie_data.sessions[0].num, classroom_login_response.num);
    assert_eq!(cookie_data.sessions[0].name, classroom_login_response.name);

    // ----- Cookie with same classroom id present, new user ----- //

    let classroom_login_data2 = ClassroomUserLoginData {
        join_code: classroom_login_data.join_code,
        username: "Gelicenseerd radiozendamateur 2".to_owned(),
    };

    let response = server
        .post("/classrooms/login")
        .json(&classroom_login_data2)
        .await;

    response.assert_status_ok();

    let classroom_login_response2 = response.json::<ClassroomLoginResponse>();

    let cookie = response.cookie("classroom_user_infos");
    let cookie_data = get_cookie_data::<AllClassroomClaims>(&cookie);

    assert_eq!(cookie_data.sessions.len(), 1);
    assert_eq!(cookie_data.sessions[0].id, classroom_created_data.id);
    assert_eq!(cookie_data.sessions[0].num, classroom_login_response2.num);
    assert_eq!(cookie_data.sessions[0].name, classroom_login_response2.name);

    // ----- Cookie with same classroom id present, existing user ----- //

    let classroom_login_data = ClassroomUserLoginData {
        join_code: classroom_login_data2.join_code,
        username: classroom_login_response.num.to_string(),
    };

    let response = server
        .post("/classrooms/login")
        .json(&classroom_login_data)
        .await;

    response.assert_status_ok();

    let cookie = response.cookie("classroom_user_infos");
    let cookie_data = get_cookie_data::<AllClassroomClaims>(&cookie);

    assert_eq!(cookie_data.sessions.len(), 1);
    assert_eq!(cookie_data.sessions[0].id, classroom_created_data.id);
    assert_eq!(cookie_data.sessions[0].num, classroom_login_response.num);
    assert_eq!(cookie_data.sessions[0].name, classroom_login_response.name);
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
        .post("/classrooms/login")
        .json(&classroom_login_data)
        .await;

    response.assert_status_ok();

    let cookie = response.cookie("classroom_user_infos");
    let cookie_data = get_cookie_data::<AllClassroomClaims>(&cookie);

    assert_eq!(cookie_data.sessions.len(), 1);
    assert_eq!(cookie_data.sessions[0].id, classroom_created_data.id);

    // ----- Create a number more classrooms ----- //

    for i in 2..12 {
        let classroom_add_data = ClassroomAddData {
            name: format!("Classroom{i}"),
            description: String::new(),
        };

        let response = server
            .post("/classrooms/add")
            .json(&classroom_add_data)
            .await;

        response.assert_status_ok();
        let classroom_created_data = response.json::<ClassroomGeneratedData>();

        let classroom_login_data = ClassroomUserLoginData {
            join_code: classroom_created_data.join_code,
            username: "Gelicenseerd radiozendamateur".to_owned(),
        };

        let response = server
            .post("/classrooms/login")
            .json(&classroom_login_data)
            .await;

        response.assert_status_ok();

        let cookie = response.cookie("classroom_user_infos");
        let cookie_data = get_cookie_data::<AllClassroomClaims>(&cookie);

        assert_eq!(cookie_data.sessions.len(), i.min(10));
        assert_eq!(
            cookie_data.sessions.last().unwrap().id,
            classroom_created_data.id
        );
    }
}
