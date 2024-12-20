pub mod authentication;
#[doc(inline)]
pub use authentication::Credentials;

pub mod objects;
#[doc(inline)]
pub use objects::Object;

pub mod scraping;
#[doc(inline)]
pub use scraping::Scraper;

mod teepee;
pub use teepee::TeePeeClient;

mod utils;
pub(crate) use utils::create_selector;
