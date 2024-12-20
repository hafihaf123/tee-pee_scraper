use anyhow::{Context, Result};
use inquire::{Password, Text};
use std::io::Write;
use std::sync::Arc;
use tee_pee_scraper::authentication::PasswordValidator;
use tee_pee_scraper::scraping::{FromUnit, MyUnits, PersonScraper, UnitScraper};
use tee_pee_scraper::Object;
use tee_pee_scraper::{Credentials, Scraper, TeePeeClient};

fn main() -> Result<()> {
    // let username = read_from_stdin("Username: ")?;
    let username = Text::new("Username:")
        .with_placeholder("placeholder")
        .with_help_message("help message")
        .prompt()
        .with_context(|| "Failed to read username")?;

    let credentials = Arc::new(Credentials::new(&username)?);
    let tee_pee_client = Arc::new(TeePeeClient::default());

    let password_validator =
        PasswordValidator::new(Arc::clone(&credentials), Arc::clone(&tee_pee_client));
    if !credentials.has_password() {
        let password = Password::new("Password:")
            .with_validator(password_validator)
            .without_confirmation()
            .prompt()
            .with_context(|| "Failed to read password")?;
        credentials.set_password(&password)?;
    }

    let mut unit_scraper = UnitScraper::new(&tee_pee_client);

    println!("Your Units:");
    for mut unit in unit_scraper.scrape(MyUnits)? {
        println!("{unit}");

        unit.scrape_child_units(&mut unit_scraper)?;
        unit.into_child_units().iter_mut().for_each(|child| {
            if child.name().eq("Húsenice") {
                println!("\n{} unit persons:\n", child.name());
                let mut person_scraper = PersonScraper::new(&tee_pee_client);
                let persons = person_scraper
                    .scrape(FromUnit(child.clone()))
                    .expect("Failed to scrape persons");
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
    print!("{message}");
    std::io::stdout().flush()?;
    let mut read_string = String::new();
    std::io::stdin()
        .read_line(&mut read_string)
        .with_context(|| "Failed to read from stdin")?;
    Ok(read_string.trim().to_string())
}
