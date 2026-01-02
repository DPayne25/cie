mod ingestion;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    ingestion::fetch::fetch_cot_data().await?;

    ingestion::fetch::fetch_fx_price_data("EUR_USD", "2016-01-01T15%3A00%3A00.000000000Z", "D", "M").await?;

    ingestion::csv_logic::csv_validate()?;  

    ingestion::csv_logic::csv_process_raw_cot()?; 
    
    ingestion::csv_logic::rename_csv()?;

   println!("✅ Mission Complete: Ingestion Completed Successfully.");


    Ok(())
} 