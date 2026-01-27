use std::{env, error::Error, path::Path};
use sqlx::PgPool;



pub async fn connect_db(config: &SentinelConfig) -> Result<PgPool, Box<dyn Error>> {
    
    let db_pool: PgPool = sqlx::postgres::PgPoolOptions::new().connect(&config.database_url).await?;
    
    sqlx::migrate!("./sql/schema").run(&db_pool).await?;

    Ok(db_pool)
} 

pub async fn get_latest_timestamp(config: &SentinelConfig, pool: &PgPool, symbol: &str) -> Result<Option<DateTime<Utc>>, Box<dyn Error>> {
    
    let latest_date: Option<DateTime<Utc>> = sqlx::query_scalar!(
        "SELECT MAX(date) FROM raw_fx_prices WHERE symbol = $1", symbol
    )
    .fetch_optional(pool)
    .await?;

    Ok(latest_date.unwrap_or(dt))
}