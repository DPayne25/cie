use chrono::{self, NaiveDate};
use rust_decimal::Decimal;
use sqlx::{PgPool, Type, pool};
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::Path,
};
use serde::Deserialize;
use zip::ZipArchive;

pub async fn fetch_cot_data(year: i32, db_pool: &PgPool) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://www.cftc.gov/files/dea/history/fut_fin_txt_{}.zip",
        year
    );
    
    let file_in_zip = get_zip_file(&url).await?;
 
    let data = parse_cot_records(file_in_zip)?;

    cot_db_ingest(&cot_data, db_pool);

    Ok(temp_path_str.to_string())
}

#[derive(Debug, Clone, Type, Deserialize)]
#[sqlx(type_name = "currency_base_type")] // Must match the exact PostgreSQL type name
pub enum CurrencyBaseType {
    Direct,
    Inverted,
}

#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct DimCurrency {
    pub cftc_contract_market_code: String,
    pub currency_pair: String,
    pub base_type: CurrencyBaseType,
}

#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct CotTff {
    #[serde(rename = "Market_and_Exchange_Names")]
    pub market_exchange_names: String,
    #[serde(rename = "Report_Date_as_YYYY-MM-DD")]
    pub report_date: NaiveDate,
    #[serde(rename = "CFTC_Contract_Market_Code")]
    pub cftc_contract_market_code: String,
    #[serde(rename = "Open_Interest_All")]
    pub open_interest: i32,
    #[serde(rename = "Dealer_Positions_Long_All")]
    pub dealer_positions_long: i32,
    #[serde(rename = "Dealer_Positions_Short_All")]
    pub dealer_positions_short: i32,
    #[serde(rename = "Dealer_Positions_Spread_All")]
    pub dealer_positions_spread: i32,
    #[serde(rename = "Asset_Mgr_Positions_Long_All")]
    pub asset_manager_positions_long: i32,
    #[serde(rename = "Asset_Mgr_Positions_Short_All")]
    pub asset_manager_positions_short: i32,
    #[serde(rename = "Asset_Mgr_Positions_Spread_All")]
    pub asset_manager_positions_spread: i32,
    #[serde(rename = "Lev_Money_Positions_Long_All")]
    pub leveraged_money_positions_long: i32,
    #[serde(rename = "Lev_Money_Positions_Short_All")]
    pub leveraged_money_positions_short: i32,
    #[serde(rename = "Lev_Money_Positions_Spread_All")]
    pub leveraged_money_positions_spread: i32,
    #[serde(rename = "Other_Rept_Positions_Long_All")]
    pub other_rept_positions_long: i32,
    #[serde(rename = "Other_Rept_Positions_Short_All")]
    pub other_rept_positions_short: i32,
    #[serde(rename = "Other_Rept_Positions_Spread_All")]
    pub other_rept_positions_spread: i32,
    /*
    #[serde(rename = "Traders_Tot_All")]
    pub traders_total: Option<i32>,
    #[serde(rename = "Traders_Dealer_Long_All")]
    pub traders_dealer_long: Option<i32>,
    #[serde(rename = "Traders_Dealer_Short_All")]
    pub traders_dealer_short: Option<i32>,
    #[serde(rename = "Traders_Dealer_Spread_All")]
    pub traders_dealer_spread: Option<i32>,
    #[serde(rename = "Traders_Asset_Mgr_Long_All")]
    pub traders_asset_manager_long: Option<i32>,
    #[serde(rename = "Traders_Asset_Mgr_Short_All")]
    pub traders_asset_manager_short: Option<i32>,
    #[serde(rename = "Traders_Asset_Mgr_Spread_All")]
    pub traders_asset_manager_spread: Option<i32>,
    #[serde(rename = "Traders_Lev_Money_Long_All")]
    pub traders_leveraged_money_long: Option<i32>,
    #[serde(rename = "Traders_Lev_Money_Short_All")]
    pub traders_leveraged_money_short: Option<i32>,
    #[serde(rename = "Traders_Lev_Money_Spread_All")]
    pub traders_leveraged_money_spread: Option<i32>,
    #[serde(rename = "Traders_Other_Rept_Long_All")]
    pub traders_other_rept_long: Option<i32>,
    #[serde(rename = "Traders_Other_Rept_Short_All")]
    pub traders_other_rept_short: Option<i32>,
    #[serde(rename = "Traders_Other_Rept_Spread_All")]
    pub traders_other_rept_spread: Option<i32>,
    #[serde(rename = "Traders_Tot_Rept_Long_All")]
    pub traders_tot_rept_long: Option<i32>,
    #[serde(rename = "Traders_Tot_Rept_Short_All")]
    pub traders_tot_rept_short: Option<i32>,
    #[serde(rename = "Conc_Gross_LE_4_TDR_Long_All")]
    pub conc_gross_le4_long: Option<Decimal>,
    #[serde(rename = "Conc_Gross_LE_4_TDR_Short_All")]
    pub conc_gross_le4_short: Option<Decimal>,
    #[serde(rename = "Conc_Gross_LE_8_TDR_Long_All")]
    pub conc_gross_le8_long: Option<Decimal>,
    #[serde(rename = "Conc_Gross_LE_8_TDR_Short_All")]
    pub conc_gross_le8_short: Option<Decimal>,
    #[serde(rename = "Conc_Net_LE_4_TDR_Long_All")]
    pub conc_net_le4_long: Option<Decimal>,
    #[serde(rename = "Conc_Net_LE_4_TDR_Short_All")]
    pub conc_net_le4_short: Option<Decimal>,
    #[serde(rename = "Conc_Net_LE_8_TDR_Long_All")]
    pub conc_net_le8_long: Option<Decimal>,
    #[serde(rename = "Conc_Net_LE_8_TDR_Short_All")]
    pub conc_net_le8_short: Option<Decimal>,*/
}

