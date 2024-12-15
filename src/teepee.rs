use crate::authentication::{Credentials, LoginForm};
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use reqwest::{IntoUrl, Url};
use scraper::{Html, Selector};
use std::fmt::Debug;
use serde::Serialize;

pub struct TeePeeClient {
    client: Client,
}

fn extract_view_state(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = match Selector::parse("input[name=\"javax.faces.ViewState\"]") {
        Ok(selector) => selector,
        Err(_) => return None,
    };
    document
        .select(&selector)
        .next()
        .and_then(|input| input.value().attr("value"))
        .map(|v| v.to_string())
}

impl TeePeeClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    pub fn login(&self, credentials: &Credentials) -> Result<()> {
        let login_url = Url::parse("https://skauting.tee-pee.com/login")
            .with_context(|| "Failed to parse login url")?;

        let view_state = self.get_view_state(&login_url)?;

        let login_form = LoginForm::from_credentials(credentials, &view_state);

        let login_response_body = self.post_form(&login_url, &login_form)
            .with_context(|| "Sending login POST request failed")?;

        if login_response_body.contains("Nesprávne používateľské meno alebo heslo") {
            return Err(anyhow!("Login Failed"));
        }

        Ok(())
    }
    pub fn get<U: IntoUrl + Clone + Debug>(&self, url: &U) -> Result<String> {
        Ok(self
            .client
            .get(url.clone())
            .send()
            .with_context(|| format!("Failed to send request to '{:?}'", url))?
            .text()
            .with_context(|| format!("Failed to parse response text from '{:?}'", url))?)
    }
    pub fn post_form<U: IntoUrl + Clone + Debug, T: Serialize + ?Sized>(&self, url: &U, form: &T) -> Result<String> {
        Ok(self
            .client
            .post(url.clone())
            .form(form)
            .send()
            .with_context(|| format!("Failed to send request to '{:?}'", url))?
            .text()
            .with_context(|| format!("Failed to parse response text from '{:?}'", url))?)
    }
    fn get_view_state<U: IntoUrl + Clone + Debug>(&self, url: &U) -> Result<String> {
        let page_text = self.get(url)?;

        Ok(extract_view_state(&page_text)
            .with_context(|| format!("Failed to extract view state from page: '{:?}'", url))?)
    }
}

impl Default for TeePeeClient {
    fn default() -> Self {
        Self {
            client: Client::builder()
                .cookie_store(true)
                .build()
                .expect("Failed to build client"),
        }
    }
}
