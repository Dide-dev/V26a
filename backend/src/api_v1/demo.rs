use axum::response::Html;
use axum::{Extension, Router};
use axum::routing::{get, post};
use sqlx::{PgPool};
use crate::db::demo::{fetch_counter_value, increment_counter_value};

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

async fn backend_json_test(db: Extension<PgPool>) -> axum::Json<JsonTestStruct> {
    let counter_value = fetch_counter_value(&db).await.unwrap(); // TODO: return error?

    let test_struct = JsonTestStruct {
        a: "Hello, from rust struct!".to_owned(),
        b: counter_value,
    };

    axum::Json(test_struct)
}

async fn increment_counter_test(db: Extension<PgPool>) -> axum::Json<JsonTestStruct> {
    let counter_value = increment_counter_value(&db).await.unwrap(); // TODO: return error?

    let test_struct = JsonTestStruct {
        a: "Incremented counter!".to_owned(),
        b: counter_value,
    };

    axum::Json(test_struct)
}
