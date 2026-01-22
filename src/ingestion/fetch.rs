use std::{fs, env};
use tokio::fs;
use chrono::{DateTime, Utc, TimeZone};
use fxoanda;
use crate::config::OandaConfig;


pub async fn fetch_cot_data() -> Result<(), Box<dyn std::error::Error>> {
    
    let cot_data = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;

    fs::write("data/raw/cot/date-RawCOTReport.csv", cot_data)?;

    Ok(())
} 

pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    config: &OandaConfig,
    instrument: &str,  
    granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn std::error::Error>> {

    let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap(); //todo line 23


    let api_key: String = config.api_key.clone();

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