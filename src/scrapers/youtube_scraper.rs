use scraper::{ Html, Selector };

pub struct YoutubeScraper {
  url: String,
  html: Html,
}

impl YoutubeScraper {

  pub fn from_url(url: impl Into<String>) -> Self {
    Self { url: url.into(), html: Html::parse_fragment("<div>bad Div</div>")}
  }

  pub async fn crawl(self) -> Result<Self, Box<dyn std::error::Error>> {
    let html = self.get_response().await?;

    Ok(Self {
      url: self.url,
      html,
    })
  }

  pub fn test(&self) -> Result<(), Box<dyn std::error::Error>> {
    let selector = Selector::parse("div")?;

    let result: Vec<String> = self.html.select(&selector).map(|item| {
      item.inner_html()
    }).collect();

    println!("{:?}", result);

    Ok(())
  }

  async fn get_response(&self) -> Result<Html, Box<dyn std::error::Error>> {
    let response: String = reqwest::get(&self.url)
      .await?
      .text()
      .await?;

    Ok(Html::parse_document(&response))
  }
}