use crate::authentication::{Credentials, LoginForm};
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use reqwest::IntoUrl;
use scraper::{Html, Selector};
use serde::Serialize;
use std::fmt::Debug;

/// A client used to interact with the <https://skauting.tee-pee.com> site
///
/// # Construction
///
/// It is recommended to use [`TeePeeClient::default()`] to construct an instance.
/// Only use [`TeePeeClient::new()`] when you know what you are doing and look at its documentation first.
/// 
/// # Examples
/// ```
/// # use tee_pee_scraper::authentication::Credentials;
/// use tee_pee_scraper::TeePeeClient;
/// let teepee = TeePeeClient::default();
/// # let credentials = Credentials::new("username").unwrap();
/// # credentials.set_password("pass").unwrap();
/// assert!(teepee.login(&credentials).is_err()) // because wrong credentials were used
/// ```
#[derive(Clone)]
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
        let login_url = "https://skauting.tee-pee.com/login";

        let view_state = self.get_view_state(login_url)?;
        let login_form = LoginForm::from_credentials(credentials, &view_state)?;

        let login_response_body = self
            .post_form(login_url, &login_form)
            .with_context(|| "Sending login POST request failed")?;

        if login_response_body.contains("Nesprávne používateľské meno alebo heslo") {
            return Err(anyhow!("Authentication failed"));
        }

        Ok(())
    }

    pub fn get<U: IntoUrl + Copy + Debug>(&self, url: U) -> Result<String> {
        Ok(self
            .client
            .get(url)
            .send()
            .with_context(|| format!("Failed to send request to '{:?}'", url))?
            .text()
            .with_context(|| format!("Failed to parse response text from '{:?}'", url))?)
    }

    pub fn post_form<U: IntoUrl + Copy + Debug, T: Serialize + ?Sized>(
        &self,
        url: U,
        form: &T,
    ) -> Result<String> {
        Ok(self
            .client
            .post(url)
            .form(form)
            .send()
            .with_context(|| format!("Failed to send request to '{:?}'", url))?
            .text()
            .with_context(|| format!("Failed to parse response text from '{:?}'", url))?)
    }

    fn get_view_state<U: IntoUrl + Copy + Debug>(&self, url: U) -> Result<String> {
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
                .https_only(true)
                .build()
                .expect("Failed to build client"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::teepee::extract_view_state;
    use crate::TeePeeClient;
    use reqwest::blocking::Client;
    use reqwest::Url;

    #[test]
    fn test_extract_view_state() {
        let html =
            "<input type=\"hidden\" name=\"javax.faces.ViewState\" value=\"test_view_state\" />";
        assert_eq!(
            extract_view_state(html),
            Some("test_view_state".to_string())
        );

        let html_missing = "<html><body>No view state here</body></html>";
        assert_eq!(extract_view_state(html_missing), None);
    }

    #[test]
    fn test_get_view_state_success() {
        let mut server = mockito::Server::new();
        server.mock("GET", "/some_page")
            .with_status(200)
            .with_body("<input type=\"hidden\" name=\"javax.faces.ViewState\" value=\"test_view_state\" />")
            .create();

        let client = TeePeeClient::new(Client::new());
        let url = Url::parse(&server.url()).unwrap();
        let result = client
            .get_view_state(url.join("some_page").unwrap().as_str())
            .unwrap();

        assert_eq!(result, "test_view_state".to_string());
    }

    #[test]
    fn test_get_view_state_failure() {
        let mut server = mockito::Server::new();
        server
            .mock("GET", "/some_page")
            .with_status(200)
            .with_body("<html><body>No view state here</body></html>")
            .create();

        let client = TeePeeClient::new(Client::new());
        let result = client.get_view_state(&format!("{}/some_page", server.url()));

        assert!(result.is_err());
    }
}
