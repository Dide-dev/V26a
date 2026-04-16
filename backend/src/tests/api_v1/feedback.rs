/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::db_structs::feedback::FeedbackData;
use crate::tests::api_v1::setup_login;
use sqlx::PgPool;

#[sqlx::test]
async fn test_add_feedback(db: PgPool) {
    let server = setup_login(db).await;

    let feedback_data = FeedbackData {
        email: Some("test@test.nl".to_owned()),
        positive: Some("Tops".to_owned()),
        negative: Some("Tips".to_owned()),
        additional: Some("Very cool".to_owned()),
        grade: Some(7),
    };

    let response = server.post("/feedback/provide").json(&feedback_data).await;
    response.assert_status_ok();
}
