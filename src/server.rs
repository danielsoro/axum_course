use std::sync::Arc;

use axum::Router;
use log::debug;
use tokio::net::TcpListener;

use crate::hello;

pub struct AppState {
    pub message: &'static str,
}

pub async fn run() {
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3030".into());
    let server_url = format!("{}:{}", "0.0.0.0", port);
    let listener = TcpListener::bind(server_url).await.unwrap();

    debug!("listening on {}", listener.local_addr().unwrap());

    let app_sate = Arc::new(AppState {
        message: "My State",
    });
    let routes = Router::new().merge(hello::routes(app_sate.clone()));

    axum::serve(listener, routes).await.unwrap()
}
