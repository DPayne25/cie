mod ingestion;
//use chrono::{DateTime, Utc};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    //let start_date = "2016-01-01T15:00:00.000000000Z".parse::<DateTime<Utc>>().unwrap();
  
    ingestion::fetch::fetch_cot_data().await?;

    ingestion::fetch::fetch_fx_price_data(&client, "EUR_USD", "D", "M").await?;

    ingestion::csv_logic::csv_validate()?;  

    ingestion::csv_logic::csv_process_raw_cot()?; 
    
    ingestion::csv_logic::rename_csv()?;

   println!("✅ Mission Complete: Ingestion Completed Successfully.");


    Ok(())
} 