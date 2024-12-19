pub mod authentication;

pub mod object;

pub mod scraping;

mod teepee;

use anyhow::anyhow;
use scraper::Selector;
pub use teepee::TeePeeClient;

pub fn create_selector(selectors: &str) -> anyhow::Result<Selector> {
    Ok(match Selector::parse(selectors) {
        Ok(selector) => selector,
        Err(e) => {
            return Err(anyhow!("Parsing a selector failed: {}", e));
        }
    })
}
