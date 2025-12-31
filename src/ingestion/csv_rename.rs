use std::{fs, format, error::Error};
use csv::Reader;

pub fn rename_csv() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("data/raw/cot/date-cot_report.csv");

    for (row, result) in reader.records().enumerate() {
        let record = result?;

         // Locate the second row (index 1). Second line incase future update includes headers
        if row == 1 {
            // Date value is in third column (index 2)
            let cot_date = &record[2]; 
        }
        else {
            println!("Error: Date not found in expected row.");
        }
        break;
        }

    fs::rename("data/raw/cot/date-cot_report.csv", format!("data/raw/cot/{}-cot_report.csv", cot_date));

    // Cofirm rename
    println!("Successfully renamed file to {}-cot_report.csv", cot_date); 

    Ok(())
}