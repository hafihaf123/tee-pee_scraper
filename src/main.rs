use std::io::Write;
use anyhow::{Context, Result};
use rpassword::prompt_password;
use tee_pee_scraper::authentication::Credentials;
use tee_pee_scraper::scraping::{ChildUnits, MyUnits, TeePeeScraper, UnitScraper};
use tee_pee_scraper::TeePeeClient;

fn main() -> Result<()> {
    let username = read_from_stdin("Username: ")?;
    let password = prompt_password("Password: ")?;

    let credentials = Credentials::new(username, password)?;

    let tee_pee_client = TeePeeClient::default();

    tee_pee_client.login(&credentials)?;

    let mut unit_scraper = UnitScraper::new(&tee_pee_client);

    println!("Your Units:");
    for unit in unit_scraper.scrape(MyUnits)? {
        println!("{}", unit);
        
        unit_scraper
            .scrape(ChildUnits(unit))?
            .iter()
            .for_each(|unit| {
                println!("   {}", unit);
            });
    }

    Ok(())
}

pub fn read_from_stdin(message: &str) -> Result<String> {
    print!("{}", message);
    std::io::stdout().flush()?;
    let mut read_string = String::new();
    std::io::stdin()
        .read_line(&mut read_string)
        .with_context(|| "Failed to read request url from stdin")?;
    Ok(read_string.trim().to_string())
}
