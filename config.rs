use serde::Deserialize;



struct SentinelConfig {

     oanda_api_key: String,
     oanda_host: String,
     database_url: String,
     default_start_date: DateTime<Utc>


}

impl SentinelConfig { 

    pub fn from_env() -> Result< Self, Box<dyn std::error::Error>> {

        dotenv::from_filename(".env").ok();

        let oanda_api_key = env::var("OANDA_API_KEY")?;
        let oanda_host = env::var("OANDA_HOST")?;
        let database_url = env::var("DATABASE_URL")?;

        let date_parse = NaiveDate::parse_from_str(&env::var("DEFAULT_START_DATE")?, "%Y-%m-%d")
            .expect("Invalid date format. Use 'YYYY-MM-DD' in `/.env`.");
        let date_time = date_parse.and_hms(0, 0, 0)
            .expect("Invalid time components. Use 'HH, MM, SS'.");
        let default_start_date_utc = date_time.and_utc();

         Ok(Self {oanda_api_key: oanda_api_key, oanda_host: oanda_host, database_url: database_url, default_start_date: default_start_date})

    }
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