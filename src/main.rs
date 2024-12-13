use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};

fn extract_view_state(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("input[name=\"javax.faces.ViewState\"]").unwrap();
    document
        .select(&selector)
        .next()
        .and_then(|input| input.value().attr("value"))
        .map(|v| v.to_string())
}

fn main() -> Result<()> {
    let request_url = read_from_stdin("Url:")?;
    let username = read_from_stdin("Username:")?;
    let password = read_from_stdin("Password:")?;

    let client = Client::builder()
        .cookie_store(true)
        .build()
        .with_context(|| "Failed to build client")?;

    login_with_creds(&username, &password, &client)?;

    let response = client
        .get(request_url)
        .send()
        .with_context(|| "Failed to send request")?;

    let response_body = response
        .text()
        .with_context(|| "Failed to parse response text")?;

    println!("\nResponse body:");
    println!("{}", response_body);

    Ok(())
}

fn login_with_creds(username: &String, password: &String, client: &Client) -> Result<()> {
    let login_url = Url::parse("https://skauting.tee-pee.com/login")
        .with_context(|| "Failed to parse login url")?;

    let login_page = client
        .get(login_url.clone())
        .send()
        .with_context(|| "Failed to fetch login page")?;
    let login_page_text = login_page
        .text()
        .with_context(|| "Failed to read login page text")?;
    let view_state = extract_view_state(&login_page_text)
        .with_context(|| "Failed to extract javax.faces.ViewState")?;

    let login_form_data = [
        ("loginForm", "loginForm"),
        ("usernameId", &username),
        ("passwordId", &password),
        ("loginBtnId", ""),
        ("javax.faces.ViewState", &view_state),
    ];

    let login = client
        .post(login_url.clone())
        .form(&login_form_data)
        .send()
        .with_context(|| "Sending login POST request failed")?;

    let login_response_body = login
        .text()
        .with_context(|| "Failed to parse login response text")?;

    if login_response_body.contains("Nesprávne používateľské meno alebo heslo") {
        return Err(anyhow!("Login Failed"));
    }

    Ok(())
}

fn read_from_stdin(message: &str) -> Result<String> {
    println!("{}", message);
    let mut read_string = String::new();
    std::io::stdin()
        .read_line(&mut read_string)
        .with_context(|| "Failed to read request url from stdin")?;
    Ok(read_string.trim().to_string())
}
