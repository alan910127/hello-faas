use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Json,
};
use hyper::Client;
use serde_json::{json, Value};

pub async fn invoke(request: Request<Body>) -> Result<Response<Body>, (StatusCode, Json<Value>)> {
    let (parts, body) = request.into_parts();

    let Some(path_and_query) = parts.uri.path_and_query() else {
        return Err((StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))));
    };
    let path = path_and_query.path();
    let query = path_and_query.query();

    let Some(function_id) = path.split('/').nth(1) else {
        return Err((StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))));
    };
    let paths = path
        .strip_prefix(&format!("/invoke/{}", function_id))
        .unwrap_or("/");

    let mut builder = Request::builder().method(parts.method).uri(format!(
        "http://localhost:8080{}?{}",
        paths,
        query.unwrap_or_default()
    ));

    for (key, value) in parts.headers {
        if let Some(key) = key {
            builder = builder.header(key, value);
        }
    }
    let req = builder
        .header("x-faas-function-id", function_id)
        .body(body)
        .map_err(|e| {
            tracing::error!(?e, "Failed to build request");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
        })?;

    let client = Client::builder().build_http();
    client.request(req).await.map_err(|e| {
        tracing::error!(?e, "Failed to forward request");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
    })
}
