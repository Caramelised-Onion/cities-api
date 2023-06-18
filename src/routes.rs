use axum::extract::State;
use sqlx::{Row, PgPool};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn get_random_city(State(pool): State<PgPool>) -> String {
    let v: String = sqlx::query("SELECT city FROM cities ORDER BY RANDOM() LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap()
        .try_get("city")
        .unwrap();
    format!("{} is my city", v)
}