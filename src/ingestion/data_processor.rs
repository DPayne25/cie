/*use std::{error::Error, format, fs, fs::File, collections::HashSet, path::Path, };
use csv::{ReaderBuilder, Reader, WriterBuilder, Writer};

const INPUT_PATH: &str = "data/raw/cot/date-RawCOTReport.csv";


pub fn process_cot_report() -> Result<(), Box<dyn Error>> {
    let target_tff_codes: HashSet<&str> = ["090741","092741", "096742", "097741", "099741", "232741", "112741"]
        .iter()
        .collect();

    let mut reader = ReaderBuilder::new().has_headers(false).from_path(INPUT_PATH)?;

    let mut valid_records = Vec::new();
    let mut extracted_date: Option<String> = None;

    for (i, result) in reader.records().enumerate() {
        let record = result?;

        if extracted_date.is_none() && record.len() > 2 {
            extracted_date = Some(record[2].to_string());
        }

        if let Some(code) = record.get(3) {
            if target_tff_codes.contains(code) {
                valid_records.push(record);
            }
        }
    }

    let date_str = extracted_date.ok_or("• Critical: No date found in the COT report.");

    let raw_archive_path: String = format!("data/raw/cot/{}-RawCOTReport.csv", date_str);
    let processed_path: String = format!("data/processed/cot/{}-ProcessedCOTReport.csv", date_str);
    Ok(())
}
*/