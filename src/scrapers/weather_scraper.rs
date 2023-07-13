use scraper::{ Html, Selector };

use std::fmt;

#[derive(Debug)]
pub struct DailyForecast {
    pub day: String, // cambiar a chrono types quizas luego
    pub max_temp: i32,
    pub min_temp: i32,
}

impl DailyForecast {

    pub fn new(day: String, max_temp: i32, min_temp: i32) -> Self {
        Self {
            day,
            max_temp,
            min_temp
        }
    }
}

#[derive(Debug)]
pub struct WeatherScraper {
    url: String,
    html: Html
}

impl WeatherScraper {

    pub fn from_url(url: impl Into<String>) -> Self {
        Self { url: url.into(), html: Html::parse_fragment("<div>Bad Div</div>") }
    }

    pub async fn crawl(self) -> Result<Self, Box<dyn std::error::Error>> {
        let html = self.get_response().await?;
        Ok(Self {
            url: self.url,
            html,
        })
        //let selector = Selector::parse("div [data-testid=TemperatureValue]")?;
        // let selected = html.select(&selector);

        /*let temps: Vec<i32> = html.select(&selector)
            .map(|selected_item| {
                selected_item
                .inner_html()
                .chars()
                .take(2)
                .collect::<String>()
                .parse()
                .unwrap()
            }).collect();

        println!("{:?}", temps);
        Ok(())*/
    }

    // TODO: renombrar a get forecast o algo asi
    pub fn get_details(&self) -> Result<Vec<DailyForecast>, Box<dyn std::error::Error>> {
        // let selector = Selector::parse("[data-testid=DailyForecast]")?;
        let selector = Selector::parse("[data-testid=DetailsSummary]")?;

        Ok(self.html.select(&selector).map(|item| {
            // name of the day (relative)
            let relative_day: String = item.select(&Selector::parse("[data-testid=daypartName]").unwrap()).map(|summary| {
                summary.inner_html()
            }).next().unwrap();

            let main_temp: Vec<i32> = item.select(&Selector::parse("[data-testid=TemperatureValue]").unwrap()).map(|temps| {
                /*
                println!("test: {:?}", temps.inner_html()
                .chars()
                .take(2)
                .collect::<String>());
                */
                
                temps.inner_html()
                .chars()
                .take(2)
                .collect::<String>()
                .parse()
                .unwrap_or(-99999999) // use this value to insert a NULL

            }).collect();

            // [relative_day].to_vec()
            DailyForecast::new(relative_day, main_temp[0].clone(), main_temp[1].clone())
        }).collect())
    }

    async fn get_response(&self) -> Result<Html, Box<dyn std::error::Error>> {
        let response: String = reqwest::get(&self.url)
            .await?
            .text()
            .await?;
    
        Ok(Html::parse_document(&response))
    }
}