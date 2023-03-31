use log::Level;

use sqlx::PgConnection;
use sqlx::Row;

mod dbconfig;
use dbconfig::connection::{create_connection_pool};




fn logger() {
    simple_logger::init_with_level(Level::Info).unwrap();
}


async fn run() -> Result<(), Box<dyn std::error::Error>>{
    logger();   
    let pool = create_connection_pool("datasource").await?;
    let mut conn = pool.acquire().await?;


    let sql = "select 1+1 as sum";
    query(&mut conn, sql).await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await?;
    Ok(()) 
}


async fn query(conn: &mut PgConnection, sql: &str) -> Result<(), Box<dyn std::error::Error>> {


    let res = sqlx::query(sql)
        .fetch_one(conn)
        .await?;
    
    let sum: i32 = res.get("sum");
    println!("sum: {}", sum);

    Ok(())
}