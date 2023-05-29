use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::traits::{BadRequest, InternalServerError};

use super::AppState;

/// Deploy a new function
pub async fn deploy(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    tracing::info!(?multipart, "Received multipart");
    let mut binary_content = vec![];

    while let Some(field) = multipart
        .next_field()
        .await
        .or_bad_request("Failed to parse multipart")?
    {
        let bytes = field
            .bytes()
            .await
            .or_bad_request("Failed to parse multipart")?;

        binary_content.extend_from_slice(&bytes);
    }

    tracing::info!("Received binary content length: {}", binary_content.len());

    let uuid = uuid::Uuid::new_v4().to_string();

    let binary_path = state
        .binary_repository
        .create(&uuid, &binary_content)
        .await
        .or_internal_error("Failed to deploy new function")?;

    tracing::info!(?binary_path, "Binary path");

    let function = state
        .function_repository
        .create(&uuid)
        .await
        .or_internal_error("Failed to deploy new function")?;

    let function_port = {
        let mut port_counter = state.port_counter.lock().await;
        *port_counter += 1;
        *port_counter
    };

    let container = state
        .container_repository
        .create_container(
            &state.base_image,
            &function.id,
            function_port,
            binary_path.to_str().unwrap_or_default(),
        )
        .await
        .or_internal_error("Failed to create container")?;

    state
        .function_repository
        .update(
            &function.id,
            Some(&container.id),
            Some(function_port.into()),
        )
        .await
        .or_internal_error("Failed to update function")?;

    Ok(Json(
        json!({ "message": "Function deployed!", "id": function.id }),
    ))
}
