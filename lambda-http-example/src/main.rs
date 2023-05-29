use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_ansi(false)
        .without_time()
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name));

    run(app).await
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

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({
        "message": format!("I am POST /foo/:name, name={name}")
    }))
}
