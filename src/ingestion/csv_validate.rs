use std::error::Error;



pub fn validate_csv() -> Result<(), Box<dyn std::error::Error>> {

    let file_path = "data/raw/cot/date-cot_report.csv"; // #todo change to dynamic path (most recent file)

    let mut reader = csv::Reader::from_path(file_path)?;

    for result in reader.records() {

        let record = result?;

        

        let tff_code = &record[7]; // TFF code is in the 8th column (index 7)

        let valid_tff_codes = vec!["099741", "097741", "096741", "232741", "090741", "092741", "112741"];

        // Check if the record contains the expected TFF codes
        if !valid_tff_codes.contains(&tff_code) {
            return Err(format!("Invalid TFF code found: {}", tff_code).into());
        }
    }
    println!("CSV validation passed. Valid TFF codes are available.");
    Ok(())
}