mod demo;

use crate::api_v1::demo::demo;
use axum::Router;

pub fn v1_api() -> Router {
    Router::new().nest("/demo", demo())
}
