use crate::objects::teepee_object::TeePeeObject;

#[derive(Default)]
pub struct Parent {
    name: String,
    surname: String,
    phone: Option<String>,
    email: Option<String>,
}

impl Parent {
    pub fn new(name: &str, surname: &str) -> Self {
        Self {
            name: name.into(),
            surname: surname.into(),
            ..Default::default()
        }
    }

    pub fn builder() -> ParentBuilder {
        ParentBuilder::new()
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

impl TeePeeObject for Parent {}

#[derive(Default)]
pub struct ParentBuilder {
    name: String,
    surname: String,
    phone: Option<String>,
    email: Option<String>,
}

impl ParentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Parent {
        Parent {
            name: self.name,
            surname: self.surname,
            phone: self.phone,
            email: self.email,
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.into();
        self
    }

    pub fn surname(&mut self, surname: &str) -> &mut Self {
        self.surname = surname.into();
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
