mod ingestion;
use futures::future::join_all;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let fx_pairs  = vec![
        "EUR_USD", 
        //"USD_JPY", 
        "GBP_USD", 
        "AUD_USD", 
        //"USD_CAD", 
        //"USD_CHF", 
        "NZD_USD",
        //"GBP_AUD",
        //"GBP_NZD",
        "EUR_AUD",
        "EUR_JPY",
        //"GBP_JPY",
        //"CAD_JPY"
        ];

    let client = reqwest::Client::new();
  
    let fetch_d_fx_data = fx_pairs.iter().map(|pair| {
        ingestion::fetch_fxprice::fetch_fx_price_data(&client, 
            pair,  
            "M")
    });

    println!("• Fetching D FX price data from: OANDA");
    
    join_all(fetch_d_fx_data).await;

    let _fetch_h1_fx_data = fx_pairs.iter().map(|pair| {
        ingestion::fetch_fxprice::fetch_fx_price_data(&client, 
            pair,
            "M")
    });

    join_all(_fetch_h1_fx_data).await;

    println!("• Fetching COT data from: www.cftc.gov");

    for year in 2021..=2026 {

        ingestion::fetch_cot::fetch_cot_data(year).await?;

        ingestion::csv_logic::csv_validate()?;  

        ingestion::csv_logic::csv_process_raw_cot()?; 
        
        ingestion::csv_logic::rename_csv(year)?;
    }


   println!("• Mission Complete: Ingestion Completed Successfully.");


    Ok(())
} 