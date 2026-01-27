use std::{env, error::Error};
use tokio::fs;
use chrono::{DateTime, Utc, TimeZone};
use fxoanda;
use crate::config::OandaConfig;


pub async fn fetch_cot_data() -> Result<(), Box<dyn Error>> {
    
    let cot_data = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;

    fs::write("data/raw/cot/date-RawCOTReport.csv", cot_data)?;

    Ok(())
} 

pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    config: &SentinelConfig,
    pool: sqlx::PgPool,
    instrument: &str,  
    granularity: &str, 
    price_type: &str,
    default_start_date: DateTime<Utc>
) -> Result<(), Box<dyn Error>> {//todo line 23


    let api_key: String = config.oanda_api_key.clone();

    let oanda_client= fxoanda::Client {
        host: config.oanda_host.clone(),
        reqwest: client.clone(),
        authentication: api_key.to_string()
    };


    let get_data = fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument(instrument.to_string())
        .with_from(default_start_date)
        .with_granularity(granularity.into())
        .with_price(price_type.to_string())
        .with_count(5000)
        .remote(&oanda_client).await;



    let fx_data = get_data?;

    let candles = fx_data.candles;

    let json = serde_json::to_string_pretty(&candles)?;

//    fs::write(format!("data/raw/fx_prices/{}_{}_RawFXPriceData.json", instrument, granularity), json)?;
    

    Ok(())
} 

pub enum CandlestickGranularity {
    S5, S10, S15, S30, M1, M2, M4, M5, M10, M15, M30, H1, H2, H3, H4, H6, H8, H12, D, W, M
}

impl From<CandlestickGranularity> for fxoanda::CandlestickGranularity {
    fn from(g: CandlestickGranularity) -> Self {
        match g {
            CandlestickGranularity::S5 => fxoanda::CandlestickGranularity::S5,
            CandlestickGranularity::S10 => fxoanda::CandlestickGranularity::S10,
            CandlestickGranularity::S15 => fxoanda::CandlestickGranularity::S15,
            CandlestickGranularity::S30 => fxoanda::CandlestickGranularity::S30,
            CandlestickGranularity::M1 => fxoanda::CandlestickGranularity::M1,
            CandlestickGranularity::M2 => fxoanda::CandlestickGranularity::M2,
             CandlestickGranularity::M4 => fxoanda::CandlestickGranularity::M4,
            CandlestickGranularity::M5 => fxoanda::CandlestickGranularity::M5,
            CandlestickGranularity::M10 => fxoanda::CandlestickGranularity::M10,
            CandlestickGranularity::M15 => fxoanda::CandlestickGranularity::M15,
            CandlestickGranularity::M30 => fxoanda::CandlestickGranularity::M30,
            CandlestickGranularity::H1 => fxoanda::CandlestickGranularity::H1,
            CandlestickGranularity::H2 => fxoanda::CandlestickGranularity::H2,
            CandlestickGranularity::H3 => fxoanda::CandlestickGranularity::H3,
            CandlestickGranularity::H4 => fxoanda::CandlestickGranularity::H4,
            CandlestickGranularity::H6 => fxoanda::CandlestickGranularity::H4,
            CandlestickGranularity::H8 => fxoanda::CandlestickGranularity::H8,
            CandlestickGranularity::H12 => fxoanda::CandlestickGranularity::H12,
            CandlestickGranularity::D => fxoanda::CandlestickGranularity::D,
            CandlestickGranularity::W => fxoanda::CandlestickGranularity::W,
            CandlestickGranularity::M => fxoanda::CandlestickGranularity::M
        }
    }
}