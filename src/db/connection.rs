use std::env;
use sqlx::PgPool;

pub async fn db_access() -> Result<String, Box<dyn std::error::Error>> {
    dotenvy::from_filename("./config/database.env").ok();
    let db_url: String = env::var("DATABASE_URL")?;
    
    Ok(db_url)
}

pub async fn connect_db() -> Result<PgPool, Box<dyn std::error::Error>> {
    let db_url = db_access().await?;
    let db_pool: PgPool = sqlx::postgres::PgPoolOptions::new().connect(&db_url).await?;
    sqlx::migrate!("./sql/schema").run(&db_pool).await?;

    Ok(db_pool)
} 