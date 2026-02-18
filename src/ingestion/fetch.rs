use std::{fs::{self, File}, env, error::Error, path::Path, io};
use chrono::{DateTime, Utc, TimeZone};
use dotenvy::dotenv;
use fxoanda;
use zip::ZipArchive;



pub async fn fetch_cot_data(year: i64) -> Result<String, Box<dyn Error>> {
    
    let url = format!("https://www.cftc.gov/files/dea/history/fut_fin_txt_{}.zip", year);

    println!("• Fetching COT data from: {}", url);

    let response_bytes = reqwest::get(&url).await?.bytes().await?;
    let cursor = io::Cursor::new(response_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let mut file_in_zip = archive.by_name("FinFutWk.txt")
        .map_err(|_| format!("Could not find 'FinFutWk.txt' in archive for year {}", year))?;

    let out_dir = Path::new("data/raw/cot");
    fs::create_dir_all(out_dir)?;

    let temp_path_str = "data/raw/cot/date-RawCOTReport.csv";
    let out_path = Path::new(temp_path_str);
    let mut outfile = File::create(&out_path)?;

    io::copy(&mut file_in_zip, &mut outfile)?;

    println!("• Successfully extracted and saved raw COT data to: {:?}", out_path);

    Ok(temp_path_str.to_string())
} 


pub async fn fetch_fx_price_data(
    client: &reqwest::Client,
    instrument: &str, 
    //from_date: DateTime<Utc>, 
    granularity: &str, 
    price_type: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap(); //todo line 23

    dotenv().ok();

    let api_key = env::var("OANDA_API_KEY").map_err(|_| "OANDA_API_KEY not set in .env file")?;

    let oanda_client= fxoanda::Client {
        host: "api-fxtrade.oanda.com".to_string(),
        reqwest: client.clone(),
        authentication: api_key.to_string()
    };

    //let from_date_str = from_date.to_rfc3339();

    let get_data = fxoanda::GetInstrumentCandlesRequest::new()
        .with_instrument(instrument.to_string())
        .with_from(dt)
        .with_granularity(fxoanda::CandlestickGranularity::D)
        .with_price(price_type.to_string())
        .with_count(5000)
        .remote(&oanda_client).await;



    let fx_data = get_data?;

    let candles = fx_data.candles;

    let json = serde_json::to_string_pretty(&candles)?;

    fs::write(format!("data/raw/fx_prices/{}_{}_RawFXPriceData.json", instrument, granularity), json)?;

    Ok(())
} 