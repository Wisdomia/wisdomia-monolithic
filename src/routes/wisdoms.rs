use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};

use crate::{db::_get_wisdoms, helpers::default_handle_error, state::AppState};

use serde_json::json;

pub fn routes() -> Router {
    Router::new().nest("/wisdoms", _routes())
}

fn _routes() -> Router {
    Router::new()
        .route("/", get(get_wisdoms))
}


async fn get_wisdoms(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    tracing_fast_dev::tfd().info("GET_WISDOM", "FUNCTION");
    match _get_wisdoms(&state.db).await {
        Ok(wisdoms) => (StatusCode::OK, Json(json!({ "wisdoms": wisdoms }))).into_response(),
        Err(e) => default_handle_error(e),
    }
}