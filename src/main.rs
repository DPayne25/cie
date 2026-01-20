mod ingestion;
mod db;
use futures::future::join_all;
use std::path::{PathBuf};
use chrono::{Utc, TimeZone};
use crate::ingestion::errors::IngestionError;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _pool = db::connection::connect_db().await?;

    println!("• Database connected and schema synced!");

    let fx_pairs  = vec![
        "EUR_USD", 
        "USD_JPY", 
        "GBP_USD", 
        "AUD_USD", 
        "USD_CAD", 
        "USD_CHF", 
        "NZD_USD"];

    let client = reqwest::Client::new();

    let raw_cot_temp_path = PathBuf::from("data/raw/cot/latest_cot_raw.csv");
    let processed_cot_temp_path = PathBuf::from("data/processed/cot/latest_cot_processed.csv");

    ingestion::fetch::fetch_cot_data(&raw_cot_temp_path).await.map_err(|e| e.into())?;

    let cot_date = ingestion::csv_logic::extract_cot_date(&raw_cot_temp_path).map_err(|e| e.into())?;

    let final_raw_cot_path = PathBuf::from(format!("data/raw/cot/{}-RawCOTReport.csv", cot_date));
    let final_processed_cot_path = PathBuf::from(format!("data/processed/cot/{}-ProcessedCOTReport.csv", cot_date));

    ingestion::csv_logic::csv_validate(&raw_cot_temp_path).map_err(|e| e.into())?;

    ingestion::csv_logic::csv_process_raw_cot(&raw_cot_temp_path, &processed_cot_temp_path).map_err(|e| e.into())?;

    ingestion::csv_logic::finalize_cot_filenames(
        &raw_cot_temp_path,
        &final_raw_cot_path, 
        &processed_cot_temp_path, 
        &final_processed_cot_path
    ).map_err(|e| e.into())?;

    println!("• COT Report Ingestion Complete.");

    println!("• Starting FX Price Data Ingestion...");

    let fx_from_date = Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).earliest()
        .ok_or_else(|| IngestionError::Config("• Failed to create fixed 'from date'".to_string()))
        .map_err(|e| e.into())?;

    let fetch_all_fx_data_futures = fx_pairs.iter().map(|pair| {
        ingestion::fetch::fetch_fx_price_data(
            &client, 
            pair, 
            ingestion::fetch::CandlestickGranularity::D, 
            ingestion::fetch::PriceType::Mid, 
            fx_from_date)
    });

    let fx_fetch_results: Vec<Results<(), IngestionError>> =
    join_all(fetch_all_fx_data_futures).await;

    for result in fx_fetch_results {
        if let Err(e) = result {
            eprintln!("• Error fetching FX data: {}", e);
            return Err(e.into());
        }
    }

    println!("• FX Price Data Ingestion Complete.");

    println!("• Mission Complete: All Ingestion Tasks Completed Successfully.");


    Ok(())
} 