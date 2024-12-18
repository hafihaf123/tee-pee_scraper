mod scraper;
mod scraper_mode;
pub use scraper::Scraper;

mod unit_scraper;
pub use unit_scraper::UnitScraper;
pub use unit_scraper::UnitScraperMode::*;

mod utils;

pub mod person_scraper;
pub use person_scraper::PersonScraper;
pub use person_scraper::PersonScraperMode::*;
