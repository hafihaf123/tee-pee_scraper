use anyhow::{anyhow, Result};
use keyring::Entry;

/// Wrapper for [`Entry`] with a username field. Used to create or load saved credentials
///
/// # Examples
///
/// ```
/// use tee_pee_scraper::authentication::Credentials;
/// let creds = Credentials::new("credentials").unwrap();
/// assert!(!creds.has_password());
/// creds.set_password("pass").unwrap();
/// assert_eq!(creds.password().unwrap(), "pass");
/// # creds.remove_password().unwrap();
/// ```
pub struct Credentials {
    username: String,
    password: Entry,
}

impl Credentials {
    /// Creates a new instance of [`Credentials`] with a username.
    ///
    /// The service for the keyring entry is *skauting.tee-pee.com* as that is the site this crate
    /// is used to interact with
    ///
    /// # Errors
    ///
    /// - the username is an empty string
    ///     - `anyhow::Error(Invalid username: empty)`
    /// - the creation of a new entry fails
    ///     - see [`Entry::new()`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::authentication::Credentials;
    /// assert!(Credentials::new("").is_err());
    /// assert!(Credentials::new("credentials_new").is_ok());
    /// ```
    pub fn new(username: &str) -> Result<Self> {
        if username.is_empty() {
            return Err(anyhow!("Invalid username: empty"));
        }
        let password = Entry::new("skauting.tee-pee.com", username)?;

        Ok(Self {
            username: username.into(),
            password,
        })
    }

    /// Returns the `username` field value as a string slice
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::authentication::Credentials;
    /// let creds = Credentials::new("credentials_username").unwrap();
    /// assert_eq!(creds.username(), "credentials_username");
    /// ```
    #[must_use]
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the password contained in the entry, if any
    ///
    /// # Errors
    ///
    /// - the same as for [`Entry::get_password()`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::authentication::Credentials;
    /// let creds = Credentials::new("credentials_password").unwrap();
    /// assert!(creds.password().is_err()); // NoEntry error
    /// creds.set_password("pass").unwrap();
    /// assert_eq!(creds.password().unwrap(), "pass");
    /// # creds.remove_password().unwrap();
    /// ```
    pub fn password(&self) -> keyring::Result<String> {
        self.password.get_password()
    }

    /// Checks whether the [`Entry`] contains a password, returning the corresponding `bool` value
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::authentication::Credentials;
    /// let creds = Credentials::new("credentials_has_password").unwrap();
    /// assert!(!creds.has_password());
    /// creds.set_password("pass").unwrap();
    /// assert!(creds.has_password());
    /// # creds.remove_password().unwrap();
    /// ```
    #[must_use]
    pub fn has_password(&self) -> bool {
        self.password.get_password().is_ok()
    }

    /// Sets the [`Entry`] password to a specific value
    ///
    /// # Errors
    ///
    /// - the same as for [`Entry::set_password()`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::authentication::Credentials;
    /// let creds = Credentials::new("credentials_set_password").unwrap();
    /// assert!(!creds.has_password());
    /// creds.set_password("pass").unwrap();
    /// assert_eq!(creds.password().unwrap(), "pass");
    /// # creds.remove_password().unwrap();
    /// ```
    pub fn set_password(&self, password: &str) -> keyring::Result<()> {
        self.password.set_password(password)
    }

    /// Removes the corresponding password from the [`Entry`]
    ///
    /// # Errors
    ///
    /// - the same as for [`Entry::delete_credential()`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use tee_pee_scraper::authentication::Credentials;
    /// let creds = Credentials::new("credentials_remove_password").unwrap();
    /// creds.set_password("pass").unwrap();
    /// assert!(creds.has_password());
    /// creds.remove_password().unwrap();
    /// assert!(!creds.has_password());
    /// ```
    pub fn remove_password(&self) -> keyring::Result<()> {
        self.password.delete_credential()
    }
}
