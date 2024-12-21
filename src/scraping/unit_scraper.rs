use crate::objects::Unit;
use crate::scraping::scraper_mode::ScraperMode;
use crate::scraping::utils::scrape_from_url;
use crate::scraping::{ChildUnits, MyUnits};
use crate::{Object, Scraper, TeePeeClient};
use anyhow::Result;
use indicatif::ProgressBar;
use std::time::Duration;

pub enum UnitScraperMode {
    MyUnits,
    ChildUnits(Unit),
    // AllData(Unit),
}

impl ScraperMode<Unit> for UnitScraperMode {}

pub struct UnitScraper {
    client: TeePeeClient,
}

impl UnitScraper {
    pub fn new(client: &TeePeeClient) -> Self {
        Self {
            client: client.clone(),
        }
    }
}

impl Scraper<Unit, UnitScraperMode> for UnitScraper {
    fn scrape(&mut self, mode: UnitScraperMode) -> Result<Vec<Unit>> {
        let bar = ProgressBar::new_spinner();
        bar.set_message("Scraping...");
        bar.enable_steady_tick(Duration::from_millis(100));

        let result = match mode {
            MyUnits => self.scrape_my_units(),
            ChildUnits(mut parent_unit) => {
                self.scrape_child_units(&mut parent_unit)?;
                Ok(parent_unit.into_child_units())
            } // AllData(unit) => {}
        };

        bar.finish_and_clear();
        result
    }
}

impl UnitScraper {
    fn scrape_my_units(&mut self) -> Result<Vec<Unit>> {
        let mut my_units: Vec<Unit> = Vec::new();

        scrape_from_url(
            &self.client,
            "https://skauting.tee-pee.com/user/profile#data",
            ["li#j_idt51\\:layoutMenu_5 ul li", "a", "a"],
            &mut my_units,
        )?;

        /*let html = fetch_html(
            &self.client,
            "https://skauting.tee-pee.com/user/profile#data",
        )?;

        let outer_selector = create_selector("li#j_idt51\\:layoutMenu_5 ul li")?;
        let inner_selector = create_selector("a")?;

        let mut my_units = Vec::new();

        for unit_element in html.select(&outer_selector) {
            let mut unit_builder = Unit::builders();

            unit_builder.id(extract_id(unit_element, &inner_selector)?);
            unit_builder.name(&extract_name(unit_element, &inner_selector)?);

            my_units.push(unit_builder.build()?);
        }*/

        Ok(my_units)
    }

    fn scrape_child_units(&self, parent_unit: &mut Unit) -> Result<()> {
        scrape_from_url(
            &self.client,
            &format!(
                "https://skauting.tee-pee.com/units/{}/detail#units",
                parent_unit.id()
            ),
            [
                "div#orgUnitDetailsTabViewId\\:j_idt103_content div.ui-g",
                "span.ListItemName",
                "a.ui-link.ui-widget",
            ],
            parent_unit.child_units_mut(),
        )?;

        /*let parent_unit_url = format!(
            "https://skauting.tee-pee.com/units/{}/detail#units",
            parent_unit.id()
        );

        let html = fetch_html(&self.client, &parent_unit_url)?;

        let outer_selector =
            create_selector("div#orgUnitDetailsTabViewId\\:j_idt103_content div.ui-g")?;
        let name_selector = create_selector("span.ListItemName")?;
        let id_selector = create_selector("a.ui-link.ui-widget")?;

        for unit_element in html.select(&outer_selector) {
            let mut unit_builder = Unit::builders();

            unit_builder.name(&extract_name(unit_element, &name_selector)?);
            unit_builder.id(extract_id(unit_element, &id_selector)?);

            parent_unit.add_child_unit(unit_builder.build()?);
        }*/

        Ok(())
    }
}

impl Unit {
    pub fn scrape_child_units(&mut self, scraper: &mut UnitScraper) -> Result<()> {
        scraper.scrape_child_units(self)?;
        Ok(())
    }
}
