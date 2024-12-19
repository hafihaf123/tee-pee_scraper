use crate::object::builder::PersonBuilder;
use crate::object::object::Object;

pub enum Gender {
    Male,
    Female,
}

#[derive(Default)]
pub struct Person {
    name: String,
    id: u32,
    gender: Option<Gender>,
    birth_date: Option<String>,
    nickname: Option<String>,
    volunteer: Option<bool>,
    ztp: Option<bool>,
    // parents: Vec<Parent>,
}

impl Person {
    pub fn new(
        name: &str,
        id: u32,
        gender: Option<Gender>,
        birth_date: Option<String>,
        nickname: Option<String>,
        volunteer: Option<bool>,
        ztp: Option<bool>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            id,
            gender,
            birth_date,
            nickname,
            volunteer,
            ztp,
            // parents: Vec::new(),
        }
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

    pub fn volunteer(&self) -> Option<bool> {
        self.volunteer
    }

    pub fn ztp(&self) -> Option<bool> {
        self.ztp
    }

    // pub fn parents(&self) -> &Vec<Parent> {
    //     &self.parents
    // }
    //
    // pub fn add_parent(&mut self, parent: Parent) {
    //     self.parents.push(parent);
    // }
}

impl Object<PersonBuilder> for Person {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.id
    }
}
