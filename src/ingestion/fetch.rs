use std::{fs, env, path::Path};
use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use fxoanda;
use crate::ingestion::errors::IngestionError;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum PriceType {
    Bid, Ask, Mid
}

impl From<PriceType> for String {
    fn from(p: PriceType) -> Self {
        match p {
            PriceType::Bid => "bid".to_string(),
            PriceType::Ask => "ask".to_string(),
            PriceType::Mid => "mid".to_string()
        }
    }
}

pub async fn fetch_cot_data(output_path: &Path) -> Result<(), IngestionError> {
    let cot_data = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;

    fs::write(output_path, cot_data)?;

    Ok(())
} 

pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    instrument: &str,  
    granularity: CandlestickGranularity, 
    price_type: PriceType,
    from_date: DateTime<Utc>
) -> Result<(), IngestionError> {
    //let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap(); //todo line 23
    dotenv().ok();

    let api_key = env::var("OANDA_API_KEY")
        .map_err(|e| IngestionError::Config(format!("OANDA_API_KEY not set: {}", e)))?;

    let oanda_client= fxoanda::Client {
        host: "api-fxtrade.oanda.com".to_string(),
        reqwest: client.clone(),
        authentication: api_key
    };

    let get_data = fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument(instrument.to_string())
        .with_from(from_date)
        .with_granularity(granularity.into())
        .with_price(price_type.into())
        .with_count(5000)
        .remote(&oanda_client).await;



    let fx_data = get_data?;

    let candles = fx_data.candles;

    let json = serde_json::to_string_pretty(&candles)?;

    let filename = format!("data/raw/fx_prices/{}_{}_RawFXPriceData.json", instrument, <CandlestickGranularity as Into<fxoanda::CandlestickGranularity>>::into(granularity));

    fs::write(&filename, json)?;

    Ok(())
} 