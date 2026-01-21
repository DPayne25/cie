pub struct AppConfig {
    pub oanda_api_key: String,
    pub db_url: String
}

impl AppConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        dotenvy().ok();
        Ok(Self {
            oanda_api_key: std::env::var("OANDA_API_KEY")?,
            db_url: std::env::var("DATABASE_URL")?
        })
    }
}