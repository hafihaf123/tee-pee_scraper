pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}
