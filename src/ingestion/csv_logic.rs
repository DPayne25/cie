use std::{format, fs, fs::File, path::Path};
#[allow(unused_imports)]
use csv::{ReaderBuilder, Reader, WriterBuilder, Writer};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use crate::ingestion::errors::IngestionError;


static TFF_CODES: Lazy <HashSet<&'static str>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert("090741");
    s.insert("092741");
    s.insert("096742");
    s.insert("097741");
    s.insert("099741");
    s.insert("232741");
    s.insert("112741");
    s
});

pub fn extract_cot_date(file_path: &Path) -> Result<String, IngestionError> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let record = reader.record()
        .nth(1)
        .ok_or_else(|| IngestionError::MissingData(
            format!("No date found in the third column of the second rose in COT file: {}", file_path.display())
        ))?;

    if cot_date.is_empty() || cot_date.len() < 8 {
        return Err(IngestionError::InvalidDate(format!("Extracted COT date '{}' seems invalid.", cot_date)));
    }

    Ok(cot_date.to_string())
}

pub fn csv_validate(input_path: &Path) -> Result<(), IngestionError> {

    let file = File::open(input_path)?;

    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut valid_codes_found = false;

    for(i, result) in reader.records().enumerate() {
        match result {
            Ok(record) => {
                if let Some(tff_code) = record.get(3) {
                    if TFF_CODES.contains(&tff_code) {
                        println!("• TFF code {} found in row {}", tff_code, i + 1); 

                        valid_codes_found = true;                        
                    }
                }
            }
            Err(e) => {
                eprintln!("• Error reading row {}: {}", i + 1, e);
                return Err(IngestionError::Csv(e));
            }
        }
    }

    if !valid_codes_found {
        return Err(IngestionError::Validation(
            format!("• No vlid Tff codes found in file: {}", input_path.display())));
    }

    println!("• Successful CSV validation. Valid TFF codes are available.");
    Ok(())
}


pub fn csv_process_raw_cot(input_path: &Path, output_path: &Path) -> Result<(), IngestionError> {

    let input_file = File::open(input_path);
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(input_file);

    let mut writer = WriterBuilder::new().from_path(output_path)?;

    for result in reader.records() {
        let record = result?;
        let tff_code = record.get(3).ok_or_else(|| {
            IngestionError::Validation(format!("Missing TFF code in record: {:?}", record))
        })?;
        if TFF_CODES.contains(tff_code) {
            writer.write_record(&record)?;
        }
    }

    writer.flush()?;
    println!("• Processing successful. Filtered data saved to {}", output_path.display());
    Ok(())
}


pub fn finalize_cot_filenames(
    raw_temp_path: &Path,
    raw_final_path: &Path,
    processed_temp_path: &Path,
    processed_final_path: &Path
) -> Result<(), IngestionError> {
    if let Some(parent) = raw_final_path.parent() {
        fs::create_dir_all(parent).map_err(|e| IngestionError::Io)?;
    }    

    if let Some(parent) = processed_final_path.parent() {
        fs::create_dir_all(parent).map_err(|e| IngestionError::Io)?;
    }

    fs::rename(raw_temp_path, raw_final_path)
        .map_err(|e| IngestionError::FsOperation(format!("• Failed to rename raw COT file from {} to {}: {}", raw_temp_path.display(), raw_final_path, e)))?;

    fs::rename(processed_temp_path, processed_final_path)
        .map_err(|e| IngestionError::FsOperation(format!("• Failed to rename processed COT file from {} to {}: {}", processed_temp_path.display(), processed_final_path, e)))?;

    println!("• Successfully finalized raw COT file to {}", raw_final_path.display());

    println!("• Successfully finalized processed COT file to {}", processed_final_path.display());

    Ok(())
}