use anyhow::{anyhow, Result};
use keyring::Entry;

pub struct Credentials {
    username: String,
    password: Entry,
}

impl Credentials {
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

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> keyring::Result<String> {
        self.password.get_password()
    }

    pub fn has_password(&self) -> bool {
        self.password.get_password().is_ok()
    }

    pub fn set_password(&self, password: &str) -> keyring::Result<()> {
        self.password.set_password(password)
    }

    pub fn remove_password(&self) -> keyring::Result<()> {
        self.password.delete_credential()
    }
}
