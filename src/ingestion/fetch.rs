use std::{fs, env, io};
use chrono::{DateTime, Utc, TimeZone};
use dotenvy::dotenv;
use fxoanda;

pub async fn fetch_cot_data(year: i32) -> Result<(), Box<dyn Error>> {
    
    // Using bytes in preparation for a future hard disk bypass version
    let cot_request = reqwest::get(format!("https://www.cftc.gov/files/dea/history/fut_fin_txt_{}.zip", year))
        .await?
        .bytes()
        .await?;

    let mut archive = ZipArchive::new(Cursor::from(cot_request))?;
    
    let mut index_zip = None;

    for i in 0..archive.len() {
        
        if let Ok(file) = archive.by_index(i) {
            
            if file.name().ends_with(".txt") {
                index_zip = Some(i);
                break; 
            }
        }
    }

    // 4. Handle the Option result
    let final_index = index_zip.ok_or("※ No .txt file found")?;
    
    let file_in_zip = archive.by_index(final_index)?;

    let cot_data = csv::ReaderBuilder::new()
        .from_reader(file_in_zip);

    let target_tff_codes  = ["090741","092741", "096742", "097741", "099741", "232741", "112741", "095741", "120741", "216742", "233741"];

    let mut reports: Vec<CotReport> = Vec::new();


    for result in cot_data.into_deserialize::<RawCot>() {
        // Handle CSV/Deserialization errors safely
        let record = match result {
            Ok(rec) => rec,
            Err(e) => {
                eprintln!("※ Skipping malformed CSV record: {}", e);
                continue;
            }
        };

        if target_tff_codes.contains(&record.tff_code.as_str()) {
            // Handle Domain Parsing errors safely
            match CotReport::try_from(record) {

                Ok(cot_report) => reports.push(cot_report),

                Err(e) => {

                    eprintln!("※ Data Corruption Detected. Skipping Row: {}", e);
                
                    continue;
                }
            }
        }
    }

    if reports.is_empty() {
        println!("※ Warning: No valid records found for year {}.", year);
        return Ok(())
    }

    println!("• Batching {} records for insertion...", reports.len());

    Ok(())
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