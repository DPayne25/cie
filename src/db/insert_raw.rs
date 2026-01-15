/*
use sqlx::{PgPool};

pub async fn insert_raw_cot(pool: &PgPool, data: &Vec) -> Result<(), Box<dyn std::error::Error>>{

    let db_url: String = std::env::var("DATABASE_URL")?;
    let mut data = Vec::new();

    for report in rows{
        data = sqlx::query!("INSERT INTO ").execute(pool).await?;

    }
    

    println!("{}", data);

    Ok(())
}
*/