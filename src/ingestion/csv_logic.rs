use std::{error::Error, format, fs, fs::File};
#[allow(unused_imports)]
use csv::{ReaderBuilder, Reader, WriterBuilder, Writer};

pub fn csv_validate() -> Result<(), Box<dyn std::error::Error>> {

    let file_path = "data/raw/cot/date-RawCOTReport.csv"; // #todo change to dynamic path (most recent file)

    let file = File::open(file_path)?;

    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

    let confirm_tff_codes = vec!["090741","092741", "096742", "097741", "099741", "232741", "112741"];
    for(i, result) in reader.records().enumerate() {
        match result {
            Ok(record) => {
                if let Some(tff_code) = record.get(3) {
                    if confirm_tff_codes.contains(&tff_code) {
                        println!("• TFF code {} found in row {}", tff_code, i + 1); 
                    }
                }
            }
            Err(e) => eprintln!("• Error reading row {}: {}", i + 1, e)
        }
    }

    println!("• Successful CSV validation. Valid TFF codes are available.");
    Ok(())
}


pub fn rename_csv() -> Result<(), Box<dyn Error>> {
    let path1 = "data/raw/cot/date-RawCOTReport.csv";
    let path2 = "data/processed/cot/date-ProcessedCOTReport.csv";
    let mut reader = csv::Reader::from_path(path1)?;

    let record = reader.records().nth(1).ok_or("Error: No second row found")??;

    let cot_date = record.get(2).ok_or("Error: No date found in the third column")?;

    let new_path1 = format!("data/raw/cot/{}-RawCOTReport.csv", cot_date);
    let new_path2 = format!("data/processed/cot/{}-ProcessedCOTReport.csv", cot_date);

    fs::rename(path1, &new_path1)?;
    fs::rename(path2, &new_path2)?;

    // Confirm rename
    println!("• Successfully renamed file to {}", new_path1); 
    println!("• Successfully renamed file to {}", new_path2); 

    Ok(())
}



pub fn csv_process_raw_cot() -> Result<(), Box<dyn std::error::Error>> {

    let input_path = "data/raw/cot/date-RawCOTReport.csv";

    let output_path = "data/processed/cot/date-ProcessedCOTReport.csv";

    let input_file = File::open(&input_path)?;

    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(input_file);

    let mut writer = csv::Writer::from_path(output_path)?;

    let confirm_tff_codes = vec!["090741","092741", "096742", "097741", "099741", "232741", "112741"];
    

    for result in reader.records() {
        let record = result?;
        let tff_code = &record[3];

        if confirm_tff_codes.contains(&tff_code) {
            writer.write_record(&record)?;
        }
    }
    writer.flush()?;
    println!("• Processing successful. Filtered data saved to {}", output_path);
    Ok(())
}


