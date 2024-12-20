use super::Credentials;
use anyhow::Result;
use serde::Serialize;

/// A structure representing the login form data for <https://skauting.tee-pee.som>
#[derive(Serialize)]
pub struct LoginForm {
    #[serde(rename = "loginForm")]
    login_form: String,
    #[serde(rename = "usernameId")]
    username_id: String,
    #[serde(rename = "passwordId")]
    password_id: String,
    #[serde(rename = "loginBtnId")]
    login_btn_id: String,
    #[serde(rename = "javax.faces.ViewState")]
    javax_faces_view_state: String,
}

impl LoginForm {
    /// Constructor for [`LoginForm`] from string slices
    #[must_use]
    pub fn new(username: &str, password: &str, javax_faces_view_state: &str) -> Self {
        Self {
            login_form: "loginForm".to_string(),
            username_id: username.to_string(),
            password_id: password.to_string(),
            login_btn_id: String::default(),
            javax_faces_view_state: javax_faces_view_state.to_string(),
        }
    }

    /// Constructor for [`LoginForm`] using the [`Credentials`] structure
    ///
    /// # Errors
    ///
    /// - when getting the password from [`Credentials`] fails
    ///     - see [`Credentials::password()`]
    pub fn from_credentials(
        credentials: &Credentials,
        javax_faces_view_state: &str,
    ) -> Result<Self> {
        let password = credentials.password()?;
        Ok(Self {
            login_form: "loginForm".to_string(),
            username_id: credentials.username().to_string(),
            password_id: password,
            login_btn_id: String::default(),
            javax_faces_view_state: javax_faces_view_state.to_string(),
        })
    }
}
