use crate::object::builder::ObjectBuilder;
use crate::object::Object;
use crate::scraping::scraper_mode::ScraperMode;
use anyhow::Result;

pub trait Scraper<T: Object<B>, B: ObjectBuilder<T>, M: ScraperMode<T, B>> {
    fn scrape(&mut self, mode: M) -> Result<Vec<T>>;
}
