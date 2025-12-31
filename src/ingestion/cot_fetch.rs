#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let cot_data = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;

    println!("{}", cot_data);

    cot_date = cot_data.lines().nth(3);

    fs::write("data/raw/cot/{}-cot_report.csv", cot_date, cot_data)?;
    
    Ok(())
} 