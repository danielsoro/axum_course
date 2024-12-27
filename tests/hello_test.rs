use axum::{
    body::{Body, Bytes},
    http::{self, Request, StatusCode},
};
use axum_course::routes;
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn hello() {
    let hello_routes = routes::hello::routes();
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
