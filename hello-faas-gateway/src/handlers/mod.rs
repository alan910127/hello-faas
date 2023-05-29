use std::sync::Arc;

use crate::repositories::{ContainerRepository, FunctionRepository};
use axum::Json;
use serde_json::{json, Value};

pub mod deploy;
pub mod invoke;

pub use deploy::deploy;
pub use invoke::invoke;

#[derive(Clone)]
pub struct AppState {
    function_repository: Arc<FunctionRepository>,
    container_repository: Arc<ContainerRepository>,
    base_image: String,
}

impl AppState {
    pub fn new(
        function_repository: Arc<FunctionRepository>,
        container_repository: Arc<ContainerRepository>,
        base_image: String,
    ) -> Self {
        Self {
            function_repository,
            container_repository,
            base_image,
        }
    }
}

pub async fn root() -> Json<Value> {
    Json(json!({ "message": "Server is running!" }))
}
