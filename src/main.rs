mod ingestion;
use anyhow::{Context, Error};
use chrono::{Datelike, NaiveDate, Utc};
use dotenvy::dotenv;
use futures::future::join_all;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").context("DATABASE_URL not set in .env")?;
    let db_pool = PgPool::connect(&db_url).await?;

    let client = reqwest::Client::new();
    let broker_key = env::var("OANDA_API_KEY").context("OANDA_API_KEY not set in .env file")?;

    let last: Option<NaiveDate> = sqlx::query_scalar!("SELECT MAX(report_date) FROM cot_tff")
        .fetch_one(&db_pool)
        .await?;

    let start_year = last.map(|d| d.year()).unwrap_or(2016);

    for year in start_year..=Utc::now().year() {
        ingestion::fetch_cot::fetch_cot_data(&year, &db_pool).await?;
    }

    let pairs = vec![
        "EUR_USD", "USD_JPY", "GBP_USD", "AUD_USD", "USD_CAD", "USD_CHF", "NZD_USD", "GBP_AUD",
        "GBP_NZD", "EUR_AUD", "EUR_JPY", "GBP_JPY", "CAD_JPY",
    ];

    let futures = pairs.iter().map(|pair| {
        ingestion::fetch_fxprice::fetch_fx_price_data(&client, "M", pair, &broker_key, &db_pool)
    });

    for result in join_all(futures).await {
        if let Err(e) = result {
            eprintln!("{e:#}");
        }
    }
    Ok(())
}
