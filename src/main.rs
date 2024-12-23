use anyhow::{Context, Result};
use indicatif::ProgressBar;
use inquire::{Password, Text};
use std::sync::Arc;
use std::time::Duration;
use tee_pee_scraper::authentication::PasswordValidator;
use tee_pee_scraper::scraping::{MyUnits, PersonScraper, UnitScraper};
use tee_pee_scraper::{Credentials, Object, Scraper, TeePeeClient};

fn main() -> Result<()> {
    let username = Text::new("Username:")
        .prompt()
        .with_context(|| "Failed to read username")?;

    let credentials = Arc::new(Credentials::new(&username)?);
    let tee_pee_client = Arc::new(TeePeeClient::default());

    let password_validator =
        PasswordValidator::new(Arc::clone(&credentials), Arc::clone(&tee_pee_client));

    if credentials.has_password() {
        let bar = ProgressBar::new_spinner();
        bar.set_message("Authenticating...");
        bar.enable_steady_tick(Duration::from_millis(100));

        tee_pee_client.login(&credentials)?;

        bar.finish_and_clear();
    } else {
        Password::new("Password:")
            .with_validator(password_validator)
            .without_confirmation()
            .prompt()?;
    }

    let mut unit_scraper = UnitScraper::new(&tee_pee_client);

    println!("\nYour Units:");
    for mut unit in unit_scraper.scrape(MyUnits)? {
        println!("{unit}");

        unit.scrape_child_units(&mut unit_scraper)?;
        unit.into_child_units().iter_mut().for_each(|child| {
            println!("   {}", child.name());
            if child.name().eq("Rysi") {
                let mut person_scraper = PersonScraper::new(&tee_pee_client);
                child
                    .scrape_persons(&mut person_scraper)
                    .expect("Failed scraping persons");
                child.persons().iter().for_each(|person| {
                    println!("      {}", person.name());
                });
                println!();
            }
        });
    }

    Ok(())
}
