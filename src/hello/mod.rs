use std::collections::HashMap;
use axum::{Json, Router};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use log::info;

async fn hello() -> impl IntoResponse {
    info!("Calling hello route");
    Html("<strong>Hello World</strong>")
}

async fn hello_json() -> impl IntoResponse {
    info!("Calling json route");
    Json(HashMap::from([
        ("message", "Hello World".to_string()),
        ("date", chrono::Utc::now().to_rfc3339()),
    ]))
}
pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/json", get(hello_json))
}