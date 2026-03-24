use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;

async fn hello_world() -> impl IntoResponse {
    Json(json!({
        "status":"ok",
        "message":"Hello, World!"
    }))
}

pub fn create_app() -> Router {
    Router::new().route("/hello", get(hello_world))
}
