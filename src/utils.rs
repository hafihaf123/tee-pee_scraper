use anyhow::anyhow;
use scraper::Selector;

/// Convenience method for creating a [`Selector`].
///
/// Converts the Error type of [`Selector::parse`] to [`anyhow::Error`] with a bit of
/// additional context
pub(crate) fn create_selector(selectors: &str) -> anyhow::Result<Selector> {
    Ok(match Selector::parse(selectors) {
        Ok(selector) => selector,
        Err(e) => {
            return Err(anyhow!("Parsing a selector failed: {}", e));
        }
    })
}
