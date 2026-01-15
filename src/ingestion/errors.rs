use thiserror::Error;

#[derive(Error, Debug)]
pub enum IngetionError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Oanda API error: {0}")]
    Oanda(#[from] fxoanda::Error),
    #[error("JSON serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Data validation error: {0}")]
    Validation(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Missing data error: {0}")]
    MissingData(String),
    #[error("File system operation failed: {0}")]
    FsOperation(String),
    #[error("Invalid date format: {0}")]
    InvalidDate(String)
}