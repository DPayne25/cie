#![allow(unused_imports)]
use std::{env, error::Error, io::{Cursor, Seek, Read}, path::Path, future::IntoFuture, num::ParseIntError};
use sqlx::{Executor, PgPool, Postgres, QueryBuilder, query};
use tokio::fs;
use chrono::{DateTime, TimeZone, Utc, format::ParseError, NaiveDate};
use fxoanda;
use csv::ReaderBuilder;
use serde::Deserialize;
use zip::ZipArchive;
use bytes::Bytes;
use rust_decimal::{Decimal, prelude::FromPrimitive};
#[path = "../config.rs"]
mod config;
#[path = "../db/util.rs"]
mod util;

//=======================================================================================================
// Structs and Enums
//=======================================================================================================

#[derive(Deserialize, Debug)]
struct RawCot{
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

#[derive(Debug, sqlx::FromRow)]
struct CotReport {
    market_name: String,
    report_date: NaiveDate,
    tff_code: String,
    open_interest_all: Decimal,
    dealer_long: Decimal,
    dealer_short: Decimal,
    dealer_spread: Decimal,
    asset_mgr_long: Decimal,
    asset_mgr_short: Decimal,
    asset_mgr_spread: Decimal,
    lev_money_long: Decimal,
    lev_money_short: Decimal,
    lev_money_spread: Decimal,
    other_rept_long: Decimal,
    other_rept_short: Decimal,
    other_rept_spread: Decimal,
}

impl TryFrom<RawCot> for CotReport {
    type Error = Box<dyn Error>;

    fn try_from(raw: RawCot) -> Result<Self, Self::Error> {
        Ok(CotReport {
            market_name: raw.market_name,
            report_date: NaiveDate::parse_from_str(&raw.report_date, "%Y-%m-%d")?,
            tff_code: raw.tff_code,
            open_interest_all: raw.open_interest_all.parse::<Decimal>()?,
            dealer_long: Decimal::from_str(&raw.dealer_long.replace(',', ""))?,
            dealer_short: Decimal::from_str(&raw.dealer_short.replace(',', ""))?,
            dealer_spread: Decimal::from_str(&raw.dealer_spread.replace(',', ""))?,
            asset_mgr_long: Decimal::from_str(&raw.asset_mgr_long.replace(',', ""))?,
            asset_mgr_short: Decimal::from_str(&raw.asset_mgr_short.replace(',', ""))?,
            asset_mgr_spread: Decimal::from_str(&raw.asset_mgr_spread.replace(',', ""))?,
            lev_money_long: Decimal::from_str(&raw.lev_money_long.replace(',', ""))?,
            lev_money_short: Decimal::from_str(&raw.lev_money_short.replace(',', ""))?,
            lev_money_spread: Decimal::from_str(&raw.lev_money_spread.replace(',', ""))?,
            other_rept_long: Decimal::from_str(&raw.other_rept_long.replace(',', ""))?,
            other_rept_short: Decimal::from_str(&raw.other_rept_short.replace(',', ""))?,
            other_rept_spread: Decimal::from_str(&raw.other_rept_spread.replace(',', ""))?,
        })

    }
}





//=======================================================================================================
// COT Data Fetching
//=======================================================================================================

pub async fn fetch_cot_data(year: i32, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    
    let cot_request = reqwest::get(format!("https://www.cftc.gov/files/dea/history/fut_fin_txt_{}.zip", year))
        .await?
        .bytes()
        .await?;

    let mut archive = ZipArchive::new(Cursor::from(cot_request))?;
    
    let index_zip = (0..archive.len())
        .find(|&i| {archive.by_index(i).unwrap().name().ends_with(".txt")})
        .ok_or("※ No .txt file found in the ZIP archive.")?;   
    
    let file = archive.by_index(index_zip)?;

    let cot_data = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(file);

    let target_tff_codes  = ["090741","092741", "096742", "097741", "099741", "232741", "112741", "095741", "120741", "216742", "233741"];

    let mut reports: Vec<CotReport> = Vec::new();


for result in cot_data.into_deserialize::<RawCot>() {
        // Handle CSV/Deserialization errors safely
        let record = match result {
            Ok(rec) => rec,
            Err(e) => {
                eprintln!("※ Skipping malformed CSV record: {}", e);
                continue;
            }
        };

        if target_tff_codes.contains(&record.tff_code.as_str()) {
            // Handle Domain Parsing errors safely
            match CotReport::try_from(record) {

                Ok(cot_report) => reports.push(cot_report),

                Err(e) => {

                    eprintln!("※ Data Corruption Detected. Skipping Row: {}", e);
                
                    continue;
                }
            }
        }
    }

    if reports.is_empty() {
        println!("※ Warning: No valid records found for year {}.", year);
        return Ok(());
    }

    println!("• Batching {} records for insertion...", reports.len());
    
    batch_insert_cot(pool, reports).await?;

    Ok(())
} 

pub fn parse_cftc_numbers(value: &str) -> Result<i64, ParseIntError> { 
    if value.contains(',') {
        value.replace(",", "").trim().parse()
    } else {
        value.trim().parse()
    }
}

pub async fn batch_insert_cot(pool: &PgPool, reports: Vec<CotReport>) -> Result<(), Box<dyn Error>> {
    const BATCH_SIZE: usize = 2000;
    for chunk in reports.chunks(BATCH_SIZE) {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO raw_cot_reports (market_name, report_date, tff_code, open_interest_all, dealer_long, dealer_short, dealer_spread, asset_mgr_long, asset_mgr_short, asset_mgr_spread, lev_money_long, lev_money_short, lev_money_spread, other_rept_long, other_rept_short, other_rept_spread) "
        );

        query_builder.push_values(
            chunk.iter(),
            |mut b, report| {
                b.push_bind(&report.market_name)
                .push_bind(report.report_date)
                .push_bind(&report.tff_code)
                .push_bind(report.open_interest_all)
                .push_bind(report.dealer_long)
                .push_bind(report.dealer_short)
                .push_bind(report.dealer_spread)
                .push_bind(report.asset_mgr_long)
                .push_bind(report.asset_mgr_short)
                .push_bind(report.asset_mgr_spread)
                .push_bind(report.lev_money_long)
                .push_bind(report.lev_money_short)
                .push_bind(report.lev_money_spread)
                .push_bind(report.other_rept_long)
                .push_bind(report.other_rept_short)
                .push_bind(report.other_rept_spread);
            }
        );

        query_builder.push(
            " ON CONFLICT (tff_code, report_date) DO UPDATE SET 
            market_name = EXCLUDED.market_name,
            open_interest_all = EXCLUDED.open_interest_all,
            dealer_long = EXCLUDED.dealer_long,
            dealer_short = EXCLUDED.dealer_short,
            dealer_spread = EXCLUDED.dealer_spread,
            asset_mgr_long = EXCLUDED.asset_mgr_long,
            asset_mgr_short = EXCLUDED.asset_mgr_short,
            asset_mgr_spread = EXCLUDED.asset_mgr_spread,
            lev_money_long = EXCLUDED.lev_money_long,
            lev_money_short = EXCLUDED.lev_money_short,
            lev_money_spread = EXCLUDED.lev_money_spread,
            other_rept_long = EXCLUDED.other_rept_long,
            other_rept_short = EXCLUDED.other_rept_short,
            other_rept_spread = EXCLUDED.other_rept_spread;"
        );

        let query = query_builder.build();

        query.execute(pool).await?;
    }

