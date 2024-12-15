use super::Parent;

pub enum Gender {
    Male,
    Female
}

pub struct Person {
    name: String,
    surname: String,
    gender: Option<Gender>,
    birth_date: Option<String>,
    nickname: Option<String>,
    id: Option<u32>,
    volunteer: Option<bool>,
    ztp: Option<bool>,
    parents: Vec<Parent>
}