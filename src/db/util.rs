use sqlx::Pool;
use chrono::{DateTime, Utc};
use std::error::Error;

pub async fn get_latest_timestamp(pool: &Pool<sqlx::Postgres>, symbol: &str) -> Result<Option<DateTime<Utc>>, Box<dyn Error>> {
    let latest_date: Option<DateTime<Utc>> = sqlx::query_scalar!(
        "SELECT MAX(date) FROM raw_fx_prices WHERE symbol = $1;", symbol
    )
    .fetch_optional(pool)
    .await?;
    Ok(latest_date)
}