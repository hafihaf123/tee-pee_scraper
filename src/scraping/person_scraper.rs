use crate::objects::{Person, Unit};
use crate::scraping::scraper_mode::ScraperMode as ScraperModeTrait;
use crate::scraping::utils::scrape_object_basics;
use crate::{Object, Scraper, TeePeeClient};
use anyhow::Result;
use indicatif::ProgressBar;
use std::time::Duration;
use ScraperMode::FromUnit;

pub enum ScraperMode {
    FromUnit(Unit),
}

impl ScraperModeTrait<Person> for ScraperMode {}

pub struct PersonScraper {
    client: TeePeeClient,
}

impl PersonScraper {
    #[must_use]
    pub fn new(client: &TeePeeClient) -> Self {
        Self {
            client: client.clone(),
        }
    }
}

impl Scraper<Person, ScraperMode> for PersonScraper {
    fn scrape(&mut self, mode: ScraperMode) -> Result<Vec<Person>> {
        let bar = ProgressBar::new_spinner();
        bar.set_message("Scraping...");
        bar.enable_steady_tick(Duration::from_millis(100));

        let result = match mode {
            FromUnit(mut unit) => {
                self.scrape_from_unit(&mut unit)?;
                unit.into_persons()
            }
        };

        bar.finish_and_clear();
        Ok(result)
    }
}

impl PersonScraper {
    fn scrape_from_unit(&self, unit: &mut Unit) -> Result<()> {
        scrape_object_basics(
            &self.client,
            &format!(
                "https://skauting.tee-pee.com/units/{}/detail#persons",
                unit.id()
            ),
            [
                "div.ui-panel-content.ui-widget-content",
                "span.ListItemName",
                "a.ui-link.ui-widget",
            ],
            unit.persons_mut(),
        )
    }
}

impl Unit {
    pub fn scrape_persons(&mut self, scraper: &mut PersonScraper) -> Result<()> {
        scraper.scrape_from_unit(self)
    }
}
