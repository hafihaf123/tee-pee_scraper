/*use crate::objects::objects::Object;
use anyhow::{anyhow, Result};

#[derive(Default)]
pub struct Parent {
    name: String,
    phone: Option<String>,
    email: Option<String>,
}

impl Parent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }

    pub fn phone(&self) -> &Option<String> {
        &self.phone
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }
}

#[derive(Default)]
pub struct ParentBuilder {
    name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
}

impl ParentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Parent> {
        let name = self.name.ok_or_else(|| anyhow!("name is required"))?;

        Ok(Parent {
            name,
            phone: self.phone,
            email: self.email,
        })
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn phone(&mut self, phone: &str) -> &mut Self {
        self.phone = Some(phone.into());
        self
    }

    pub fn email(&mut self, email: &str) -> &mut Self {
        self.email = Some(email.into());
        self
    }
}
*/