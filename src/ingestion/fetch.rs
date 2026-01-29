use std::{env, error::Error, io::{Cursor}};
use tokio::fs;
use chrono::{DateTime, Utc, TimeZone};
use fxoanda;
use csv::ReaderBuilder;
use serde::Deserialize;

//=======================================================================================================
// COT Data Fetching
//=======================================================================================================

pub async fn fetch_cot_data(year: i32) -> Result<(), Box<dyn Error>> {
    
    let cot_request = reqwest::get(format!("https://www.cftc.gov/files/dea/history/com_disagg_txt_{}.zip", year))
        .await?
        .bytes()
        .await?;

    let mut archive = ZipArchive::new(Cursor::from(cot_request))?;
    
    let index_zip = (0..archive.len())
        .find(|&i| {archive.by_index(i).unwrap().name().ends_with(".txt")})
        .ok_or("No .txt file found in the ZIP archive.")?;   
    
    let file = archive.by_index(index_zip)?;
    let cot_data = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(file);

    let target_tff_codes: HashSet<&str> = ["090741","092741", "096742", "097741", "099741", "232741", "112741"]
        .iter()
        .collect();

    for result in cot_data.deserialize::<RawCOTRow>() {
        let raw = result?;

        if !target_tff_codes.contains(raw.)
    }

    Ok(())
} 

pub fn parse_cftc_numbers(value: &str) -> i64 { 
        value
            .replace(",", "")
            .trim()
            .parse::<i64>().unwrap_or(0)
}

#[derive(Deserialize, Debug)]
struct RawCOTRow{
    #[serde(rename = "Market_and_Exchange_Names")]
    market_name: String,
    #[serde(rename = "Report_Date_as_YYYY-MM-DD")]
    report_date: String,
    #[serde(rename = "CFTC_Contract_Market_Code")]
    tff_code: String,
    #[serde(rename = "Open_Interest_All")]
    open_interest_all: String,
    #[serde(rename = "Dealer_Positions_Long_All")]
    dealer_long: String,
    #[serde(rename = "Dealer_Positions_Short_All")]
    dealer_short: String,
    #[serde(rename = "Dealer_Positions_Spread_All")]
    dealer_spread: String,
    #[serde(rename = "Asset_Mgr_Positions_Long_All")]
    asset_mgr_long: String,
    #[serde(rename = "Asset_Mgr_Positions_Short_All")]
    asset_mgr_short: String,
    #[serde(rename = "Asset_Mgr_Positions_Spread_All")]
    asset_mgr_spread: String,
    #[serde(rename = "Lev_Money_Positions_Long_All")]
    lev_money_long: String,
    #[serde(rename = "Lev_Money_Positions_Short_All")]
    lev_money_short: String,
    #[serde(rename = "Lev_Money_Positions_Spread_All")]
    lev_money_spread: String,
    #[serde(rename = "Other_Rept_Positions_Long_All")]
    other_rept_long: String,
    #[serde(rename = "Other_Rept_Positions_Short_All")]
    other_rept_short: String,
    #[serde(rename = "Other_Rept_Positions_Spread_All")]
    other_rept_spread: String,
}

//=======================================================================================================
// FX Price Data Fetching
//=======================================================================================================

pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    config: &SentinelConfig,
    pool: sqlx::PgPool,
    instrument: &str,  
    granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn Error>> {

    let start_date: DateTime<Utc> = db::util::get_latest_timestamp(&pool, instrument)
        .await?
        .unwrap_or(config.default_start_date);

    let api_key: String = config.oanda_api_key.clone();

    let oanda_client= fxoanda::Client {
        host: config.oanda_host.clone(),
        reqwest: client.clone(),
        authentication: api_key.to_string()
    };


    let get_data = fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument(instrument.to_string())
        .with_from(start_date)
        .with_granularity(granularity.into())
        .with_price(price_type.to_string())
        .with_count(5000)
        .remote(&oanda_client).await;



    let fx_data = get_data?;

    let candles = fx_data.candles;

    for candle in candles { 
        sqlx::query!(
            "INSERT INTO raw_fx_prices (symbol, date, open, high, low, close, volume, complete) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (symbol, date) DO UPDATE SET 
            date = EXCLUDED.date,
            open = EXCLUDED.open,
            high = EXCLUDED.high,
            low = EXCLUDED.low,
            close = EXCLUDED.close,
            volume = EXCLUDED.volume,
            complete = EXCLUDED.complete;",
            instrument,
            candle.time,
            candle.mid.o,
            candle.mid.h,
            candle.mid.l,
            candle.mid.c,
            candle.volume,
            candle.complete
        ).execute(&pool).await?;
    }

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