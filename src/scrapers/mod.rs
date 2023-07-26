pub mod test_scraper;
pub mod weather_scraper;
pub mod youtube_scraper;

use crate::async_trait::async_trait;

#[async_trait]
pub trait Scraper {
  async fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}