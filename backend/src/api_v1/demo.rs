/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::api_v1::session::Claims;
use crate::db::demo::{add_counter_value, fetch_counter_value};
use crate::errors::ApiError;
use crate::{Config, get_connection};
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use sqlx::PgPool;

/// Creates an `Axum` [`Router`] for the demo category, combining its endpoints.
pub fn demo() -> Router<Config> {
    Router::new()
        .route("/html", get(backend_html_test))
        .route("/json", get(backend_json_test))
        .route("/increment", get(increment_counter_test))
        .route("/add-to-counter", post(add_counter_test))
        .route("/protected", get(protected_test))
}

/// A test endpoint that checks whether the user is logged in as user 1.
async fn protected_test(claims: Claims) -> Result<String, ApiError> {
    if claims.sub != 1 {
        Err(ApiError::NoPermission)?;
    }
    Ok("Welcome, user 1!".to_string())
}

/// A test endpoint that returns a basic HTML response.
async fn backend_html_test() -> Html<&'static str> {
    Html("<h1>Hello from axum!</h1>")
}

/// A test struct, containing some example fields to later be converted to JSON.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonTestStruct {
    pub a: String,
    pub b: i32,
}

/// A test endpoint that retrieves the value of a counter in the database.
///
/// # Returns
/// A [`Result`] containing the counter value in a [`JsonTestStruct`].
///
/// # Error
/// May return an [`ApiError`] instead.
async fn backend_json_test(
    Extension(db): Extension<PgPool>,
) -> Result<Json<JsonTestStruct>, ApiError> {
    let counter_value = fetch_counter_value(get_connection!(db)).await?;

    let test_struct = JsonTestStruct {
        a: "Hello, from rust struct!".to_owned(),
        b: counter_value,
    };

    Ok(Json(test_struct))
}

/// A test endpoint that increments the value of a counter in the database,
/// and returns the updated value as JSON.
///
/// # Returns
/// A [`Result`] containing the updated counter value in a [`JsonTestStruct`] if successful,
/// or an [`ApiError`] if not.
async fn increment_counter_test(
    Extension(db): Extension<PgPool>,
) -> Result<Json<JsonTestStruct>, ApiError> {
    let counter_value = add_counter_value(get_connection!(db), 1).await?;

    let test_struct = JsonTestStruct {
        a: "Incremented counter!".to_owned(),
        b: counter_value,
    };

    Ok(Json(test_struct))
}

/// A struct representing the body of an HTTP request to add to a counter.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddCounterBody {
    pub add: i32,
}

/// A test endpoint that adds a given value in the request body to a counter in the database.
/// This reads the given request body and tries to parse it as [`AddCounterBody`].
///
/// # Returns
/// Nothing if successful, or an [`ApiError`] if not.
async fn add_counter_test(
    Extension(db): Extension<PgPool>,
    Json(body): Json<AddCounterBody>,
) -> Result<(), ApiError> {
    add_counter_value(get_connection!(db), body.add).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use axum_test::TestServer;
    use sqlx::query;

    #[tokio::test]
    async fn test_backend_html() {
        let server = TestServer::new(Router::new().route("/html", get(backend_html_test)));
        let response = server.get("/html").await;
        response
            .assert_status_ok()
            .assert_text("<h1>Hello from axum!</h1>");
    }

    #[sqlx::test]
    async fn test_backend_json_default(db: PgPool) {
        let server = TestServer::new(
            Router::new()
                .route("/json", get(backend_json_test))
                .layer(Extension(db)),
        );
        let response = server.get("/json").await;
        response.assert_status_ok().assert_json(&JsonTestStruct {
            a: "Hello, from rust struct!".to_string(),
            b: 0,
        });
    }

    #[sqlx::test]
    async fn test_backend_json_value(db: PgPool) {
        query!("INSERT INTO counters VALUES (0, 67)")
            .execute(&db)
            .await
            .unwrap();

        let server = TestServer::new(
            Router::new()
                .route("/json", get(backend_json_test))
                .layer(Extension(db)),
        );
        let response = server.get("/json").await;
        response.assert_status_ok().assert_json(&JsonTestStruct {
            a: "Hello, from rust struct!".to_string(),
            b: 67,
        });
    }
}
