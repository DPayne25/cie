use std::{fs, env};
use chrono::{DateTime, Utc, TimeZone};
use dotenvy::dotenv;
use fxoanda::*;
use reqwest::{ClientBuilder, Client, Url};

pub async fn fetch_cot_data() -> Result<(), Box<dyn std::error::Error>> {
    
    let cot_data = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;

    println!("{}", cot_data);

    fs::write("data/raw/cot/date-RawCOTReport.csv", cot_data)?;

    Ok(())
} 

pub fn fetch_fx_price_data(
    instrument: &str, 
    //from_date: DateTime<Utc>, Add loop to input into `dt` below #todo so input varies on input
    granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2016, 1, 1, 15, 0, 0).unwrap(); //todo line 23

    let oanda_time_format = dt.to_rfc3339();

    dotenv().ok();

    let api_key = env::var("OANDA_API_KEY").map_err(|_| "OANDA_API_KEY not set in .env file")?;

    let client= fxoanda::Client {
        host: "api-fxtrade.oanda.com".to_string(),
        reqwest: reqwest::Client::new(),
        authentication: String::from(api_key)
    };

    let mut get_data = fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument(instrument.to_string())
        .with_from(from_date)
        .with_granularity(CandlestickGranularity::from_str(granularity)?)
        .with_price(price_type.to_string())
        .remote(&client).await;



    let fx_data = get_data.json().await?;
    
    fs::write(format!("data/raw/fx_prices/{}_{}_{}_RawFXPriceData.json", instrument, from_date, granularity), fx_data)?;

    Ok(())
} 