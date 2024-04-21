use uuid::Uuid;
use serde_json::json;
use axum::{
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};

pub async fn response_mapper(uri: Uri, method: Method, response: Response) -> Response {
    tracing_fast_dev::tfd().info("RESPONSE_MAPPER", "MIDDLEWARE");

    let err_option: Option<&String> = response.extensions().get::<String>();
    if err_option.is_none() {
        return response;
    };

    let is_server_error = response.status().eq(&StatusCode::INTERNAL_SERVER_ERROR);

    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string();

    let error_text = err_option.unwrap();

    let client_error_text = if is_server_error {
        "Something went wrong"
    } else {
        error_text
    };

    let client_response_error = json!({
        "uuid": uuid_string,
        "error": client_error_text
    });

    let server_log = json!({
        "uuid": uuid_string,
        "method": method.as_str(),
        "uri": uri.to_string(),
        "error":error_text
    });

    dbg!(server_log); // TODO: Send to logging service (Sentry) maybe?

    (response.status(), Json(client_response_error)).into_response()
}
