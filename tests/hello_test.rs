use std::sync::Arc;

use axum::{
    body::{Body, Bytes},
    http::{self, Request, StatusCode},
};
use axum_course::{hello, server::AppState};
use http_body_util::BodyExt;
use serde_json;
use tower::ServiceExt;

#[tokio::test]
async fn hello() {
    let app_sate = Arc::new(AppState {
        message: "My State",
    });
    let hello_routes = hello::routes(app_sate);
    let response = hello_routes
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/hello")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, Bytes::from("<strong>Hello World</strong>"));
}

#[tokio::test]
async fn hello_json() {
    let app_sate = Arc::new(AppState {
        message: "My State",
    });
    let hello_routes = hello::routes(app_sate);
    let response = hello_routes
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/hello/Name")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["message"], "Hello Name");
    assert!(json["date"].is_string());
}
