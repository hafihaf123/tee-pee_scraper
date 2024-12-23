use crate::objects::builders::PersonBuilder;
use crate::objects::Object;

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Default, Debug, Clone)]
/// Represents a person with various attributes.
pub struct Person {
    /// The name of the person.
    name: String,
    /// The unique identifier of the person.
    id: u32,
    /// The gender of the person.
    gender: Option<Gender>,
    /// The birthdate of the person.
    birth_date: Option<String>,
    /// The nickname of the person.
    nickname: Option<String>,
    /// Indicates if the person is a volunteer.
    volunteer: Option<bool>,
    /// Indicates if the person has ZTP (Zero Tolerance Policy).
    ztp: Option<bool>,
    // parents: Vec<Parent>,
}

impl Person {
    /// Creates a new `Person`.
    ///
    /// # Parameters
    /// - `name`: The name of the person.
    /// - `id`: The unique identifier of the person.
    /// - `gender`: The gender of the person.
    /// - `birth_date`: The birth date of the person.
    /// - `nickname`: The nickname of the person.
    /// - `volunteer`: Indicates if the person is a volunteer.
    /// - `ztp`: Indicates if the person has ZTP.
    ///
    /// # Returns
    /// A new `Person` instance.
    #[must_use]
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

    /// Returns a reference to the gender of the person.
    #[must_use]
    pub fn gender(&self) -> &Option<Gender> {
        &self.gender
    }

    /// Returns a reference to the birthdate of the person.
    #[must_use]
    pub fn birth_date(&self) -> &Option<String> {
        &self.birth_date
    }

    /// Returns a reference to the nickname of the person.
    #[must_use]
    pub fn nickname(&self) -> &Option<String> {
        &self.nickname
    }

    /// Returns whether the person is a volunteer.
    #[must_use]
    pub fn volunteer(&self) -> Option<bool> {
        self.volunteer
    }

    /// Returns whether the person has ZTP.
    #[must_use]
    pub fn ztp(&self) -> Option<bool> {
        self.ztp
    }

    // /// Returns a reference to the parents of the person.
    // pub fn parents(&self) -> &Vec<Parent> {
    //     &self.parents
    // }
    //
    // pub fn add_parent(&mut self, parent: Parent) {
    //     self.parents.push(parent);
    // }
}

impl Object for Person {
    type Builder = PersonBuilder;

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.id
    }
}
