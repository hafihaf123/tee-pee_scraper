use crate::{Credentials, TeePeeClient};
use indicatif::ProgressBar;
use inquire::validator::{ErrorMessage, StringValidator, Validation};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct PasswordValidator {
    credentials: Arc<Credentials>,
    tee_pee_client: Arc<TeePeeClient>,
    counter: RefCell<u8>,
}

impl PasswordValidator {
    #[must_use]
    pub fn new(credentials: Arc<Credentials>, tee_pee_client: Arc<TeePeeClient>) -> Self {
        Self {
            credentials,
            tee_pee_client,
            counter: 0.into(),
        }
    }
}

impl StringValidator for PasswordValidator {
    fn validate(
        &self,
        input: &str,
    ) -> Result<Validation, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        let mut counter = self.counter.borrow_mut();
        *counter += 1;
        if *counter == 2 {
            println!();
        }

        self.credentials.set_password(input)?;

        let bar = ProgressBar::new_spinner();
        bar.set_message("Authenticating...");
        bar.enable_steady_tick(Duration::from_millis(100));

        let auth_result = self.tee_pee_client.login(&self.credentials);

        bar.finish_and_clear();

        match auth_result {
            Ok(()) => Ok(Validation::Valid),
            Err(e) => {
                self.credentials.remove_password()?;
                if *counter >= 3 {
                    return Err("Too many tries".into());
                };
                if e.to_string().contains("Authentication failed") {
                    Ok(Validation::Invalid(ErrorMessage::from(e)))
                } else {
                    Err(e.into())
                }
            }
        }
    }
}
