use crate::authentication::{Credentials, LoginForm};
use crate::create_selector;
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use reqwest::IntoUrl;
use scraper::Html;
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
/// # let credentials = Credentials::new("teepee").unwrap();
/// # credentials.set_password("pass").unwrap();
/// assert!(teepee.login(&credentials).is_err()); // wrong credentials were used
/// # credentials.remove_password().unwrap();
/// ```
#[derive(Clone)]
pub struct TeePeeClient {
    client: Client,
}

fn extract_view_state(html: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector = create_selector("input[name=\"javax.faces.ViewState\"]")?;
    document
        .select(&selector)
        .next()
        .and_then(|input| input.value().attr("value"))
        .map(ToString::to_string)
        .ok_or_else(|| anyhow::anyhow!("Could not find view state"))
}

impl TeePeeClient {
    /// Constructs a new instance of [`TeePeeClient`] with a [`Client`], returning it.
    ///
    /// **The usage of this function is _NOT_ recommended** - in most cases you should construct
    /// a new [`TeePeeClient`] instance using the [`TeePeeClient::default()`] method
    ///
    /// # Arguments
    ///
    /// - client: [`Client`]
    ///     - it should implement a *cookie store*
    ///     - also recommended to have the *'https only'* restriction
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::TeePeeClient;
    /// let client = reqwest::blocking::Client::builder()
    ///                             .cookie_store(true)
    ///                             .https_only(true)
    ///                             .build().unwrap();
    /// let teepee = TeePeeClient::new(client);
    /// ```
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Logs a user in based on their credentials by storing a validated session cookie
    /// inside the [`TeePeeClient`]'s client field's cookie store.
    ///
    /// # Prerequisites
    ///
    /// - The client field must have a cookie store implemented
    ///     - the [`TeePeeClient::default()`] function sets it up for you!
    ///     - when constructing using [`TeePeeClient::new()`], the client you pass into it needs to
    ///       be built with `.cookie_store(true)` or `.cookie_provider(...)`
    ///
    /// # Errors
    ///
    /// - the function may return an error value in the following cases:
    ///     - propagated errors
    ///         - communication with the server fails
    ///         - parsing the credentials fails
    ///     - the server does not accept the credentials as valid
    ///         - `anyhow::Error("Authentication failed")`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::authentication::Credentials;
    /// use tee_pee_scraper::TeePeeClient;
    /// let teepee = TeePeeClient::default();
    ///
    /// let credentials = Credentials::new("teepee_login").unwrap();
    /// credentials.set_password("password").unwrap();
    ///
    /// assert!(teepee.login(&credentials).is_err()); // Authentication failed
    /// # credentials.remove_password().unwrap();
    /// ```
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

    /// Processes a get request to an url using the [`TeePeeClient`], returning the response text
    /// as a String
    ///
    /// # Errors
    ///
    /// - the function may return an error value in the following cases:
    ///     - sending the request fails
    ///     - parsing the response text fails
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::TeePeeClient;
    /// let teepee = TeePeeClient::default();
    /// let login_page_text = teepee.get("https://skauting.tee-pee.com/login").unwrap();
    /// assert!(login_page_text.contains("Login"));
    /// ```
    pub fn get<U: IntoUrl + Copy + Debug>(&self, url: U) -> Result<String> {
        self.client
            .get(url)
            .send()
            .with_context(|| format!("Failed to send request to '{url:?}'"))?
            .text()
            .with_context(|| format!("Failed to parse response text from '{url:?}'"))
    }

    /// Processes a post request containing a form to an url using the [`TeePeeClient`], returning
    /// the response text as a String
    ///
    /// # Errors
    ///
    /// - the function may return an error value in the following cases:
    ///     - sending the request fails
    ///     - parsing the response text fails
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::authentication::LoginForm;
    /// use tee_pee_scraper::TeePeeClient;
    /// let teepee = TeePeeClient::default();
    /// let login_form = LoginForm::new("username", "password", "0:0");
    /// let login_page_text = teepee.post_form("https://skauting.tee-pee.com/login", &login_form).unwrap();
    /// assert!(login_page_text.contains("Login")); // response for invalid login data
    /// ```
    pub fn post_form<U: IntoUrl + Copy + Debug, T: Serialize + ?Sized>(
        &self,
        url: U,
        form: &T,
    ) -> Result<String> {
        self.client
            .post(url)
            .form(form)
            .send()
            .with_context(|| format!("Failed to send request to '{url:?}'"))?
            .text()
            .with_context(|| format!("Failed to parse response text from '{url:?}'"))
    }

    /// Extracts the value of "javax.faces.ViewState" (for sending forms)
    ///
    /// # Errors
    ///
    /// - sending a GET request to the url fails
    /// - the "javax.faces.ViewState" element could not be found
    pub fn get_view_state<U: IntoUrl + Copy + Debug>(&self, url: U) -> Result<String> {
        let page_text = self.get(url)?;

        extract_view_state(&page_text)
            .with_context(|| format!("Failed to extract view state from page: '{url:?}'"))
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
            extract_view_state(html).unwrap(),
            "test_view_state".to_string()
        );

        let html_missing = "<html><body>No view state here</body></html>";
        assert!(extract_view_state(html_missing).is_err());
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
