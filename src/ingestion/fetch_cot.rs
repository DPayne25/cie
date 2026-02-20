use std::{fs::{self, File}, error::Error, path::Path, io};
use zip::ZipArchive;



pub async fn fetch_cot_data(year: i64) -> Result<String, Box<dyn Error>> {
    
    let url = format!("https://www.cftc.gov/files/dea/history/fut_fin_txt_{}.zip", year);

    println!("• Fetching COT data from: {}", url);

    let response_bytes = reqwest::get(&url).await?.bytes().await?;
    let cursor = io::Cursor::new(response_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let mut file_in_zip = archive.by_index(0)
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

