use super::Parent;
use crate::objects::teepee_object::TeePeeObject;

pub enum Gender {
    Male,
    Female,
}

#[derive(Default)]
pub struct Person {
    name: String,
    surname: String,
    gender: Option<Gender>,
    birth_date: Option<String>,
    nickname: Option<String>,
    id: Option<u32>,
    volunteer: Option<bool>,
    ztp: Option<bool>,
    parents: Vec<Parent>,
}

impl Person {
    pub fn new(name: &str, surname: &str) -> Self {
        Self {
            name: name.into(),
            surname: surname.into(),
            ..Default::default()
        }
    }

    pub fn builder() -> PersonBuilder {
        PersonBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn surname(&self) -> &str {
        &self.surname
    }

    pub fn gender(&self) -> &Option<Gender> {
        &self.gender
    }

    pub fn birth_date(&self) -> &Option<String> {
        &self.birth_date
    }

    pub fn nickname(&self) -> &Option<String> {
        &self.nickname
    }

    pub fn id(&self) -> Option<u32> {
        self.id
    }

    pub fn volunteer(&self) -> Option<bool> {
        self.volunteer
    }

    pub fn ztp(&self) -> Option<bool> {
        self.ztp
    }

    pub fn parents(&self) -> &Vec<Parent> {
        &self.parents
    }

    pub fn add_parent(&mut self, parent: Parent) {
        self.parents.push(parent);
    }
}

impl TeePeeObject for Person {}

#[derive(Default)]
pub struct PersonBuilder {
    name: String,
    surname: String,
    gender: Option<Gender>,
    birth_date: Option<String>,
    nickname: Option<String>,
    id: Option<u32>,
    volunteer: Option<bool>,
    ztp: Option<bool>,
}

impl PersonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Person {
        Person {
            name: self.name,
            surname: self.surname,
            gender: self.gender,
            birth_date: self.birth_date,
            nickname: self.nickname,
            id: self.id,
            volunteer: self.volunteer,
            ztp: self.ztp,
            ..Default::default()
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

    pub fn gender(&mut self, gender: Gender) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    pub fn birth_date(&mut self, birth_date: &str) -> &mut Self {
        self.birth_date = Some(birth_date.into());
        self
    }

    pub fn nickname(&mut self, nickname: &str) -> &mut Self {
        self.nickname = Some(nickname.into());
        self
    }

    pub fn id(&mut self, id: u32) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn volunteer(&mut self, volunteer: bool) -> &mut Self {
        self.volunteer = Some(volunteer);
        self
    }

    pub fn ztp(&mut self, ztp: bool) -> &mut Self {
        self.ztp = Some(ztp);
        self
    }
}
