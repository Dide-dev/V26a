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

fn create_multipart_body(boundary: &str, bytes: &[u8], filename: &str) -> (String, Vec<u8>) {
    let mut body = Vec::new();

    body.extend(format!("--{boundary}\r\n").as_bytes());
    body.extend(
        format!("Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n")
            .as_bytes(),
    );
    body.extend(b"Content-Type: application/octet-stream\r\n\r\n");
    body.extend(bytes);
    body.extend(format!("\r\n--{boundary}--\r\n").as_bytes());

    let content_type = format!("multipart/form-data; boundary={boundary}");

    (content_type, body)
}

#[sqlx::test]
async fn test_upload_image_success(db: PgPool) {
    let config = crate::Config::load();
    let server = setup_login(db).await;

    let image_bytes = vec![0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];

    let boundary = "testboundary";
    let (content_type, body) = create_multipart_body(boundary, &image_bytes, "test.png");

    let response: TestResponse = server
        .post("/images/add")
        .add_header("content-type", &content_type)
        .bytes(Bytes::from(body))
        .await;

    response.assert_status_ok();

    let path: String = response.text();
    assert!(path.contains(&config.upload_dir));
}

#[sqlx::test]
async fn test_image_upload_no_file(db: PgPool) {
    let server = setup_login(db).await;

    let boundary = "testboundary";
    let body = format!("--{boundary}--\r\n").into_bytes();

    let response = server
        .post("/images/add")
        .add_header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .bytes(Bytes::from(body))
        .await;

    response.assert_status_internal_server_error();
}

#[sqlx::test]
async fn test_upload_file_too_large(db: PgPool) {
    let server = setup_login(db).await;

    let large_file = vec![0u8; 11 * 1024 * 1024]; // 11MB

    let boundary = "testboundary";
    let (content_type, body) = create_multipart_body(boundary, &large_file, "big.png");

    let response = server
        .post("/images/add")
        .add_header("content-type", &content_type)
        .bytes(Bytes::from(body))
        .await;

    response.assert_status_bad_request();
}

#[sqlx::test]
async fn test_upload_invalid_image_type(db: PgPool) {
    let server = setup_login(db).await;

    let text_bytes = b"this is not an image";

    let boundary = "testboundary";
    let (content_type, body) = create_multipart_body(boundary, text_bytes, "file.txt");

    let response = server
        .post("/images/add")
        .add_header("content-type", &content_type)
        .bytes(Bytes::from(body))
        .await;

    response.assert_status_bad_request();
}
#[sqlx::test]
async fn test_upload_duplicate_image(db: PgPool) {
    let server = setup_login(db).await;

    let image_bytes = vec![0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];

    let boundary = "testboundary";

    let (content_type, body) = create_multipart_body(boundary, &image_bytes, "test.png");

    let response1 = server
        .post("/images/add")
        .add_header("content-type", &content_type)
        .bytes(Bytes::from(body.clone()))
        .await;

    response1.assert_status_ok();
    let path1: String = response1.text();

    let response2 = server
        .post("/images/add")
        .add_header("content-type", &content_type)
        .bytes(Bytes::from(body))
        .await;

    response2.assert_status_ok();
    let path2: String = response2.text();

    assert_eq!(path1, path2);
}
