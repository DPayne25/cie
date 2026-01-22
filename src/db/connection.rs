use std::{env, error::Error, path::Path};
use sqlx::PgPool;



pub async fn connect_db(config: &SentinelConfig) -> Result<PgPool, Box<dyn Error>> {
    
    let db_pool: PgPool = sqlx::postgres::PgPoolOptions::new().connect(&config.database_url).await?;
    
    sqlx::migrate!("./sql/schema").run(&db_pool).await?;

    Ok(db_pool)
} 