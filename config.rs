struct SentinelConfig {

     oanda_api_key: String,
     database_url: String

}

impl SentinelConfig { 

    pub fn from_env() -> Result< Self, Box<dyn std::error::Error>> {

        dotenv::from_filename(".env").ok();

        let oanda_api_key = env::var("OANDA_API_KEY")?;
        let database_url = env::var("DATABASE_URL")?;

         Ok(Self {oanda_api_key: oanda_api_key, database_url: database_url})

    }
}