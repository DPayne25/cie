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

    let response_bytes = reqwest::get(&url).await?.bytes().await?;
    let cursor = io::Cursor::new(response_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let mut file_in_zip = archive
        .by_index(0)
        .map_err(|_| format!("Could not find 'FinFutWk.txt' in archive for year {}", year))?;

    let cot_data: CotTff = ;//insert assignment;

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
    .bind(cot_data.market_exchange_names)
    .bind(cot_data.report_date)
    .bind(cot_data.cftc_contract_market_code)
    .bind(cot_data.open_interest)
    .bind(cot_data.dealer_positions_long)
    .bind(cot_data.dealer_positions_short)
    .bind(cot_data.dealer_positions_spread)
    .bind(cot_data.asset_manager_positions_long)
    .bind(cot_data.asset_manager_positions_short)
    .bind(cot_data.asset_manager_positions_spread)
    .bind(cot_data.leveraged_money_positions_long)
    .bind(cot_data.leveraged_money_positions_short)
    .bind(cot_data.leveraged_money_positions_spread)
    .bind(cot_data.other_rept_positions_long)
    .bind(cot_data.other_rept_positions_short)
    .bind(cot_data.other_rept_positions_spread)
    .bind(cot_data.market_exchange_names)
    .bind(cot_data.market_exchange_names)
    .bind(cot_data.market_exchange_names)
    .execute(db_pool)
    .await?;


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
    pub market_exchange_names: String,
    pub report_date: NaiveDate,
    pub cftc_contract_market_code: String,
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
