mod scraper;

mod scraper_mode;
pub use scraper::Scraper;

mod unit_scraper;
pub use unit_scraper::ScraperMode::*;
#[doc(inline)]
pub use unit_scraper::UnitScraper;

mod utils;

mod person_scraper;
#[doc(inline)]
pub use person_scraper::PersonScraper;
pub use person_scraper::ScraperMode::*;
