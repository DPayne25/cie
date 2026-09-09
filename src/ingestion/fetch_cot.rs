use std::{fs::{self, File}, error::Error, path::Path, io};
use rust_decimal::Decimal;
use zip::ZipArchive;
use sqlx::Type;
use chrono;

pub async fn fetch_cot_data(year: i32) -> Result<String, Box<dyn Error>> {
    
    let url = format!("https://www.cftc.gov/files/dea/history/fut_fin_txt_{}.zip", year);


    let response_bytes = reqwest::get(&url).await?.bytes().await?;
    let cursor = io::Cursor::new(response_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let mut file_in_zip = archive.by_index(0)
        .map_err(|_| format!("Could not find 'FinFutWk.txt' in archive for year {}", year))?;

    let out_dir = Path::new("data/raw/cot");
    fs::create_dir_all(out_dir)?;

    let temp_path_str = "data/raw/cot/date-RawCOTReport.csv";
    let out_path = Path::new(temp_path_str);
    let mut outfile = File::create(&out_path)?;

    io::copy(&mut file_in_zip, &mut outfile)?;


    Ok(temp_path_str.to_string())
} 


#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "currency_base_type")] // Must match the exact PostgreSQL type name
pub enum CurrencyBaseType {
    Direct,
    Inverted,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DimCurrency {
    pub cftc_contract_code: String,
    pub currency_pair: String,
    pub base_type: CurrencyBaseType,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CotTff {
    pub market_exchange_names: String,
    pub report_date: Date<>,
    pub cftc_market_code: String,
    pub open_interest: i32,
    pub dealer_positions_long: i32,
    pub dealer_positions_short: i32,
    pub dealer_positions_spread: i32,
    pub asset_manager_positions_long: i32,
    pub asset_manager_positions_short: i32,
    pub asset_manager_positions_spread: i32,
    pub leveraged_money_positions_long: i32,
    pub leveraged_money_positions_short: i32,
    pub leveraged_money_positions_spread: i32,
    pub other_rept_positions_long: i32,
    pub other_rept_positions_short: i32,
    pub other_rept_positions_spread: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct FxPrice {
    pub currency_pair: String,
    pub price_date: Date<>,
    pub complete: bool,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub lose_price: Decimal,
    pub tick_volume: i32,
}
