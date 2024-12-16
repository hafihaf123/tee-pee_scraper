use anyhow::{anyhow, Result};

pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Result<Self> {
        if username.is_empty() || password.is_empty() {
            return Err(anyhow!("Invalid credentials"));
        }
        Ok(Self { username, password })
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}
