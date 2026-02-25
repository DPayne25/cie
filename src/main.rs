mod ingestion;
use std::path::Path;

use futures::future::join_all;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let fx_pairs  = vec![
        "EUR_USD", 
        "USD_JPY", 
        "GBP_USD", 
        "AUD_USD", 
        "USD_CAD", 
        "USD_CHF", 
        "NZD_USD"];

    let client = reqwest::Client::new();
  
    let fetch_all_fx_data = fx_pairs.iter().map(|pair| {
        ingestion::fetch_fxprice::fetch_fx_price_data(&client, 
            pair, 
            "D", 
            "M")
    });
    
    join_all(fetch_all_fx_data).await;

    for year in 2016..=2026 {

        if Path::new("data/raw/cot/{year}-RawCOTReport.csv").exists() {
          continue;
        } else {  

            ingestion::fetch_cot::fetch_cot_data(year).await?;

            ingestion::csv_logic::csv_validate()?;  

            ingestion::csv_logic::csv_process_raw_cot()?; 
            
            ingestion::csv_logic::rename_csv(year)?;
        }
    }


   println!("• Mission Complete: Ingestion Completed Successfully.");


    Ok(())
} 