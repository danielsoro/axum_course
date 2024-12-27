use axum_course::server;

#[tokio::main]
async fn main() {
    env_logger::init();
    server::run().await
}
