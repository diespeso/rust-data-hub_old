pub mod daily_forecast;

pub use daily_forecast::Entity as DailyForecast;

use dotenv::dotenv;
use std::env;


use sea_orm::{ Database, DbConn, DbErr };

pub async fn connect() -> Result<DbConn, DbErr> {
    //let conn_string = "mysql://localhost:3306/scraper_test";
    let conn_string = format!("mysql://{}/{}", env::var("DATABASE_ADDRESS").unwrap(), env::var("DATABASE_NAME").unwrap());
    println!("{:?}", conn_string);
    let db = Database::connect(conn_string).await?;

    Ok(db)
}