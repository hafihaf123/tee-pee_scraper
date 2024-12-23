use crate::objects::builders::ObjectBuilder;
use crate::objects::Unit;
use crate::scraping::scraper_mode::ScraperMode as ScraperModeTrait;
use crate::scraping::utils::{extract_id, extract_name, fetch_html, scrape_object_basics};
use crate::scraping::{ChildUnits, MyUnits};
use crate::utils::create_selector;
use crate::{Object, Scraper, TeePeeClient};
use anyhow::Result;
use indicatif::ProgressBar;
use std::time::Duration;

/// Enum representing the different modes in which the scraper can operate.
pub enum ScraperMode {
    /// Scrape the user's units.
    MyUnits,
    /// Scrape the child units of a given unit.
    ChildUnits(Unit),
    // /// Scrape all data of a given unit.
    // AllData(Unit),
}

impl ScraperModeTrait<Unit> for ScraperMode {}

/// A struct representing a scraper for units.
pub struct UnitScraper {
    client: TeePeeClient,
}

impl UnitScraper {
    /// Creates a new `UnitScraper`.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the `TeePeeClient` used to make requests.
    ///
    /// # Returns
    ///
    /// A new instance of `UnitScraper`.
    #[must_use]
    pub fn new(client: &TeePeeClient) -> Self {
        Self {
            client: client.clone(),
        }
    }
}

impl Scraper<Unit, ScraperMode> for UnitScraper {
    fn scrape(&mut self, mode: ScraperMode) -> Result<Vec<Unit>> {
        let bar = ProgressBar::new_spinner();
        bar.set_message("Scraping...");
        bar.enable_steady_tick(Duration::from_millis(100));

        let result = match mode {
            MyUnits => self.scrape_my_units(),
            ChildUnits(mut parent_unit) => {
                self.scrape_child_units(&mut parent_unit)?;
                Ok(parent_unit.into_child_units())
            }
        };

        bar.finish_and_clear();
        result
    }
}

impl UnitScraper {
    /// Scrapes the user's units.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Unit` objects if successful,
    /// or an error if the scraping fails.
    fn scrape_my_units(&mut self) -> Result<Vec<Unit>> {
        let mut my_units: Vec<Unit> = Vec::new();

        let html = fetch_html(
            &self.client,
            "https://skauting.tee-pee.com/user/profile#data",
        )?;

        let outer_selector = create_selector("li#j_idt51\\:layoutMenu_5 ul li")?;
        let inner_selector = create_selector("a")?;

        for unit_element in html.select(&outer_selector) {
            let mut builder = Unit::builder();

            builder.id(extract_id(unit_element, &inner_selector)?);
            builder.name(&extract_name(unit_element, &inner_selector)?);

            my_units.push(builder.build()?);
        }

        Ok(my_units)
    }

    /// Scrapes the child units of a given parent unit.
    ///
    /// # Arguments
    ///
    /// * `parent_unit` - A mutable reference to the parent `Unit` whose
    ///   child units will be scraped.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the scraping operation.
    fn scrape_child_units(&self, parent_unit: &mut Unit) -> Result<()> {
        scrape_object_basics(
            &self.client,
            &format!(
                "https://skauting.tee-pee.com/units/{}/detail#units",
                parent_unit.id()
            ),
            ["table.Wid100", "span.ListItemName", "a.ui-link.ui-widget"],
            parent_unit.child_units_mut(),
        )?;

        Ok(())
    }
}

impl Unit {
    /// Scrapes the child units of the current unit using the provided scraper.
    ///
    /// # Arguments
    ///
    /// * `scraper` - A mutable reference to the `UnitScraper` used to scrape the child units.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the scraping operation.
    ///
    /// # Errors
    ///
    /// - If the scraping operation fails.
    pub fn scrape_child_units(&mut self, scraper: &mut UnitScraper) -> Result<()> {
        scraper.scrape_child_units(self)
    }
}
