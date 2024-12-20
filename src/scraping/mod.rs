mod scraper;

mod scraper_mode;
pub use scraper::Scraper;

mod unit_scraper;
#[doc(inline)]
pub use unit_scraper::UnitScraper;
pub use unit_scraper::UnitScraperMode::*;

mod utils;

mod person_scraper;
#[doc(inline)]
pub use person_scraper::PersonScraper;
pub use person_scraper::PersonScraperMode::*;
