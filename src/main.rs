extern crate dotenv;

use dotenv::dotenv;
use std::env;


pub mod scrapers;
pub mod database;
pub mod engine;

use std::sync::Arc;

use scrapers::test_scraper::TestScraper;
use scrapers::weather_scraper::WeatherScraper;

use sea_orm::entity::prelude::*;

use database::connect;
use database::{ daily_forecast, DailyForecast};

use engine::Engine;
/*
fn main() {
    println!("Hello, world!");
}

*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // check db connection
    let connection = Arc::new(connect().await?);

    // let test = DailyForecast::find_by_id(1).one(&connection).await?;

    //let scraper = TestScraper::from_url("https://webscraper.io/test-sites/e-commerce/allinone".to_string());
    //scraper.crawl().await?;

    /*let scraper = WeatherScraper
        ::from_url("https://weather.com/es-ES/tiempo/10dias/l/Tijuana+Baja+California+M%C3%A9xico?canonicalCityId=f6606c94c8b9ce428862073deb5476389640470862452df1536180a2e6f7aa47")
        .crawl().await?;
    println!("{:#?}", scraper.get_details()?);
    */

    let engine = Engine::new(Arc::clone(&connection));
    engine.run().await?;
    Ok(())
}
