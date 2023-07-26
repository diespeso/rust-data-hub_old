use std::sync::Arc;
use std::cell::RefCell;

use crate::scrapers::Scraper;

pub struct GenericScraperEngine {
  pub scraper: Arc<RefCell<dyn Scraper>>
}

impl GenericScraperEngine {
  // TODO: esto e implementar la trait
}