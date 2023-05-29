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

    let function_id = path.split('/').nth(2).or_not_found()?;
    let paths = path
        .strip_prefix(&format!("/invoke/{}", function_id))
        .unwrap_or("/");
    tracing::info!(?paths, ?query, ?function_id, "Invoke function");

    let function_container = state
        .container_repository
        .find_by_function_id(function_id)
        .await;
    tracing::info!(?function_container, "Found function container");

    let container_id = function_container
        .first()
        .map(|c| c.id.clone())
        .or_not_found()?;

    if !function_container
        .first()
        .map(|c| c.state == "running")
        .unwrap_or(false)
    {
        state
            .container_repository
            .start_container(&container_id)
            .await
            .or_internal_error("Failed to start container")?;
    }

    let function = state
        .function_repository
        .find_by_id(function_id)
        .await
        .or_not_found()?;

    let uri = match query {
        Some(q) => format!(
            "http://localhost:{}{}?{}",
            function.exposed_port.unwrap_or(8080),
            paths,
            q
        ),
        None => format!(
            "http://localhost:{}{}",
            function.exposed_port.unwrap_or(8080),
            paths
        ),
    };
    tracing::info!(?uri, "Forwarding request");

    let mut builder = Request::builder().method(parts.method).uri(uri);

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
