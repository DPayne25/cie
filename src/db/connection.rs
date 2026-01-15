use std::env;
use sqlx::{Pool, Postgres};

pub async fn connect_db() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    dotenvy::from_filename("./config/database.env").ok();
    let db_url: String = env::var("DATABASE_URL")?;
    let db_pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new().connect(&db_url).await?;
    sqlx::migrate!("./sql/schema").run(&db_pool).await?;

    Ok(db_pool)
} 