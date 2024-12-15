use anyhow::{Context, Result};
use rpassword::prompt_password;
use tee_pee_scraper::authentication::Credentials;
use tee_pee_scraper::TeePeeClient;

fn main() -> Result<()> {
    let request_url = read_from_stdin("Url: ")?;
    let username = read_from_stdin("Username: ")?;
    let password = prompt_password("Password: ")?;

    let credentials = Credentials::new(username, password);

    let tee_pee = TeePeeClient::default();

    tee_pee.login(&credentials)?;

    println!(
        "\nResponse body:{}",
        tee_pee.get(&request_url)?
    );

    Ok(())
}

pub fn read_from_stdin(message: &str) -> Result<String> {
    println!("{}", message);
    let mut read_string = String::new();
    std::io::stdin()
        .read_line(&mut read_string)
        .with_context(|| "Failed to read request url from stdin")?;
    Ok(read_string.trim().to_string())
}
