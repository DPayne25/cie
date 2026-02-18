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
  
    let cot_fetch_tasks = (start_year..=current_year).map(|year| {
        println!("--- Syncing task for COT data for year: {} ---", year);

        ingestion::fetch::fetch_cot_data(year)
    });

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