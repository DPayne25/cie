use reqwest;
use std::{fs, env, error::Error};
use chrono::{DateTime, Utc, TimeZone};
use dotenvy::dotenv;
use fxoanda;


pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    instrument: &str, 
    //from_date: DateTime<Utc>, 
    granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn Error>> {
    let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap(); //todo line 23

    dotenv().ok();

    let api_key = env::var("OANDA_API_KEY").map_err(|_| "OANDA_API_KEY not set in .env file")?;

    let oanda_client= fxoanda::Client {
        host: "api-fxtrade.oanda.com".to_string(),
        reqwest: client.clone(),
        authentication: api_key.to_string()
    };

    //let from_date_str = from_date.to_rfc3339();

    let get_data = fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument(instrument.to_string())
        .with_from(dt)
        .with_granularity(fxoanda::CandlestickGranularity::D)
        .with_price(price_type.to_string())
        .with_count(5000)
        .remote(&oanda_client).await;



    let fx_data = get_data?;

    let candles = fx_data.candles;

    let json = serde_json::to_string_pretty(&candles)?;

    fs::write(format!("data/raw/fx_prices/{}_{}_RawFXPriceData.json", instrument, granularity), json)?;

    Ok(())
} 