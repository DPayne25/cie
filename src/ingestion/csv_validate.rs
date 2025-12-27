use std::{fs, error::Error, io, process};



fn validate_csv() -> Result<(), Box<dyn Error>> {
    let file_path = "data/raw/cot/date-cot_report.csv"; // #todo change to dynamic path (most recent file)
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        // Check if the record contains the expected TFF codes
        let tff_code = &record[7]; // TFF code is in the 8th column (index 7)
        let valid_tff_codes = vec!["099741", "097741", "096741", "232741", "090741", "092741", "112741"];
        if !valid_tff_codes.contains(&tff_code) {
            return Err(Box::new::io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid TFF code found: {}", tff_code),
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;
    println!("{}", resp);
    fs::write("data/raw/cot/date-cot_report.csv", resp)?;
    
    validate_csv()?;
    
    Ok(())
} 