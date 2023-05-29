use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();
}
