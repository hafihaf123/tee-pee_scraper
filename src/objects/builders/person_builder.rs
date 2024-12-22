use crate::objects::builders::ObjectBuilder;
use crate::objects::person::Gender;
use crate::objects::Person;
use anyhow::anyhow;

/// A builder for creating `Person` objects.
#[derive(Default)]
pub struct PersonBuilder {
    name: Option<String>,
    id: Option<u32>,
    gender: Option<Gender>,
    birth_date: Option<String>,
    nickname: Option<String>,
    volunteer: Option<bool>,
    ztp: Option<bool>,
}

impl PersonBuilder {
    /// Sets the gender for the person being built.
    ///
    /// # Arguments
    ///
    /// * `gender` - The `Gender` of the person.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn gender(&mut self, gender: Gender) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    /// Sets the birthdate for the person being built.
    ///
    /// # Arguments
    ///
    /// * `birth_date` - A string slice that holds the birthdate.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn birth_date(&mut self, birth_date: &str) -> &mut Self {
        self.birth_date = Some(birth_date.into());
        self
    }

    /// Sets the nickname for the person being built.
    ///
    /// # Arguments
    ///
    /// * `nickname` - A string slice that holds the nickname.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn nickname(&mut self, nickname: &str) -> &mut Self {
        self.nickname = Some(nickname.into());
        self
    }

    /// Sets the volunteer status for the person being built.
    ///
    /// # Arguments
    ///
    /// * `volunteer` - A boolean that indicates if the person is a volunteer.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn volunteer(&mut self, volunteer: bool) -> &mut Self {
        self.volunteer = Some(volunteer);
        self
    }

    /// Sets the ZTP status for the person being built.
    ///
    /// # Arguments
    ///
    /// * `ztp` - A boolean that indicates if the person has ZTP status.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn ztp(&mut self, ztp: bool) -> &mut Self {
        self.ztp = Some(ztp);
        self
    }
}

impl ObjectBuilder for PersonBuilder {
    type Object = Person;

    fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    fn id(&mut self, id: u32) -> &mut Self {
        self.id = Some(id);
        self
    }

    fn build(self) -> anyhow::Result<Person> {
        let name = self.name.ok_or_else(|| anyhow!("name is required"))?;
        let id = self.id.ok_or_else(|| anyhow!("id is required"))?;

        Ok(Person::new(
            &name,
            id,
            self.gender,
            self.birth_date,
            self.nickname,
            self.volunteer,
            self.ztp,
        ))
    }
}
