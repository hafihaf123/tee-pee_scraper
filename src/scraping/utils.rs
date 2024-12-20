use crate::objects::builders::ObjectBuilder;
use crate::{Object, TeePeeClient};
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::IntoUrl;
use scraper::{ElementRef, Html, Selector};
use std::fmt::Debug;

pub(super) fn fetch_html<U: IntoUrl + Copy + Debug>(
    client: &TeePeeClient,
    parent_unit_url: U,
) -> Result<Html> {
    Ok(Html::parse_document(&client.get(parent_unit_url)?))
}

pub(super) fn extract_id(menu_element: ElementRef, id_selector: &Selector) -> Result<u32> {
    let re = Regex::new(r"/\w+/(\d+)/detail")?;

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

pub(super) fn extract_name(menu_element: ElementRef, name_selector: &Selector) -> Result<String> {
    menu_element
        .select(name_selector)
        .next()
        .and_then(|name_element| name_element.text().next())
        .map_or_else(
            || Err(anyhow!("Could not find name")),
            |name| Ok(name.into()),
        )
}

pub(super) fn scrape_from_url<U: IntoUrl + Copy + Debug, T: Object<B>, B: ObjectBuilder<T>>(
    client: &TeePeeClient,
    url: U,
    selectors: [&str; 3],
    container: &mut Vec<T>,
) -> Result<()> {
    let html = fetch_html(client, url)?;

    let outer_selector = crate::utils::create_selector(selectors[0])?;
    let name_selector = crate::utils::create_selector(selectors[1])?;
    let id_selector = crate::utils::create_selector(selectors[2])?;

    for unit_element in html.select(&outer_selector) {
        let mut builder = T::builder();

        builder.id(extract_id(unit_element, &id_selector)?);
        builder.name(&extract_name(unit_element, &name_selector)?);

        container.push(builder.build()?);
    }

    Ok(())
}
