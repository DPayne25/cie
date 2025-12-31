use std::{fs, format, error::Error};

pub fn rename_csv() -> Result<(), Box<dyn Error>> {
    let path = "data/raw/cot/date-cot_report.csv";
    let mut reader = csv::Reader::from_path(path)?;

    let record = reader.records().nth(1).ok_or("Error: No second row found")??;

    let cot_date = record.get(2).ok_or("Error: No date found in the third column")?;

    let new_path = format!("data/raw/cot/{}-cot_report.csv", cot_date);

    fs::rename(path, &new_path)?;

    // Cofirm rename
    println!("Successfully renamed file to {}", new_path); 

    Ok(())
}