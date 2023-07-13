use scraper::{Html, Selector, ElementRef};
pub struct TestScraper {
    url: String
}

impl TestScraper {

    pub fn from_url(url: String) -> Self {
        Self { url }
    }

    pub async fn crawl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let html = self.get_response().await?;

        let selector: Selector = Selector::parse(".thumbnail")?;

        let selected = html.select(&selector);

        // println!("{:?}", selected);

        for item in selected {
            let select_name = Selector::parse(".caption h4")?;

            let parsed_items = item.select(&select_name).map(|item| item.inner_html()).collect::<Vec<String>>();
            let price = &parsed_items[0];
            let mut price_mod = price.chars();
            price_mod.next();
            let price_num: f64 = price_mod.as_str().parse()?;
            let name = Html::parse_document(&parsed_items[1])
                .select(&Selector::parse("a")?).next().unwrap().inner_html();
            println!("price: {}, name: {}", price_num, name.trim());
        }

        Ok(())
    }

    async fn get_response(&self) -> Result<Html, Box<dyn std::error::Error>> {
        let response = reqwest::get(&self.url)
            .await?
            .text()
            .await?;
        
        Ok(Html::parse_document(&response))
    }
}