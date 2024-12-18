use anyhow::{Context, Result};
use rpassword::prompt_password;
use std::io::Write;
use tee_pee_scraper::authentication::Credentials;
use tee_pee_scraper::scraping::{FromUnit, MyUnits, PersonScraper, Scraper, UnitScraper};
use tee_pee_scraper::TeePeeClient;

fn main() -> Result<()> {
    let username = read_from_stdin("Username: ")?;
    let credentials = Credentials::new(&username)?;

    let tee_pee_client = TeePeeClient::default();

    loop {
        if !credentials.has_password() {
            let password = prompt_password("Password: ")?;
            credentials.set_password(&password)?;
        }
        match tee_pee_client.login(&credentials) {
            Ok(_) => break,
            Err(e) => {
                credentials.remove_password()?;
                if !e.to_string().contains("Authentication failed") {
                    return Err(e);
                }
                eprintln!("{}", e);
            }
        }
    }

    let mut unit_scraper = UnitScraper::new(&tee_pee_client);

    println!("Your Units:");
    for mut unit in unit_scraper.scrape(MyUnits)? {
        println!("{}", unit);

        unit.scrape_child_units(&mut unit_scraper)?;
        unit.into_child_units().iter_mut().for_each(|child| {
            if child.name().eq("Húsenice") {
                println!("\n{} unit persons:\n", child.name());
                let mut person_scraper = PersonScraper::new(&tee_pee_client);
                let persons = person_scraper.scrape(FromUnit(child.clone())).expect("Failed to scrape persons");
                for person in persons {
                    println!("{}", person.name());
                }
                println!();
            }
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
