pub mod generic_scraper_engine;

use std::sync::Arc;

use chrono::prelude::*;
use futures::future::join_all;

use sea_orm::{ Database, DbConn, DbErr, Set, ActiveModelTrait };

use crate::database::{daily_forecast, DailyForecast};

use crate::{scrapers::weather_scraper::WeatherScraper };

#[derive(Debug)]
pub struct Engine {
    pub conn: Arc<DbConn>,
    //pub scraper:  WeatherScraper//cambiar a trait luego?
}

impl Engine {
    pub fn new(conn: Arc<DbConn>) -> Self {
        Self {
            conn,
            // scraper
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let scraper = WeatherScraper::from_url("https://weather.com/es-ES/tiempo/10dias/l/Tijuana+Baja+California+M%C3%A9xico?canonicalCityId=f6606c94c8b9ce428862073deb5476389640470862452df1536180a2e6f7aa47")
            .crawl()
            .await?;
        
        let details = scraper.get_details()?;

        /*
        for daily_forecast in details {
            let new_forecast = daily_forecast::ActiveModel {
                day: Set(daily_forecast.day.clone()),
                max_temp: Set(daily_forecast.max_temp),
                min_temp: Set(daily_forecast.min_temp),
                ..Default::default()
            };

            new_forecast.insert(&*self.conn).await?;
        }*/

        let details_future = details.iter().map(|daily_forecast| async {
            let today = Local::now().naive_local(); // BUG: inserts like utc so sometimes is tomorrows date
            let insert_date = match relative_date_into_real(today, &daily_forecast.day) {
                Ok(day) => day,
                Err(_) => today,
            };
            let daily_forecast = daily_forecast::ActiveModel {
                day: Set(insert_date),
                max_temp: Set(daily_forecast.max_temp),
                min_temp: Set(daily_forecast.min_temp),
                register_at: Set(Local::now().naive_local()),
                ..Default::default()
            };
            println!("{:?}", daily_forecast);
            //daily_forecast.insert(&*self.conn).await

            daily_forecast.insert(&*self.conn).await
        });

        join_all(details_future).await.iter().for_each(|resolve| {
            println!("{:?}", resolve);
        });
        

        Ok(())
    }
}

pub fn relative_date_into_real(base_date: NaiveDateTime, relative_into: impl Into<String>)
    -> Result<NaiveDateTime, Box<dyn std::error::Error>> {
        // TODO: take into account year jumps
    let relative_str: String = relative_into.into();

    let base_day = base_date.day();
    let base_month = base_date.month();

    let relative_day = relative_str.split(" ").skip(1).take(1).collect::<String>();

    match relative_day.parse::<u32>() {
        Ok(day) => {
            if day >= base_day {
                Ok(NaiveDateTime::parse_from_str(
                    format!("2023-{}-{} 00:00:00", base_month, day).as_ref(),
                    "%Y-%m-%d %H:%M:%S"
                )?)
            } else {
                Ok(NaiveDateTime::parse_from_str(
                    format!("2023-{}-{} 00:00:00", base_month + 1, day).as_ref(),
                    "%Y-%m-%d %H:%M:%S"
                )?)
            }
        },
        Err(error) => {
            Err(Box::new(error))
        }
    }
}