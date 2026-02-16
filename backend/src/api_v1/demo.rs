use crate::db::demo::{fetch_counter_value, increment_counter_value};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::{Extension, Router};
use sqlx::PgPool;

pub fn demo() -> Router {
    Router::new()
        .route("/html", get(backend_html_test))
        .route("/json", get(backend_json_test))
        .route("/increment", get(increment_counter_test))
}

async fn backend_html_test() -> Html<&'static str> {
    Html("<h1>Hello from axum!</h1>")
}

#[derive(serde::Serialize)]
pub struct JsonTestStruct {
    a: String,
    b: i32,
}

async fn backend_json_test(
    db: Extension<PgPool>,
) -> Result<axum::Json<JsonTestStruct>, StatusCode> {
    let counter_value = fetch_counter_value(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let test_struct = JsonTestStruct {
        a: "Hello, from rust struct!".to_owned(),
        b: counter_value,
    };

    Ok(axum::Json(test_struct))
}

async fn increment_counter_test(
    db: Extension<PgPool>,
) -> Result<axum::Json<JsonTestStruct>, StatusCode> {
    let counter_value = increment_counter_value(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let test_struct = JsonTestStruct {
        a: "Incremented counter!".to_owned(),
        b: counter_value,
    };

    Ok(axum::Json(test_struct))
}
