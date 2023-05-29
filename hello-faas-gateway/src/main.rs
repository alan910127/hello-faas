use std::sync::Arc;

use hello_faas_gateway::{config::Config, prelude::*, repositories::FunctionRepository};

use axum::{routing::get, Json, Router, Server, ServiceExt};
use serde_json::{json, Value};
use shiplift::{Docker, RmContainerOptions};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = Config::load()?;
    tracing::info!(?config, "Loaded config");

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let function_repository = Arc::new(FunctionRepository::new(pool));
    tokio::spawn(idle_functions_cleanup_worker(function_repository.clone()));

    let app = Router::new().route("/", get(root));

    let app = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .service(app);

    tracing::info!("🚀 Server listening on http://localhost:3000");
    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await
        .with_context(|| "Failed to start server")
}

async fn root() -> Json<Value> {
    Json(json!({ "message": "Server is running!" }))
}

async fn idle_functions_cleanup_worker(function_repository: Arc<FunctionRepository>) -> Result<()> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    let docker = Docker::new();
    let containers = docker.containers();

    loop {
        interval.tick().await;

        let functions = match function_repository.find_idle().await {
            Ok(functions) => functions,
            Err(e) => {
                tracing::error!(?e, "Failed to find idle functions");
                continue;
            }
        };

        for function in functions {
            if let Some(container_id) = &function.container_id {
                let opts = RmContainerOptions::builder().force(true).build();
                if containers.get(container_id).remove(opts).await.is_ok() {
                    tracing::info!(?function, "Deleted idle function");
                    function_repository.update(&function.id, None).await;
                } else {
                    tracing::error!(?function, "Failed to delete idle function");
                }
            }
        }
    }
}
