use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::Response,
    Json,
};
use hyper::Client;
use serde_json::Value;

use super::AppState;
use crate::traits::{FunctionNotFound, InternalServerError};

pub async fn invoke(
    State(state): State<AppState>,
    request: Request<Body>,
) -> Result<Response<Body>, (StatusCode, Json<Value>)> {
    let (parts, body) = request.into_parts();

    let path_and_query = parts.uri.path_and_query().or_not_found()?;
    let path = path_and_query.path();
    let query = path_and_query.query();

    let function_id = path.split('/').nth(1).or_not_found()?;
    let paths = path
        .strip_prefix(&format!("/invoke/{}", function_id))
        .unwrap_or("/");

    let function_container = state
        .container_repository
        .find_by_function_id(function_id)
        .await;

    let container_id = match function_container.first() {
        Some(container) => container.id.clone(),
        None => {
            state
                .container_repository
                .create_container(&state.base_image, function_id)
                .await
                .or_internal_error("Failed to create container")?
                .id
        }
    };

    if !function_container
        .first()
        .map(|c| c.status == "running")
        .unwrap_or(false)
    {
        state
            .container_repository
            .start_container(&container_id)
            .await
            .or_internal_error("Failed to start container")?;
    }

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
        .or_internal_error("Failed to build request")?;

    let client = Client::builder().build_http();
    client
        .request(req)
        .await
        .or_internal_error("Failed to forward request")
}
