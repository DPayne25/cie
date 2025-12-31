mod ingestion;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    ingestion::cot_fetch::fetch_cot_data().await?;

    ingestion::csv_validate::validate_csv()?;  
    
    ingestion::csv_rename::rename_csv()?;

   println!("Mission Complete: Ingestion Completed Successfully.");


    Ok(())
} 