use reqwest::blocking::Client;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::Url;
use scraper::{Html, Selector};
use std::sync::Arc;

fn extract_view_state(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("input[name=\"javax.faces.ViewState\"]").unwrap();
    document
        .select(&selector)
        .next()
        .and_then(|input| input.value().attr("value"))
        .map(|v| v.to_string())
}

fn main() {
    println!("URL:");
    let mut request_url = String::new();
    std::io::stdin()
        .read_line(&mut request_url)
        .expect("Failed to read request url from stdin");
    request_url = request_url.trim().to_string();

    println!("username:");
    let mut username = String::new();
    std::io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username from stdin");
    username = username.trim().to_string();

    println!("password:");
    let mut password = String::new();
    std::io::stdin()
        .read_line(&mut password)
        .expect("Failed to read password from stdin");
    password = password.trim().to_string();

    let cookie_store = Arc::new(Jar::default());

    let client = Client::builder()
        .cookie_provider(cookie_store.clone())
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()
        .expect("Failed to build client");

    let login_url =
        Url::parse("https://skauting.tee-pee.com/login").expect("Failed to parse login url");

    let login_page = client
        .get(login_url.clone())
        .send()
        .expect("Failed to fetch login page");
    let login_page_text = login_page.text().expect("Failed to read login page text");
    let view_state =
        extract_view_state(&login_page_text).expect("Failed to extract javax.faces.ViewState");

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
        .expect("Sending login POST request failed");

    println!("Login response Status: {}", login.status());
    println!("Login response Cookies:");
    login.cookies().for_each(|cookie| println!("{:?}", cookie));
    if let Some(cookies) = cookie_store.cookies(&login_url) {
        println!("Stored cookies: {:?}", cookies);
    }

    let login_response_body = login.text().expect("Failed to parse login response text");

    println!("\nLogin response body:");
    println!("{}", login_response_body);

    if login_response_body.contains("Nesprávne používateľské meno alebo heslo") {
        panic!("Login Failed");
    }

    let response = client
        .get(request_url)
        .send()
        .expect("Failed to send request");

    println!("Response Status: {}", response.status());
    println!("Response Cookies:");
    response
        .cookies()
        .for_each(|cookie| println!("{:?}", cookie));

    let _response_body = response.text().expect("Failed to parse response text");

    // println!("\nResponse body:");
    // println!("{}", response_body);
}
