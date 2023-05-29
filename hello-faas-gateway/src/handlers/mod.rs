use std::sync::Arc;

use crate::repositories::{BinaryRepository, ContainerRepository, FunctionRepository};
use axum::Json;
use serde_json::{json, Value};

pub mod deploy;
pub mod invoke;

pub use deploy::deploy;
pub use invoke::invoke;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    function_repository: Arc<FunctionRepository>,
    container_repository: Arc<ContainerRepository>,
    binary_repository: Arc<BinaryRepository>,
    base_image: String,
    port_counter: Arc<Mutex<u16>>,
}

impl AppState {
    pub fn new(
        function_repository: Arc<FunctionRepository>,
        container_repository: Arc<ContainerRepository>,
        binary_repository: Arc<BinaryRepository>,
        base_image: String,
    ) -> Self {
        Self {
            function_repository,
            container_repository,
            binary_repository,
            base_image,
            port_counter: Arc::new(Mutex::new(8081)),
        }
    }
}

pub async fn root() -> Json<Value> {
    Json(json!({ "message": "Server is running!" }))
}
