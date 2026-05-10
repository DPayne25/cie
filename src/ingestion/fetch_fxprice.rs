use reqwest;
use std::{env, error::Error, fs};
use chrono::{DateTime, Utc, TimeZone, Duration};
use dotenvy::dotenv;
use fxoanda;
use tokio::time::sleep;
use std::time::Duration as StdDuration;

pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    instrument: &str,
    price_type: &str,
) -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("OANDA_API_KEY").map_err(|_| "OANDA_API_KEY not set in .env file")?;

    let oanda_client = fxoanda::Client {
        host: "api-fxtrade.oanda.com".to_string(),
        reqwest: client.clone(),
        authentication: api_key.to_string(),
    };

    let start: DateTime<Utc> = Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap();

    // --- H1 ---
    let mut current_start = start;
    let mut all_candles_h1: Vec<fxoanda::Candlestick> = Vec::new();

    while current_start < Utc::now() {
        let response = fxoanda::GetInstrumentCandlesRequest::new()
            .with_instrument(instrument.to_string())
            .with_from(current_start)
            .with_granularity(fxoanda::CandlestickGranularity::H1)
            .with_price(price_type.to_string())
            .with_count(5000)
            .remote(&oanda_client)
            .await?;

        let candles = match response.candles {
            Some(c) if !c.is_empty() => c,
            _ => break,
        };

        match candles.last().and_then(|c| c.time) {
            Some(last_time) => current_start = last_time + Duration::hours(1),
            None => break,
        }

        all_candles_h1.extend(candles);
        sleep(StdDuration::from_millis(500)).await;
    }

    // --- Daily ---
    let mut current_start = start;
    let mut all_candles_d: Vec<fxoanda::Candlestick> = Vec::new();

    while current_start < Utc::now() {
        let response = fxoanda::GetInstrumentCandlesRequest::new()
            .with_instrument(instrument.to_string())
            .with_from(current_start)
            .with_granularity(fxoanda::CandlestickGranularity::D)
            .with_price(price_type.to_string())
            .with_count(5000)
            .remote(&oanda_client)
            .await?;

        let candles = match response.candles {
            Some(c) if !c.is_empty() => c,
            _ => break,
        };

        match candles.last().and_then(|c| c.time) {
            Some(last_time) => current_start = last_time + Duration::days(1),
            None => break,
        }

        all_candles_d.extend(candles);
        sleep(StdDuration::from_millis(500)).await;
    }

    // --- Write ---
    fs::create_dir_all("data/raw/fx_prices/H1")?;
    fs::create_dir_all("data/raw/fx_prices/D")?;

    let json_h1 = serde_json::to_string_pretty(&all_candles_h1)?;
    fs::write(
        format!("data/raw/fx_prices/H1/{}_H1_RawFXPriceData.json", instrument),
        json_h1,
    )?;

    let json_d = serde_json::to_string_pretty(&all_candles_d)?;
    fs::write(
        format!("data/raw/fx_prices/D/{}_D_RawFXPriceData.json", instrument),
        json_d,
    )?;

    println!(
        "Done. {} H1 candles, {} D candles for {}.",
        all_candles_h1.len(),
        all_candles_d.len(),
        instrument
    );

    Ok(())
}
