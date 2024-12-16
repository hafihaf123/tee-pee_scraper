use crate::objects::TeePeeObject;
use crate::scraping::ScraperMode;
use anyhow::Result;

pub trait TeePeeScraper<T: TeePeeObject, M: ScraperMode<T>> {
    fn scrape(&mut self, mode: M) -> Result<Vec<T>>;
}
