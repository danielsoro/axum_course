use axum::{
    response::Html,
    routing::{get, Router},
};

pub fn routes() -> Router {
    Router::new().route(
        "/hello",
        get(|| async { Html("<strong>Hello World</strong>") }),
    )
}
