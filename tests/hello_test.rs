use axum::{
    body::{Body, Bytes},
    http::{self, Request, StatusCode},
};
use axum_course::hello;
use http_body_util::BodyExt;
use serde_json;
use tower::ServiceExt;

#[tokio::test]
async fn hello() {
    let hello_routes = hello::routes();
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
    let hello_routes = hello::routes();
    let response = hello_routes
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["message"], "Hello World");
    assert!(json["date"].is_string());
}
