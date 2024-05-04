use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/ping", get(pong))
}

async fn pong() -> impl IntoResponse {
    tracing_fast_dev::tfd().info("PONG", "FUNCTION");
    (StatusCode::OK, "pong")
}
