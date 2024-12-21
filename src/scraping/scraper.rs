use crate::scraping::scraper_mode::ScraperMode;
use crate::Object;
use anyhow::Result;

pub trait Scraper<T: Object, M: ScraperMode<T>> {
    fn scrape(&mut self, mode: M) -> Result<Vec<T>>;
}
