use reqwest::IntoUrl;
use std::fmt::Debug;
use scraper::{Html, Selector};
use anyhow::anyhow;
use crate::TeePeeClient;

pub(super) fn fetch_html<U: IntoUrl + Copy + Debug>(client: &TeePeeClient, parent_unit_url: U) -> anyhow::Result<Html> {
    Ok(Html::parse_document(&client.get(parent_unit_url)?))
}

pub(super) fn create_selector(selectors: &str) -> anyhow::Result<Selector> {
    Ok(match Selector::parse(selectors) {
        Ok(selector) => selector,
        Err(e) => {
            return Err(anyhow!("Parsing a selector failed: {}", e));
        }
    })
}