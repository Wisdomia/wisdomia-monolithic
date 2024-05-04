use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
pub fn default_handle_error<E: std::fmt::Display>(e: E) -> Response<Body> {
    let mut response = (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    response.extensions_mut().insert(e.to_string());
    response
}
