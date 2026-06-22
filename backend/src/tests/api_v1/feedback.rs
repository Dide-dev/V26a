/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::get_connection_unwrap;
use crate::structs::feedback::{FeedbackData, FeedbackListResponse};
use crate::structs::users::RegisterData;
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
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    register_admin(&test_state, conn, &register_data).await;
    let server = test_state.server;
    login(&server, register_data.email).await;

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
    let test_state = setup(db);

    let register_data = RegisterData::test_values();
    register_admin(&test_state, conn, &register_data).await;
    let server = test_state.server;
    login(&server, register_data.email).await;

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
