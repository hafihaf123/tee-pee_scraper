use crate::objects::builders::ObjectBuilder;
use crate::utils::create_selector;
use crate::{Object, TeePeeClient};
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::IntoUrl;
use scraper::{ElementRef, Html, Selector};
use std::fmt::Debug;

/// Fetches the HTML content from a given URL.
///
/// # Type Parameters
///
/// * `U` - A type that can be converted into a URL and implements `Copy` and `Debug`.
///
/// # Arguments
///
/// * `client` - A reference to the `TeePeeClient` used to make the request.
/// * `parent_unit_url` - The URL from which to fetch the HTML content.
///
/// # Returns
///
/// A `Result` containing the parsed `Html` document if successful,
/// or an error if the request fails.
pub(super) fn fetch_html<U: IntoUrl + Copy + Debug>(
    client: &TeePeeClient,
    parent_unit_url: U,
) -> Result<Html> {
    Ok(Html::parse_document(&client.get(parent_unit_url)?))
}

/// Extracts an ID from a menu element using a given selector.
///
/// # Arguments
///
/// * `menu_element` - The `ElementRef` representing the menu element.
/// * `id_selector` - A reference to the `Selector` used to find the ID element.
///
/// # Returns
///
/// A `Result` containing the extracted ID as a `u32` if successful,
/// or an error if the ID cannot be found or parsed.
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

/// Extracts a name from a menu element using a given selector.
///
/// # Arguments
///
/// * `menu_element` - The `ElementRef` representing the menu element.
/// * `name_selector` - A reference to the `Selector` used to find the name element.
///
/// # Returns
///
/// A `Result` containing the extracted name as a `String` if successful,
/// or an error if the name cannot be found.
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

/// Scrapes data from a given URL and populates a container with objects of type `T`.
///
/// # Type Parameters
///
/// * `U` - A type that can be converted into a URL and implements `Copy` and `Debug`.
/// * `T` - The type of object to be created. Must implement the `Object` trait.
///
/// # Arguments
///
/// * `client` - A reference to the `TeePeeClient` used to make the request.
/// * `url` - The URL from which to scrape data.
/// * `selectors` - An array of selectors used to find the relevant elements.
/// * `container` - A mutable reference to a vector that will be populated with the scraped objects.
///
/// # Returns
///
/// A `Result` indicating success or failure of the scraping operation.
pub(super) fn scrape_from_url<U: IntoUrl + Copy + Debug, T: Object>(
    client: &TeePeeClient,
    url: U,
    selectors: [&str; 3],
    container: &mut Vec<T>,
) -> Result<()> {
    let html = fetch_html(client, url)?;

    let outer_selector = create_selector(selectors[0])?;
    let name_selector = create_selector(selectors[1])?;
    let id_selector = create_selector(selectors[2])?;

    for unit_element in html.select(&outer_selector) {
        let mut builder = T::builder();

        builder.id(extract_id(unit_element, &id_selector)?);
        builder.name(&extract_name(unit_element, &name_selector)?);

        container.push(builder.build()?);
    }

    Ok(())
}
