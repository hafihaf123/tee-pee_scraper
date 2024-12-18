use crate::objects::TeePeeObject;
use crate::scraping::scraper_mode::ScraperMode;
use anyhow::Result;

pub trait Scraper<T: TeePeeObject, M: ScraperMode<T>> {
    fn scrape(&mut self, mode: M) -> Result<Vec<T>>;
}
