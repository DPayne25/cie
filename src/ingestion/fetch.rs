#![allow(unused_imports)]
use std::{env, error::Error, io::{Cursor}, path::Path};
use sqlx::{Executor, PgPool, QueryBuilder, query};
use tokio::fs;
use chrono::{DateTime, TimeZone, Utc, format::ParseError, NaiveDate};
use fxoanda;
use csv::ReaderBuilder;
use serde::Deserialize;
use zip::ZipArchive;

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

    let mut processed_cot: Vec<ProcessedCot> = Vec::new();

    for result in cot_data.deserialize::<RawCot>() {

        let raw_cot = result?;

        if !target_tff_codes.contains(&raw_cot.tff_code.as_str()) {
            continue;
        } 

        println!("• Processing Target: {} ({})", raw_cot.market_name, raw_cot.tff_code);
        
        processed_cot.push(ProcessedCot {
            market_name: raw_cot.market_name,
            report_date: {
                let naivedate =NaiveDate::parse_from_str(&raw_cot.report_date, "%Y-%m-%d")?;
                naive_date.and_hms_opt(0,0,0)
                    .map(|dt| dt.ant_utc())
                    .ok_or_else(|| format!("※ Invalid date format for report_date: {}", raw_cot.report_date))?
            },
            tff_code: raw_cot.tff_code,
            open_interest_all: raw_cot.open_interest_all.parse::<f64>()?,
            dealer_long: parse_cftc_numbers(&raw_cot.dealer_long)?,
            dealer_short: parse_cftc_numbers(&raw_cot.dealer_short)?,
            dealer_spread: parse_cftc_numbers(&raw_cot.dealer_spread)?,
            asset_mgr_long: parse_cftc_numbers(&raw_cot.asset_mgr_long)?,
            asset_mgr_short: parse_cftc_numbers(&raw_cot.asset_mgr_short)?,
            asset_mgr_spread: parse_cftc_numbers(&raw_cot.asset_mgr_spread)?,
            lev_money_long: parse_cftc_numbers(&raw_cot.lev_money_long)?,
            lev_money_short: parse_cftc_numbers(&raw_cot.lev_money_short)?,
            lev_money_spread: parse_cftc_numbers(&raw_cot.lev_money_spread)?,
            other_rept_long: parse_cftc_numbers(&raw_cot.other_rept_long)?,
            other_rept_short: parse_cftc_numbers(&raw_cot.other_rept_short)?,
            other_rept_spread: parse_cftc_numbers(&raw_cot.other_rept_spread)?,
        });

    }

    if processed_cot.is_empty() {
        println!("※ Warning: No target COT records found for year {}.", year);
        return Ok(());
    }

    let mut query_builder: QueryBuilder<Pg> = QueryBuilder::new(
        "INSERT INTO raw_cot_reports (market_name, report_date, tff_code, open_interest_all, dealer_long, dealer_short, dealer_spread, asset_mgr_long, asset_mgr_short, asset_mgr_spread, lev_money_long, lev_money_short, lev_money_spread, other_rept_long, other_rept_short, other_rept_spread) "
    );

    query_builder.push_values(
        processed_cot.iter(),
        |mut b, cot| {
            b.push_bind(&cot.market_name)
             .push_bind(&cot.report_date)
             .push_bind(&cot.tff_code)
             .push_bind(&cot.open_interest_all)
             .push_bind(&cot.dealer_long)
             .push_bind(&cot.dealer_short)
             .push_bind(&cot.dealer_spread)
             .push_bind(&cot.asset_mgr_long)
             .push_bind(&cot.asset_mgr_short)
             .push_bind(&cot.asset_mgr_spread)
             .push_bind(&cot.lev_money_long)
             .push_bind(&cot.lev_money_short)
             .push_bind(&cot.lev_money_spread)
             .push_bind(&cot.other_rept_long)
             .push_bind(&cot.other_rept_short)
             .push_bind(&cot.other_rept_spread);
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

    let mut query = query_builder.build();

    query.execute(&pool ).await?;

    Ok(())
} 

pub fn parse_cftc_numbers(value: &str) -> Result<i64, Box<dyn Error>> { 
    let cleaned_value = value
        .replace(",", "")
        .trim()
        .to_string();
        cleaned_value.parse::<i64>().map_err(|e| e.into())
}

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

#[derive(Deserialize, Debug)]
struct ProcessedCot{
    #[serde(rename = "Market_and_Exchange_Names")]
    market_name: String,
    #[serde(rename = "Report_Date_as_YYYY-MM-DD")]
    report_date: DateTime<Utc>,
    #[serde(rename = "CFTC_Contract_Market_Code")]
    tff_code: String,
    #[serde(rename = "Open_Interest_All")]
    open_interest_all: f64,
    #[serde(rename = "Dealer_Positions_Long_All")]
    dealer_long: i64,
    #[serde(rename = "Dealer_Positions_Short_All")]
    dealer_short: i64,
    #[serde(rename = "Dealer_Positions_Spread_All")]
    dealer_spread: i64,
    #[serde(rename = "Asset_Mgr_Positions_Long_All")]
    asset_mgr_long: i64,
    #[serde(rename = "Asset_Mgr_Positions_Short_All")]
    asset_mgr_short: i64,
    #[serde(rename = "Asset_Mgr_Positions_Spread_All")]
    asset_mgr_spread: i64,
    #[serde(rename = "Lev_Money_Positions_Long_All")]
    lev_money_long: i64,
    #[serde(rename = "Lev_Money_Positions_Short_All")]
    lev_money_short: i64,
    #[serde(rename = "Lev_Money_Positions_Spread_All")]
    lev_money_spread: i64,
    #[serde(rename = "Other_Rept_Positions_Long_All")]
    other_rept_long: i64,
    #[serde(rename = "Other_Rept_Positions_Short_All")]
    other_rept_short: i64,
    #[serde(rename = "Other_Rept_Positions_Spread_All")]
    other_rept_spread: i64,
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
    config: &SentinelConfig,
    pool: PgPool,
    instrument: &str,  
    granularity: CandlestickGranularity, 
    price_type: &str
) -> Result<(), Box<dyn Error>> {

    let start_date: DateTime<Utc> = db::util::get_latest_timestamp(&pool, instrument)
        .await?
        .unwrap_or(config.default_start_date);

    let api_key: String = config.oanda_api_key.clone();

    let oanda_client= fxoanda::Client {
        host: config.oanda_host,
        reqwest: client.clone(),
        authentication: api_key,
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

    if candles.is_empty(){
        println!("※ Warning: No new FX price data found for {}.", instrument);
        return Ok(());
    }

    let rosw_to_insert: Vec<FxProceRow> = candles.iter().map(|candle| {
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

    query.execute(&pool).await?;

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
            CandlestickGranularity::H6 => fxoanda::CandlestickGranularity::H6,
            CandlestickGranularity::H8 => fxoanda::CandlestickGranularity::H8,
            CandlestickGranularity::H12 => fxoanda::CandlestickGranularity::H12,
            CandlestickGranularity::D => fxoanda::CandlestickGranularity::D,
            CandlestickGranularity::W => fxoanda::CandlestickGranularity::W,
            CandlestickGranularity::M => fxoanda::CandlestickGranularity::M
        }
    }
}

