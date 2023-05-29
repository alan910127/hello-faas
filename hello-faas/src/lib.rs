use axum::{Router, Server};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to start server: {0}")]
    Server(#[from] hyper::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub async fn run(router: Router) -> Result<(), Error>
where
{
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .map_err(Error::Server)
}
