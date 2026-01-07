use std::{fs, env};
use chrono::{DateTime, Utc, TimeZone};
use dotenvy::dotenv;
use fxoanda::GetInstrumentCandlesRequest;



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
    client: &reqwest::Client,
    instrument: &str, 
    from_date: DateTime<Utc>, 
    //granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2016, 1, 1, 15, 0, 0).unwrap(); //todo line 23

    let oanda_time_format = dt.to_rfc3339();

    let from_date = dt.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
    dotenv().ok();

    let api_key = env::var("OANDA_API_KEY").map_err(|_| "OANDA_API_KEY not set in .env file")?;

    let oanda_client= fxoanda::Client {
        host: "api-fxtrade.oanda.com".to_string(),
        reqwest: client.clone(),
        authentication: api_key.to_string()
    };

    //let from_date_str = from_date.to_rfc3339();

    let mut get_data = fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument(instrument.to_string())
        .with_from(dt)
        .with_granularity(fxoanda::CandlestickGranularity::D)
        .with_price(price_type.to_string())
        .remote(&oanda_client).await;



    let fx_data = get_data?.json().await?;
    
    fs::write(format!("data/raw/fx_prices/{}_{}_{}_RawFXPriceData.json", instrument, from_date, granularity), fx_data)?;

    Ok(())
} 