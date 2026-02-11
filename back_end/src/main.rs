use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .merge(static_frontend())
        .nest("/api", backend());

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn static_frontend() -> Router {
    use tower_http::services::ServeDir;
    let static_frontend_dir = ServeDir::new("../front_end/build");

    Router::new().fallback_service(static_frontend_dir)
}

fn backend() -> Router {
    Router::new()
        .route("/html", get(backend_html_test))
        .route("/json", get(backend_json_test))
}

async fn backend_html_test() -> Html<&'static str> {
    Html("<h1>Hello from axum!</h1>")
}

#[derive(serde::Serialize)]
struct JsonTestStruct {
    a: String,
    b: i32,
}

async fn backend_json_test() -> axum::Json<JsonTestStruct> {
    let test_struct = JsonTestStruct {
        a: "Hello, from rust struct!".to_owned(),
        b: 42,
    };

    axum::Json(test_struct)
}
