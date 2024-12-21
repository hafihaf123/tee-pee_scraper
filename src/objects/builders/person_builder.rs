use crate::objects::builders::ObjectBuilder;
use crate::objects::person::Gender;
use crate::objects::Person;
use anyhow::anyhow;

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

    pub fn volunteer(&mut self, volunteer: bool) -> &mut Self {
        self.volunteer = Some(volunteer);
        self
    }

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
