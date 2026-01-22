struct OandaConfig {

     api_key: String

}

impl OandaConfig { 

    pub fn from_env() -> Result< Self, Box<dyn std::error::Error>> {

          dotenv::from_filename(".env").ok();

          let key= env::var("OANDA_API_KEY")?;

          Ok(Self {api_key: key})

    }
}