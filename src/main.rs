mod ingestion;
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
  
    ingestion::fetch::fetch_cot_data(2025).await?;

    let fetch_all_fx_data = fx_pairs.iter().map(|pair| {
        ingestion::fetch::fetch_fx_price_data(&client, 
            pair, 
            "D", 
            "M")
    });
    
    join_all(fetch_all_fx_data).await;

    ingestion::csv_logic::csv_validate()?;  

    ingestion::csv_logic::csv_process_raw_cot()?; 
    
    ingestion::csv_logic::rename_csv()?;

   println!("• Mission Complete: Ingestion Completed Successfully.");


    Ok(())
} 