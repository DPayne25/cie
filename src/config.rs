use chrono::{DateTime, Utc, NaiveDate};
use std::env;
use dotenv;
use thiserror::Error;

// SentinelConfig
pub struct SentinelConfig {

    pub oanda_api_key: String,
    pub oanda_host: String,
    pub database_url: String,
    pub default_start_date: DateTime<Utc>, 

}

impl SentinelConfig { 

    pub fn from_env() -> Result< Self, ConfigError> {

        dotenv::from_filename(".env").ok();

        let oanda_api_key = env::var("OANDA_API_KEY")?;
        let oanda_host = env::var("OANDA_HOST")?;
        let database_url = env::var("DATABASE_URL")?;

        let date_str = env::var("DEFAULT_START_DATE")?;
        let naive_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?;
        let naive_date_time = naive_date.and_hms_opt(0,0,0)
            .ok_or(ConfigError::DateTimeConstruction)?;
        let default_start_date = naive_date_time.and_utc();

         Ok(Self {oanda_api_key: oanda_api_key, oanda_host: oanda_host, database_url: database_url, default_start_date: default_start_date})

    }
}


//Error Handling with thiserror crate
#[derive(Error, Debug)]
pub enum ConfigError { 
    #[error("Environment variable not found: {0}")]
    MissingVariabl(#[from] std::env::VarError),
    #[error("Invalid date format for DEFAULT_START_DATE: {0}. Expected 'YYYY-MM-DD'.")]
    InvalidDateFormat(#[from] chrono::ParseError),
    #[error("Internal error: Could not construct date/time object.")]
    DateTimeConstruction,
}



/* May need for Non-recompiler build version
#[derive(Deserialize, Debug)]

struct Instruments {
    #[serde(rename = "099741")]
    EURUSD: String,
    #[serde(rename = "097741")]
    USDJPY: String,
    #[serde(rename = "096742")]
    GBPUSD: String,
    #[serde(rename = "232741")]
    AUDUSD: String,
    #[serde(rename = "090741")]
    USDCAD: String,
    #[serde(rename = "092741")]
    USDCHF: String,
    #[serde(rename = "112741")]
    NZDUSD: String,
    #[serde(rename = "095741")]
    USDMXN: String,
    #[serde(rename = "120741")]
    USDBRL: String,
    #[serde(rename = "216742")]
    USDZAR: String,
    #[serde(rename = "233741")]
    BTCUSD: String, 
}
*/