/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::classrooms::{ClassroomAddData, ClassroomGeneratedData};
use crate::tests::api_v1::{setup, setup_login};
use sqlx::PgPool;

#[sqlx::test]
async fn test_new_classroom(db: PgPool) {
    let server = setup_login(db).await;

    let classroom_add_data = ClassroomAddData {
        name: "Classroom".to_owned(),
        description: String::new(),
    };

    let response = server
        .post("/classrooms")
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
        .post("/classrooms")
        .json(&classroom_add_data)
        .await;
    response.assert_status_unauthorized();
}
