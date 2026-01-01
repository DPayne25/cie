use std::fs;


pub async fn fetch_cot_data() -> Result<(), Box<dyn std::error::Error>> {
    
    let cot_data = reqwest::get("https://www.cftc.gov/dea/newcot/FinFutWk.txt")
        .await?
        .text()
        .await?;

    println!("{}", cot_data);

    fs::write("data/raw/cot/date-cot_report.csv", cot_data)?;

    Ok(())
} 

//pub async fn fetch_fx_price_data() -> Result<(), Box<dyn std::error::Error>> {

//    let fx_data = ;
//}