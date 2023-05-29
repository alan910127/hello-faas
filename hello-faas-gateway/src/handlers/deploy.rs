use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use super::AppState;

/// Deploy a new function
pub async fn deploy(
    State(state): State<AppState>,
    mut binary: Multipart,
) -> Result<Json<Value>, (StatusCode, String)> {
    while let Some(field) = binary
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        let bytes = field
            .bytes()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

        tracing::info!(?bytes, "Received binary");
    }

    let uuid = uuid::Uuid::new_v4().to_string();
    let function = state
        .function_repository
        .create(&uuid)
        .await
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create function".to_string(),
            )
        })?;

    Ok(Json(
        json!({ "message": "Function deployed!", "id": function.id }),
    ))
}
