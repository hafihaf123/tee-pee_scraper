use crate::objects::Unit;
use crate::scraping::teepee_scraper::TeePeeScraper;
use crate::scraping::utils::{create_selector, fetch_html};
use crate::scraping::{ChildUnits, MyUnits, ScraperMode};
use crate::TeePeeClient;
use anyhow::{anyhow, Result};
use regex::Regex;
use scraper::{ElementRef, Selector};

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

    pub fn get_my_units(&self) -> &Vec<Unit> {
        &self.my_units
    }

    pub fn into_my_units(self) -> Vec<Unit> {
        self.my_units
    }
}

impl TeePeeScraper<Unit, UnitScraperMode> for UnitScraper {
    fn scrape(&mut self, mode: UnitScraperMode) -> Result<Vec<Unit>> {
        match mode {
            MyUnits => self.scrape_my_units(),
            ChildUnits(mut parent_unit) => {
                self.scrape_child_units(&mut parent_unit)?;
                Ok(parent_unit.into_child_units())
            }
        }
    }
}

impl UnitScraper {
    fn scrape_my_units(&mut self) -> Result<Vec<Unit>> {
        if !self.my_units.is_empty() {
            self.my_units.clear();
        }

        let html = fetch_html(
            &self.client,
            "https://skauting.tee-pee.com/user/profile#data",
        )?;

        let outer_selector = create_selector("li#j_idt51\\:layoutMenu_5 ul li")?;
        let inner_selector = create_selector("a")?;

        for unit_element in html.select(&outer_selector) {
            let mut unit_builder = Unit::builder();

            unit_builder.id(extract_unit_id(unit_element, &inner_selector)?);
            unit_builder.name(&extract_unit_name(unit_element, &inner_selector)?);

            self.my_units.push(unit_builder.build()?);
        }

        Ok(self.my_units.clone())
    }

    fn scrape_child_units(&self, parent_unit: &mut Unit) -> Result<()> {
        let parent_unit_url = format!(
            "https://skauting.tee-pee.com/units/{}/detail#units",
            parent_unit.id()
        );

        let html = fetch_html(&self.client, &parent_unit_url)?;

        let outer_selector =
            create_selector("div#orgUnitDetailsTabViewId\\:j_idt103_content div.ui-g")?;
        let name_selector = create_selector("span.ListItemName")?;
        let id_selector = create_selector("a.ui-link.ui-widget")?;

        for unit_element in html.select(&outer_selector) {
            let mut unit_builder = Unit::builder();

            unit_builder.name(&extract_unit_name(unit_element, &name_selector)?);
            unit_builder.id(extract_unit_id(unit_element, &id_selector)?);

            parent_unit.add_child_unit(unit_builder.build()?);
        }

        Ok(())
    }
}

impl Unit {
    pub fn scrape_child_units(&mut self, scraper: &mut UnitScraper) -> Result<()> {
        scraper.scrape_child_units(self)?;
        Ok(())
    }
}

fn extract_unit_id(menu_element: ElementRef, id_selector: &Selector) -> Result<u32> {
    let re = Regex::new(r"/units/(\d+)/detail")?;

    menu_element
        .select(id_selector)
        .next()
        .and_then(|id_element| id_element.value().attr("href"))
        .and_then(|unit_link| re.captures(unit_link))
        .and_then(|unit_id_capture| unit_id_capture.get(1))
        .map_or_else(
            || Err(anyhow!("Could not find id")),
            |unit_id| unit_id.as_str().parse::<u32>().map_err(|e| anyhow!(e)),
        )
}

fn extract_unit_name(menu_element: ElementRef, name_selector: &Selector) -> Result<String> {
    menu_element
        .select(name_selector)
        .next()
        .and_then(|name_element| name_element.text().next())
        .map_or_else(
            || Err(anyhow!("Could not find name")),
            |name| Ok(name.into()),
        )
}
