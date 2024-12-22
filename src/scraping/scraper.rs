use crate::scraping::scraper_mode::ScraperMode;
use crate::Object;
use anyhow::Result;

/// A trait that defines the behavior of a web scraper.
///
/// # Type Parameters
///
/// * `T` - The type of object that the scraper will produce. Must implement the `Object` trait.
/// * `M` - The mode in which the scraper will operate. Must implement the `ScraperMode` trait for
///   the type `T`.
pub trait Scraper<T: Object, M: ScraperMode<T>> {
    /// Scrapes data based on the provided mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The mode in which the scraper will operate.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of objects of type `T` if successful,
    /// or an error if the scraping fails.
    ///
    /// # Errors
    ///
    /// An error may occur in the following cases:
    /// - the communication with the website fails
    /// - creating selectors or selecting elements fails
    fn scrape(&mut self, mode: M) -> Result<Vec<T>>;
}
