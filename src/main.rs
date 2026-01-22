mod ingestion;
use futures::future::join_all;
use sqlx::pool;
use std::{path::Path, error::Error};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let fx_pairs  = vec![
        "EUR_USD", 
        "USD_JPY", 
        "GBP_USD", 
        "AUD_USD", 
        "USD_CAD", 
        "USD_CHF", 
        "NZD_USD"];

    let config = config::SentinelConfig::from_env()?;

    let db_pool = db::connection::connect_db(&config).await?;

    let client = reqwest::Client::new();

  
    ingestion::fetch::fetch_cot_data().await?;
    let fetch_all_fx_data = fx_pairs.iter().map(|pair| {
        ingestion::fetch::fetch_fx_price_data(
            &client,
            &config,
            db_pool.clone(), 
            pair, 
            "D", 
            "M")
    });
    
    join_all(fetch_all_fx_data).await;

    ingestion::data_processor::process_cot_report()?;  
    
    

   println!("• Mission Complete: Ingestion Completed Successfully.");


    Ok(())
} 