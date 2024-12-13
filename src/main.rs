use crate::teepee::{Credentials, TeePee};
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::Url;
use std::io::Write;

mod teepee;

fn main() -> Result<()> {
    let request_url = read_from_stdin("Url:")?.parse::<Url>()?;
    let username = read_from_stdin("Username:")?;
    let password = read_from_stdin("Password:")?;

    let client = Client::builder()
        .cookie_store(true)
        .build()
        .with_context(|| "Failed to build client")?;

    let credentials = Credentials::new(username, password);

    let tee_pee = TeePee::new(client);

    tee_pee.login_with_creds(&credentials)?;

    println!(
        "\nResponse body:{}",
        tee_pee.get_response_body(request_url)?
    );

    Ok(())
}

fn read_from_stdin(message: &str) -> Result<String> {
    println!("{}", message);
    print!("> ");
    std::io::stdout().flush()?;
    let mut read_string = String::new();
    std::io::stdin()
        .read_line(&mut read_string)
        .with_context(|| "Failed to read request url from stdin")?;
    Ok(read_string.trim().to_string())
}
