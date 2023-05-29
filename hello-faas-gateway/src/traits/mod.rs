use axum::Json;
use hyper::StatusCode;
use serde_json::{json, Value};

pub trait FunctionNotFound<T> {
    fn or_not_found(self) -> Result<T, (StatusCode, Json<Value>)>;
}

impl<T> FunctionNotFound<T> for Option<T> {
    fn or_not_found(self) -> Result<T, (StatusCode, Json<Value>)> {
        self.ok_or((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Function not found" })),
        ))
    }
}

pub trait InternalServerError<T> {
    fn or_internal_error(self, message: &str) -> Result<T, (StatusCode, Json<Value>)>;
}

impl<T, E> InternalServerError<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn or_internal_error(self, message: &str) -> Result<T, (StatusCode, Json<Value>)> {
        self.map_err(|e| {
            tracing::error!(?e, message);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
        })
    }
}
