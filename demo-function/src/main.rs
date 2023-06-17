use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router, http::StatusCode,
};
use hello_faas::Error;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_target(false).init();

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo))
        .route("/foo", post(post_foo))
        .route("/foo/:name", post(post_foo_name));

    hello_faas::run(app).await
}

async fn root() -> Json<Value> {
    Json(json!({ "message": "I am GET /" }))
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "message": "I am GET /foo" }))
}

async fn post_foo() -> Json<Value> {
    Json(json!({ "message": "I am POST /foo" }))
}

async fn post_foo_name(Path(name): Path<String>) -> (StatusCode, Json<Value>) {
    if !name.eq_ignore_ascii_case("nscap") {
        return (StatusCode::UNAUTHORIZED, Json(json!({
            "message": "NO! You are not nscap!"
        })))
    }

    (StatusCode::OK, Json(json!({
        "message": format!("I am POST /foo/:name, name={name}")
    })))
}
