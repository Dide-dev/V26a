#![cfg(test)]

use crate::api_v1::demo::{AddCounterBody, JsonTestStruct};
use sqlx::PgPool;

#[sqlx::test]
async fn test_counter_increment(db: PgPool) {
    let server = crate::tests::api_v1::setup(db);

    let response = server.get("/demo/json").await;
    response.assert_status_ok();
    let before_inc: JsonTestStruct = response.json();

    let response = server.get("/demo/increment").await;
    response.assert_status_ok();

    let response = server.get("/demo/json").await;
    response.assert_status_ok();
    let after_inc: JsonTestStruct = response.json();

    assert_eq!(after_inc.b, before_inc.b + 1);
}

#[sqlx::test]
async fn test_counter_add(db: PgPool) {
    let server = crate::tests::api_v1::setup(db);

    let response = server.get("/demo/json").await;
    response.assert_status_ok();
    let before_inc: JsonTestStruct = response.json();

    let response = server.post("/demo/add-to-counter").json(&AddCounterBody {
        add: 100,
    }).await;
    response.assert_status_ok();

    let response = server.get("/demo/json").await;
    response.assert_status_ok();
    let after_inc: JsonTestStruct = response.json();

    assert_eq!(after_inc.b, before_inc.b + 100);
}
