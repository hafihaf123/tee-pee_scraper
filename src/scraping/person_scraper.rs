use crate::object::builder::{ObjectBuilder, PersonBuilder};
use crate::object::{Object, Person, Unit};
use crate::scraping::scraper_mode::ScraperMode;
use crate::scraping::utils::{create_selector, extract_id, extract_name, fetch_html};
use crate::scraping::Scraper;
use crate::TeePeeClient;
use anyhow::Result;
use PersonScraperMode::FromUnit;

pub enum PersonScraperMode {
    FromUnit(Unit),
}

impl ScraperMode<Person, PersonBuilder> for PersonScraperMode {}

pub struct PersonScraper {
    client: TeePeeClient,
}

impl PersonScraper {
    pub fn new(client: &TeePeeClient) -> Self {
        Self {
            client: client.clone(),
        }
    }
}

impl Scraper<Person, PersonBuilder, PersonScraperMode> for PersonScraper {
    fn scrape(&mut self, mode: PersonScraperMode) -> Result<Vec<Person>> {
        match mode {
            FromUnit(unit) => self.scrape_from_unit(unit),
        }
    }
}

impl PersonScraper {
    fn scrape_from_unit(&self, unit: Unit) -> Result<Vec<Person>> {
        let unit_persons_url = format!(
            "https://skauting.tee-pee.com/units/{}/detail#persons",
            unit.id()
        );
        let html = fetch_html(&self.client, &unit_persons_url)?;

        let outer_selector =
            create_selector("div#orgUnitDetailsTabViewId\\:orgUnitPersonGridId_content div.ui-g")?;
        let id_selector = create_selector("a.ui-link.ui-widget")?;
        let name_selector = create_selector("span.ListItemName")?;

        let mut unit_persons = Vec::new();

        for person_element in html.select(&outer_selector) {
            let mut person_builder = Person::builder();

            person_builder.id(extract_id(person_element, &id_selector)?);
            person_builder.name(&extract_name(person_element, &name_selector)?);

            unit_persons.push(person_builder.build()?);
        }

        Ok(unit_persons)
    }
}
