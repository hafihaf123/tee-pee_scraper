use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use serde::Serialize;

pub struct TeePee {
    client: Client,
}

pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginForm {
    #[serde(rename = "loginForm")]
    login_form: String,
    #[serde(rename = "usernameId")]
    username_id: String,
    #[serde(rename = "passwordId")]
    password_id: String,
    #[serde(rename = "loginBtnId")]
    login_btn_id: String,
    #[serde(rename = "javax.faces.ViewState")]
    javax_faces_view_state: String,
}

impl LoginForm {
    pub fn new(username: &str, password: &str, javax_faces_view_state: &str) -> Self {
        Self {
            login_form: "loginForm".to_string(),
            username_id: username.to_string(),
            password_id: password.to_string(),
            login_btn_id: "".to_string(),
            javax_faces_view_state: javax_faces_view_state.to_string(),
        }
    }
}

impl Credentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
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

impl TeePee {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    pub fn login_with_creds(&self, credentials: &Credentials) -> Result<()> {
        let login_url = Url::parse("https://skauting.tee-pee.com/login")
            .with_context(|| "Failed to parse login url")?;

        let view_state = get_login_viewstate(&self.client, login_url.clone())?;

        let login_form =
            LoginForm::new(credentials.username(), credentials.password(), &view_state);

        let login_response_body = self
            .client
            .post(login_url)
            .form(&login_form)
            .send()
            .with_context(|| "Sending login POST request failed")?
            .text()
            .with_context(|| "Failed to parse login response text")?;

        if login_response_body.contains("Nesprávne používateľské meno alebo heslo") {
            return Err(anyhow!("Login Failed"));
        }

        Ok(())
    }
    pub fn get_response_body(&self, request_url: Url) -> Result<String> {
        Ok(self
            .client
            .get(request_url)
            .send()
            .with_context(|| "Failed to send request")?
            .text()
            .with_context(|| "Failed to parse response text")?)
    }
}

fn get_login_viewstate(client: &Client, login_url: Url) -> Result<String> {
    let login_page = client
        .get(login_url)
        .send()
        .with_context(|| "Failed to fetch login page")?;
    let login_page_text = login_page
        .text()
        .with_context(|| "Failed to read login page text")?;

    Ok(extract_view_state(&login_page_text)
        .with_context(|| "Failed to extract javax.faces.ViewState")?)
}
