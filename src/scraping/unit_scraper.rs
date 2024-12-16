use crate::objects::Unit;
use crate::scraping::teepee_scraper::TeePeeScraper;
use crate::scraping::utils::{create_selector, fetch_html};
use crate::scraping::ScraperMode;
use crate::TeePeeClient;
use anyhow::Result;
use regex::Regex;

pub enum UnitScraperMode {
    MyUnits,
    ChildUnits(Unit),
}

impl ScraperMode<Unit> for UnitScraperMode {}

pub struct UnitScraper {
    client: TeePeeClient,
    my_units: Vec<Unit>,
}

impl UnitScraper {
    pub fn new(client: &TeePeeClient) -> Self {
        Self {
            client: client.clone(),
            my_units: Vec::new(),
        }
    }
}

impl TeePeeScraper<Unit, UnitScraperMode> for UnitScraper {
    fn scrape(&mut self, mode: UnitScraperMode) -> Result<Vec<Unit>> {
        match mode {
            UnitScraperMode::MyUnits => self.scrape_my_units(),
            UnitScraperMode::ChildUnits(parent_unit) => self.scrape_child_units(parent_unit),
        }
    }
}

impl UnitScraper {
    fn scrape_my_units(&mut self) -> Result<Vec<Unit>> {
        if !self.my_units.is_empty() {
            return Ok(self.my_units.clone());
        }

        let html = fetch_html(&self.client, "https://skauting.tee-pee.com/user/profile#data")?;

        let selector = create_selector("li#j_idt51\\:layoutMenu_5 ul li a")?;

        let re = Regex::new(r"/units/(\d+)/detail")?;

        for unit_element in html.select(&selector) {
            let mut unit_builder = Unit::builder();
            if let Some(unit_link) = unit_element.value().attr("href") {
                if let Some(unit_id_capture) = re.captures(unit_link) {
                    if let Some(unit_id) = unit_id_capture.get(1) {
                        unit_builder.id(unit_id.as_str().parse()?);
                    }
                }
            }
            if let Some(unit_name) = unit_element.text().next() {
                unit_builder.name(unit_name);
            }
            self.my_units.push(unit_builder.build()?);
        }

        Ok(self.my_units.clone())
    }

    fn scrape_child_units(&self, mut parent_unit: Unit) -> Result<Vec<Unit>> {
        let parent_unit_url = format!(
            "https://skauting.tee-pee.com/units/{}/detail#units",
            parent_unit.id()
        );

        let html = fetch_html(&self.client, &parent_unit_url)?;

        let outer_selector = create_selector("div#orgUnitDetailsTabViewId\\:j_idt103_content div.ui-g")?;
        let name_selector = create_selector("span.ListItemName")?;
        let id_selector = create_selector(r#"a[class="ui-link ui-widget"]"#)?;

        let re = Regex::new(r"/units/(\d+)/detail")?;

        for menu_element in html.select(&outer_selector) {
            let mut unit_builder = Unit::builder();

            if let Some(name_element) = menu_element.select(&name_selector).next() {
                if let Some(unit_name) = name_element.text().next() {
                    unit_builder.name(unit_name);
                }
            }

            if let Some(id_element) = menu_element.select(&id_selector).next() {
                if let Some(unit_link) = id_element.value().attr("href") {
                    if let Some(unit_id_capture) = re.captures(unit_link) {
                        if let Some(unit_id) = unit_id_capture.get(1) {
                            unit_builder.id(unit_id.as_str().parse()?);
                        }
                    }
                }
            }
            parent_unit.add_child_unit(unit_builder.build()?);
        }

        Ok(parent_unit.into_child_units())
    }
}

