use sqlx::postgres::PgPoolOptions;

pub async fn connect_to_db() -> sqlx::Pool<sqlx::Postgres> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    println!("Connected to the database");
    pool
}
