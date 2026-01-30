#![allow(unused_imports)]
mod ingestion;
mod db;
mod crate::config;
use futures::future::join_all;
use sqlx::PgPool;
use chrono::Utc;
use chrono::Datelike;
use std::error::Error;
use std::path::Path;
use tokio;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    let config = config::SentinelConfig::from_env()?;

    let db_pool = db::connection::connect_db(&config).await?;

    let client = reqwest::Client::new();

    let current_year: i32 = chrono::Utc::now().year();

//==================================================
// Fetch COT Data
//==================================================
    let start_year: i32 = sqlx::query_scalar!(
        "SELECT COALESCE(MAX(EXTRACT(YEAR FROM report_date)), $1) FROM raw_cot_reports;", config.default_start_date.year()
    )
    .fetch_one(&db_pool)
    .await?;

    for year in start_year..=current_year {
        println!("--- Syncing Year: {} ---", year);
        ingestion::fetch::fetch_cot_data(year, &db_pool).await?;
    }

//==================================================
// Fetch FX Price Data
//==================================================
    let fx_pairs  = vec![
        "EUR_USD", 
        "USD_JPY", 
        "GBP_USD", 
        "AUD_USD", 
        "USD_CAD", 
        "USD_CHF", 
        "NZD_USD"];

    let fetch_all_fx_data = fx_pairs.iter().map(|pair| {
        ingestion::fetch::fetch_fx_price_data(
            &client,
            &config,
            db_pool.clone(), 
            pair, 
            "D", 
            "M"
        )
    });
    
    join_all(fetch_all_fx_data).await;  
    
    

   println!("₯₯₯ Mission Complete: Ingestion Completed Successfully. ₯₯₯");


    Ok(())
} 