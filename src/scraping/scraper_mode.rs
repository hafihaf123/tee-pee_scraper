use crate::Object;

/// A trait that defines the mode in which a scraper operates.
///
/// # Type Parameters
///
/// * `T` - The type of object that the scraper will produce. Must implement the `Object` trait.
pub trait ScraperMode<T: Object> {}
