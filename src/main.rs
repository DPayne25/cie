mod ingestion;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    ingestion::fetch::fetch_cot_data().await?;

    ingestion::csv_logic::csv_validate()?;  
    
    ingestion::csv_logic::rename_csv()?;

   println!("Mission Complete: Ingestion Completed Successfully.");


    Ok(())
} 