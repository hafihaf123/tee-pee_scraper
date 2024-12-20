use std::sync::Arc;
use crate::{Credentials, TeePeeClient};
use inquire::validator::{ErrorMessage, StringValidator, Validation};

#[derive(Clone)]
pub struct PasswordValidator {
    credentials: Arc<Credentials>,
    tee_pee_client: Arc<TeePeeClient>,
}

impl PasswordValidator {
    #[must_use]
    pub fn new(credentials: Arc<Credentials>, tee_pee_client: Arc<TeePeeClient>) -> Self {
        Self {
            credentials,
            tee_pee_client,
        }
    }
}

impl StringValidator for PasswordValidator {
    fn validate(
        &self,
        input: &str,
    ) -> Result<Validation, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        self.credentials.set_password(input)?;
        match self.tee_pee_client.login(&self.credentials) {
            Ok(()) => Ok(Validation::Valid),
            Err(e) => {
                self.credentials.remove_password()?;
                if e.to_string().contains("Authentication failed") {
                    Ok(Validation::Invalid(ErrorMessage::from(e)))
                } else {
                    Err(e.into())
                }
            }
        }
    }
}
