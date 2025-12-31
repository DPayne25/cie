use std::fs;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let cot_data = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;

    println!("{}", cot_data);

    fs::write("data/raw/cot/date-cot_report.csv", cot_data)?;

    Ok(())
} 