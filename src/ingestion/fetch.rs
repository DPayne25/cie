use std::{fs, env, error::Error, str::FromStr, num::ParseIntError, io::{self, Cursor}};
use chrono::{DateTime, Utc, TimeZone, NaiveDate};
use dotenvy::dotenv;
use fxoanda;
use csv::ReaderBuilder;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use zip::ZipArchive;


#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Debug,  Serialize)]
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

/*
pub fn parse_cftc_numbers(value: &str) -> Result<i64, ParseIntError> { 
    if value.contains(',') {
        value.replace(",", "").trim().parse()
    } else {
        value.trim().parse()
    }
}
*/

pub async fn fetch_cot_data(year: i64) -> Result<(), Box<dyn Error>> {
    
    // Using bytes in preparation for a future hard disk bypass version
    let cot_request = reqwest::get(format!("https://www.cftc.gov/files/dea/history/fut_fin_txt_{}.zip", year))
        .await?
        .bytes()
        .await?;

    let mut archive = ZipArchive::new(io::Cursor::new(cot_request))?;
    
    let mut index_zip = None;

    for i in 0..archive.len() {
        
        if let Ok(file) = archive.by_index(i) {
            
            if file.name().ends_with(".txt") {
                index_zip = Some(i);
                break; 
            }
        }
    }

    // 4. Handle the Option result
    let final_index = index_zip.ok_or("※ No .txt file found")?;
    
    let file_in_zip = archive.by_index(final_index)?;

    let cot_data = csv::ReaderBuilder::new()
        .from_reader(file_in_zip);

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
        return Ok(())
    }

    let path = "data/cot_alpha_export.csv";

    let mut writer = csv::Writer::from_path(path)?;

    for report in reports {
        writer.serialize(report)?;
    }

    writer.flush()?;

    //println!("• Batching {} records for insertion...", reports.len());

    Ok(())
} 


pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    instrument: &str, 
    //from_date: DateTime<Utc>, 
    granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn std::error::Error>> {
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