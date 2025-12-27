use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;
    println!("{}", resp);
    fs::write("data/raw/cot/date-cot_report.csv", resp)?;
    Ok(())
} 