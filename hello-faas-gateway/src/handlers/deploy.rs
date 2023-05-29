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

    while let Some(field) = multipart
        .next_field()
        .await
        .or_bad_request("Failed to parse multipart")?
    {
        let bytes = field
            .bytes()
            .await
            .or_bad_request("Failed to parse multipart")?;

        tracing::info!(?bytes, "Received binary");
    }

    let uuid = uuid::Uuid::new_v4().to_string();
    let function = state
        .function_repository
        .create(&uuid)
        .await
        .or_internal_error("Failed to deploy new function")?;

    state
        .container_repository
        .create_container(&state.base_image, &function.id)
        .await
        .or_internal_error("Failed to create container")?;

    Ok(Json(
        json!({ "message": "Function deployed!", "id": function.id }),
    ))
}