    Ok(())
}



//=======================================================================================================
// FX Price Data Fetching
//=======================================================================================================

struct FxPriceRow<'a> {
    symbol: &'a str,
    date: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
    complete: bool,
}


pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    config: &config::SentinelConfig,
    pool: &PgPool,
    instrument: &str,  
    granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn Error>> {

    let start_date: DateTime<Utc> = util::get_latest_timestamp(&pool, instrument)
        .await?
        .unwrap_or(config.default_start_date);

    let api_key: String = config.oanda_api_key.clone();

    let oanda_client= fxoanda::Client {
        reqwest: client.clone(),
        host: config.oanda_host,
        authentication: api_key,
    };


    let get_data = fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument(instrument.to_string())
        .with_from(start_date)
        .with_granularity(fxoanda::CandlestickGranularity::D)
        .with_price(price_type.to_string())
        .with_count(5000)
        .remote(&oanda_client)?;



    let fx_data = get_data?;


    let candles = fx_data.candles;

    if candles.is_empty(){
        println!("※ Warning: No new FX price data found for {}.", instrument);
        return Ok(());
    }

    let rows_to_insert: Vec<FxPriceRow> = candles.iter().map(|candle| {
        FxPriceRow {
            symbol: instrument,
            date: candle.time,
            open: candle.mid.o,
            high: candle.mid.h,
            low: candle.mid.l,
            close: candle.mid.c,
            volume: candle.volume,
            complete: candle.complete
        }
    }).collect();
    
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO raw_fx_prices (symbol, date, open, high, low, close, volume, complete) "
    );

    query_builder.push_values(
        rows_to_insert.iter(),
        |mut b, row| {
            b.push_bind(row.symbol)
             .push_bind(row.date)
             .push_bind(row.open)
             .push_bind(row.high)
             .push_bind(row.low)
             .push_bind(row.close)
             .push_bind(row.volume)
             .push_bind(row.complete);
        }
    );

    query_builder.push(
        " ON CONFLICT (symbol, date) DO UPDATE SET 
        open = EXCLUDED.open,
        high = EXCLUDED.high,
        low = EXCLUDED.low,
        close = EXCLUDED.close,
        volume = EXCLUDED.volume,
        complete = EXCLUDED.complete;"
    );

    let query = query_builder.build();

    query.execute(pool).await?;

    Ok(())
} 



//=======================================================================================================
//Testing Module
//=======================================================================================================

#[cfg(test)] // Only compiles when running 'cargo test'
mod tests {
    use super::*;
    use rust_decimal_macros::dec; // You might need to add this to Cargo.toml

    #[test]
    fn test_cot_parsing_logic() {
        // 1. Create a Fake Raw Record (Mock Data)
        let raw = RawCot {
            market_name: "EURO FX".to_string(),
            report_date: "2024-01-01".to_string(),
            tff_code: "099741".to_string(),
            open_interest_all: "100.5".to_string(),
            dealer_long: "1,234".to_string(), // Test comma removal
            // ... fill other fields with dummy strings ...
            dealer_short: "0".to_string(),
            dealer_spread: "0".to_string(),
            asset_mgr_long: "0".to_string(),
            asset_mgr_short: "0".to_string(),
            asset_mgr_spread: "0".to_string(),
            lev_money_long: "0".to_string(),
            lev_money_short: "0".to_string(),
            lev_money_spread: "0".to_string(),
            other_rept_long: "0".to_string(),
            other_rept_short: "0".to_string(),
            other_rept_spread: "0".to_string(),
        };

        // 2. Run the Logic
        let result = CotReport::try_from(raw);

        // 3. Assert Expectations
        assert!(result.is_ok());
        let report = result.unwrap();
        
        assert_eq!(report.tff_code, "099741");
        assert_eq!(report.dealer_long, dec!(1234)); // Verified comma was removed!
    }
}