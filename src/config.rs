/*use chrono::{DateTime, Utc, NaiveDate};
use std::env;
use dotenvy::dotenv;
*/
pub struct SentinelConfig {

     oanda_api_key: String,
     oanda_host: String,
     database_url: String,
     default_start_date: DateTime<Utc>, 

}

impl SentinelConfig { 

    pub fn from_env() -> Result< Self, Box<dyn std::error::Error>> {

        dotenv::from_filename(".env").ok();

        let oanda_api_key = env::var("OANDA_API_KEY")?;
        let oanda_host = env::var("OANDA_HOST")?;
        let database_url = env::var("DATABASE_URL")?;

        let date_parse = NaiveDate::parse_from_str(&env::var("DEFAULT_START_DATE")?, "%Y-%m-%d")
            .expect("Invalid date format. Use 'YYYY-MM-DD' in `/.env`.");
        let date_time = date_parse.and_hms_opt(0, 0, 0)
            .expect("Invalid time components. Use 'HH, MM, SS'.");
        let default_start_date_utc = date_time.and_utc();

         Ok(Self {oanda_api_key: oanda_api_key, oanda_host: oanda_host, database_url: database_url, default_start_date: default_start_date})

    }
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