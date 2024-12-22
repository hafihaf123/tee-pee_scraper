use crate::{Credentials, TeePeeClient};
use indicatif::ProgressBar;
use inquire::validator::{ErrorMessage, StringValidator, Validation};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

/// A structure implementing the [`StringValidator`] trait for validating tee-pee passwords.
/// 
/// It wil automatically sign you in and set the passwords to the keyring entry if the password is
/// valid.
/// 
/// # Examples
/// 
/// ```no_run
/// # use std::sync::Arc;
/// # use inquire::Password;
/// # use tee_pee_scraper::{Credentials, TeePeeClient};
/// # use tee_pee_scraper::authentication::PasswordValidator;
/// let teepee = Arc::new(TeePeeClient::default());
/// let creds = Arc::new(Credentials::new("password_validator").unwrap());
/// let pass_validator = PasswordValidator::new(Arc::clone(&creds), Arc::clone(&teepee));
/// Password::new("Password")
///         .with_validator(pass_validator)
///         .prompt()
///         .unwrap();
/// ```
#[derive(Clone)]
pub struct PasswordValidator {
    credentials: Arc<Credentials>,
    tee_pee_client: Arc<TeePeeClient>,
    counter: RefCell<u8>,
}

impl PasswordValidator {
    /// Creates a new instance of [`PasswordValidator`]
    /// 
    /// The [`Credentials`] and [`TeePeeClient`] need to be wrapped in [`Arc`] to avoid cloning
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use std::sync::Arc;
    /// # use tee_pee_scraper::{Credentials, TeePeeClient};
    /// # use tee_pee_scraper::authentication::PasswordValidator;
    /// let teepee = Arc::new(TeePeeClient::default());
    /// let creds = Arc::new(Credentials::new("password_validator_new").unwrap());
    /// let pass_validator = PasswordValidator::new(Arc::clone(&creds), Arc::clone(&teepee));
    /// ```
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
    /// Confirms the given input string slice is a valid tee-pee password using
    /// [`TeePeeClient::login()`]
    /// 
    /// **This function also**
    /// - sets the password for `self.credentials`
    /// - signs the `self.tee_pee_client` if the credentials are correct
    /// - checks for the number of sign-in attempts
    ///     - returns a custom `Error` after the third unsuccessful attempt
    /// 
    /// # Errors
    /// 
    /// - this function may return an Error in the following scenarios:
    ///     - credentials manipulation fails
    ///         - see [`Credentials::set_password()`] and [`Credentials::remove_password()`]
    ///     - there were 3 consecutive unsuccessful login attempts
    ///     - an error other than `Err("Authentication failed")` happened while trying to log in
    ///         - see [`TeePeeClient::login()`]
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
