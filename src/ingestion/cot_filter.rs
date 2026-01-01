use std::error::Error;
use csv;

pub fn filter_raw_cot() -> Result<(), Box<dyn std::error::Error>> {

    let input_path = "data/raw/cot/date-raw_cot_report.csv";

    let output_path = "data/processed/cot/date-processed_cot_report.csv";

    let mut reader = csv::Reader::from_path(input_path)?;

    let mut writer = csv::Writer::from_path(output_path)?;

    for result in reader.records() {
        let record = result?;
        let tff_code = &record[7];
        let confirm_tff_codes = vec!["099741", "097741", "096741", "232741", "090741", "092741", "112741"];
        if !confirm_tff_codes.contains(&tff_code) {
            writer.write_record(&record)?;
        }
        writer.flush()?;
    }
    println!("Processing successful. Filtered data saved to {}", output_path);
    Ok(())
}