#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct FxPrice {
    pub currency_pair: String,
    pub price_date: NaiveDate,
    pub complete: bool,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub close_price: Decimal,
    pub tick_volume: i32,
}

pub async fn get_zip_file(url: &String) -> Result<ZipFile<file_in_zip>, anyhow::Error> {
    let response_bytes  = reqwest::get(&url).await?.bytes().await?;
    let cursor  = io::Cursor::new(response_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let mut file_in_zip = archive
        .by_index(0)
        .map_err(|_| format!("Could not find 'FinFutWk.txt' in archive for year {}", year))?;
    
    Ok(file_in_zip)
}

async fn parse_cot_records<R: Read>(reader: R) -> Result<Vec<CotTff>>, anyhow::Error> {
    csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(reader)
}

pub async fn cot_db_ingest(data: &CotTff, pool: &PgPool) -> Result<(), Box<dyn Error>>{
    sqlx::query!(
        r#"INSERT INTO cot_tff (
            market_exchange_names, 
            report_date,
            cftc_contract_market_code,
            open_interest,
            dealer_positions_long,
            dealer_positions_short,
            dealer_positions_spread,
            asset_manager_positions_long,
            asset_manager_positions_short,
            asset_manager_positions_spread,
            leveraged_money_positions_long,
            leveraged_money_positions_short,
            leveraged_money_positions_spread,
            other_rept_positions_long,
            other_rept_positions_short,
            other_rept_positions_spread)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $12, $13, $14, $15, $16, $17, $18, $19);"#
    )
    .bind(data.market_exchange_names)
    .bind(data.report_date)
    .bind(data.cftc_contract_market_code)
    .bind(data.open_interest)
    .bind(data.dealer_positions_long)
    .bind(data.dealer_positions_short)
    .bind(data.dealer_positions_spread)
    .bind(data.asset_manager_positions_long)
    .bind(data.asset_manager_positions_short)
    .bind(data.asset_manager_positions_spread)
    .bind(data.leveraged_money_positions_long)
    .bind(data.leveraged_money_positions_short)
    .bind(data.leveraged_money_positions_spread)
    .bind(data.other_rept_positions_long)
    .bind(data.other_rept_positions_short)
    .bind(data.other_rept_positions_spread)
    .bind(data.market_exchange_names)
    .bind(data.market_exchange_names)
    .bind(data.market_exchange_names)
    .execute(pool)
    .await?;

    Ok(())
}