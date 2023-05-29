use std::sync::Arc;

use crate::repositories::FunctionRepository;
use axum::Json;
use serde_json::{json, Value};

pub mod deploy;
pub mod invoke;

pub use deploy::deploy;
pub use invoke::invoke;

#[derive(Debug, Clone)]
pub struct AppState {
    function_repository: Arc<FunctionRepository>,
}

impl AppState {
    pub fn new(function_repository: Arc<FunctionRepository>) -> Self {
        Self {
            function_repository,
        }
    }
}

pub async fn root() -> Json<Value> {
    Json(json!({ "message": "Server is running!" }))
}
