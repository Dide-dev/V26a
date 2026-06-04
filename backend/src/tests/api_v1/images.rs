/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

#![cfg(test)]

use crate::tests::api_v1::setup_login;
use axum::body::Bytes;
use axum_test::TestResponse;
use sqlx::PgPool;

#[sqlx::test]
async fn test_upload_image_success(db: PgPool) {
    let server = setup_login(db).await;

    // PNG file signature: ‰PNG␍␊␚␊
    let image_bytes = vec![0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];

    let response: TestResponse = server
        .post("/images/add")
        .bytes(Bytes::from(image_bytes))
        .await;

    response.assert_status_ok();

    let path: String = response.text();
    assert!(path.starts_with("/uploads/"));
}

#[sqlx::test]
async fn test_image_upload_empty_body(db: PgPool) {
    let server = setup_login(db).await;

    let response = server.post("/images/add").bytes(Bytes::from(vec![])).await;

    response.assert_status_bad_request();
}

#[sqlx::test]
async fn test_upload_file_too_large(db: PgPool) {
    let server = setup_login(db).await;

    let large_file = vec![0u8; 11 * 1024 * 1024];

    let response = server
        .post("/images/add")
        .bytes(Bytes::from(large_file))
        .await;

    response.assert_status_payload_too_large();
}

#[sqlx::test]
async fn test_upload_invalid_image_type(db: PgPool) {
    let server = setup_login(db).await;

    let text_bytes = b"this is not an image";

    let response = server
        .post("/images/add")
        .bytes(Bytes::from(text_bytes.to_vec()))
        .await;

    response.assert_status_bad_request();
}
#[sqlx::test]
async fn test_upload_duplicate_image(db: PgPool) {
    let server = setup_login(db).await;

    // PNG file signature: ‰PNG␍␊␚␊
    let image_bytes = vec![0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];

    let response1 = server
        .post("/images/add")
        .bytes(Bytes::from(image_bytes.clone()))
        .await;

    response1.assert_status_ok();
    let path1: String = response1.text();

    let response2 = server
        .post("/images/add")
        .bytes(Bytes::from(image_bytes))
        .await;

    response2.assert_status_ok();
    let path2: String = response2.text();

    assert_eq!(path1, path2);
}
