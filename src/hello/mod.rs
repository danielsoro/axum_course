use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Json, Router};
use log::info;
use std::collections::HashMap;
use std::sync::Arc;

use crate::server::AppState;

async fn hello() -> impl IntoResponse {
    info!("Calling hello route");
    Html("<strong>Hello World</strong>")
}

async fn hello_json(
    State(app_state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    info!("Calling json route");
    Json(HashMap::from([
        ("message", format!("Hello {}", name)),
        ("date", chrono::Utc::now().to_rfc3339()),
        ("state", app_state.message.to_string()),
    ]))
}
pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello/:name", get(hello_json))
        .with_state(app_state)
}
