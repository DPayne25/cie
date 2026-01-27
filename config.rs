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
            .expect("Invalid date format. Use 'YYYY-MM-DD'.");
        let date_time = date_parse.and_hms(0, 0, 0)
            .expect("Invalid time components. Use 'HH, MM, SS'.");
        let default_start_date = date_time.and_utc();

         Ok(Self {oanda_api_key: oanda_api_key, oanda_host: oanda_host, database_url: database_url, default_start_date: default_start_date })

    }
}