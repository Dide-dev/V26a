#![cfg(test)]

use crate::api_v1::demo::JsonTestStruct;
use axum::http::{Request, Method};
use axum::body;
use sqlx::PgPool;
use tower::Service;

#[sqlx::test]
async fn test_counter_increment(db: PgPool) {
    let mut api = crate::tests::api_v1::setup(db);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/demo/json")
        .body(String::new())
        .unwrap();
    let response = api.call(request).await.unwrap();
    assert_eq!(response.status(), 200);
    let before_inc: JsonTestStruct = serde_json::from_slice(&body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    let request = Request::builder()
        .method(Method::GET)
        .uri("/demo/increment")
        .body(String::new())
        .unwrap();
    let response = api.call(request).await.unwrap();
    assert_eq!(response.status(), 200);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/demo/json")
        .body(String::new())
        .unwrap();
    let response = api.call(request).await.unwrap();
    assert_eq!(response.status(), 200);
    let after_inc: JsonTestStruct = serde_json::from_slice(&body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(after_inc.b, before_inc.b + 1);
}

#[sqlx::test]
async fn test_counter_add(db: PgPool) {
    let mut api = crate::tests::api_v1::setup(db);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/demo/json")
        .body(String::new())
        .unwrap();
    let response = api.call(request).await.unwrap();
    assert_eq!(response.status(), 200);
    let before_add: JsonTestStruct = serde_json::from_slice(&body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/demo/add-to-counter")
        .header("Content-Type", "application/json")
        .body(r#"{"add": 100}"#.to_string())
        .unwrap();
    let response = api.call(request).await.unwrap();
    assert_eq!(response.status(), 200);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/demo/json")
        .body(String::new())
        .unwrap();
    let response = api.call(request).await.unwrap();
    assert_eq!(response.status(), 200);
    let after_add: JsonTestStruct = serde_json::from_slice(&body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(after_add.b, before_add.b + 100);
}
