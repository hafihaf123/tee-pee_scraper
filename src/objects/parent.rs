#[derive(Default)]
pub struct Parent {
    name: String,
    surname: String,
    phone: Option<String>,
    email: Option<String>,
}

impl Parent {
    pub fn new(name: String, surname: String) -> Self {
        Self {
            name,
            surname,
            ..Default::default()
        }
    }

    pub fn builder(name: String, surname: String) -> ParentBuilder {
        ParentBuilder {
            name,
            surname,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn surname(&self) -> &str {
        &self.surname
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
    name: String,
    surname: String,
    phone: Option<String>,
    email: Option<String>,
}

impl ParentBuilder {
    pub fn new(surname: String, name: String) -> Self {
        Self {
            name,
            surname,
            ..Default::default()
        }
    }

    pub fn build(self) -> Parent {
        Parent {
            name: self.name,
            surname: self.surname,
            phone: self.phone,
            email: self.email,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn surname(mut self, surname: &str) -> Self {
        self.surname = surname.into();
        self
    }

    pub fn phone(mut self, phone: &str) -> Self {
        self.phone = Some(phone.into());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.into());
        self
    }
}
