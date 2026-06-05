/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::feedback::{FeedbackData, FeedbackListResponse};
use crate::db_structs::users::RegisterData;
use crate::get_connection_unwrap;
use crate::tests::api_v1::{login, register_admin, setup, setup_login};
use sqlx::PgPool;

#[sqlx::test]
async fn test_add_feedback(db: PgPool) {
    let server = setup_login(db).await;

    let feedback_data = FeedbackData {
        email: "test@test.nl".to_owned(),
        feedback: "feedback".to_owned(),
    };

    let response = server.post("/feedback").json(&feedback_data).await;
    response.assert_status_ok();
}

#[sqlx::test]
async fn test_get_feedback(db: PgPool) {
    let conn = get_connection_unwrap!(db);
    let server = setup(db);

    let admin_data = RegisterData::test_values();
    register_admin(&server, conn, &admin_data).await;
    login(&server, admin_data.email).await;

    let feedback_data = FeedbackData {
        email: "test@test.nl".to_owned(),
        feedback: "feedback".to_owned(),
    };

    let response = server.post("/feedback").json(&feedback_data).await;
    response.assert_status_ok();

    let response = server.get("/admin/feedback?page=1&size=1").await;
    response.assert_status_ok();

    let requested_feedback: FeedbackListResponse = response.json();
    assert_eq!(requested_feedback.feedback_total, 1);
    assert_eq!(requested_feedback.feedback.len(), 1);

    let feedback = &requested_feedback.feedback[0];
    assert_eq!(feedback.email, feedback_data.email);
    assert_eq!(feedback.feedback, feedback_data.feedback);
}

#[sqlx::test]
async fn test_get_no_feedback(db: PgPool) {
    let conn = get_connection_unwrap!(db);
    let server = setup(db);

    let admin_data = RegisterData::test_values();
    register_admin(&server, conn, &admin_data).await;
    login(&server, admin_data.email).await;

    let response = server.get("/admin/feedback?page=1&size=10").await;
    response.assert_status_ok();

    let requested_feedback: FeedbackListResponse = response.json();
    assert_eq!(requested_feedback.feedback_total, 0);
    assert!(requested_feedback.feedback.is_empty());
}

#[sqlx::test]
async fn test_get_feedback_not_admin(db: PgPool) {
    let server = setup_login(db).await;

    let feedback_data = FeedbackData {
        email: "test@test.nl".to_owned(),
        feedback: "feedback".to_owned(),
    };

    let response = server.post("/feedback").json(&feedback_data).await;
    response.assert_status_ok();

    let response = server.get("/admin/feedback?page=1&size=1").await;
    response.assert_status_forbidden();
}
