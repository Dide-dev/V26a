use axum::response::Html;
use axum::Router;
use axum::routing::get;

pub fn demo() -> Router {
    Router::new()
        .route("/html", get(backend_html_test))
        .route("/json", get(backend_json_test))
}

#[derive(serde::Serialize)]
pub struct JsonTestStruct {
    a: String,
    b: i32,
}

pub async fn backend_json_test() -> axum::Json<JsonTestStruct> {
    let test_struct = JsonTestStruct {
        a: "Hello, from rust struct!".to_owned(),
        b: 42,
    };

    axum::Json(test_struct)
}

pub async fn backend_html_test() -> Html<&'static str> {
    Html("<h1>Hello from axum!</h1>")
}
