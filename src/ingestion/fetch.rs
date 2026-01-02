use std::{fs, env};
use reqwest;
#[allow(unused_imports)]
use reqwest::{RequestBuilder, Response, Client, Request};
#[allow(unused_imports)]
use chrono;

pub async fn fetch_cot_data() -> Result<(), Box<dyn std::error::Error>> {
    
    let cot_data = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;

    println!("{}", cot_data);

    fs::write("data/raw/cot/date-RawCOTReport.csv", cot_data)?;

    Ok(())
} 

pub async fn fetch_fx_price_data(
    insturment: &str, 
    from_date: &str, 
    granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn std::error::Error>> {

    let api_key = env::var(OANDA_API_KEY).map_err(|_| "OANDA_API_KEY not set in .env file")?;

    let client = reqwest::Client::new();

    let get_data = client.get(&format!("https://api-fxtrade.oanda.com/v3/instruments/{}/candles", instrument))
        .header("Authorization", format!("Bearer {}", api_key))
        .query(&[
            ("price", price_type),
            ("from", from_date),
            ("granularity", granularity)
        ])
        .send()
        .await?
        .error_for_status()?;

    let fx_data = get_data.text().await?;
    
    fs::write(format!("data/raw/fx_prices/{}_{}_{}_RawFXPriceData.json", instrument, from_date, granualarity), fx_data)?;

    Ok(())
} 