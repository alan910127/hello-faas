use sqlx::{types::time::PrimitiveDateTime, PgPool};

pub struct FunctionRepository {
    pool: PgPool,
}

pub struct DeployedFunction {
    pub id: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub invoked_at: PrimitiveDateTime,
    pub container_id: Option<String>,
}

impl FunctionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &str) -> Option<DeployedFunction> {
        sqlx::query_as!(
            DeployedFunction,
            "SELECT * FROM functions WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await
        .ok()
    }

    pub async fn create(&self, id: &str) -> Option<DeployedFunction> {
        sqlx::query_as!(
            DeployedFunction,
            "INSERT INTO functions (id) VALUES ($1) RETURNING *",
            id
        )
        .fetch_one(&self.pool)
        .await
        .ok()
    }

    pub async fn update(&self, id: &str, container_id: Option<&str>) -> Option<DeployedFunction> {
        sqlx::query_as!(
            DeployedFunction,
            "UPDATE functions SET container_id = $1 WHERE id = $2 RETURNING *",
            container_id,
            id
        )
        .fetch_one(&self.pool)
        .await
        .ok()
    }

    pub async fn set_invoked(&self, id: &str) -> Option<DeployedFunction> {
        sqlx::query_as!(
            DeployedFunction,
            "UPDATE functions SET invoked_at = NOW() WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(&self.pool)
        .await
        .ok()
    }
}
