use crate::objects::builders::UnitBuilder;
use crate::objects::Object;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum UnitType {
    Druzina,
    Oddiel,
    Zbor,
    Oblast,
    Rada,
}

#[derive(Clone, Debug, Default)]
pub struct Unit {
    name: String,
    id: u32,
    parent_unit: Option<Box<Unit>>,
    child_units: Vec<Unit>,
    supplementary_name: Option<String>,
    unit_type: Option<UnitType>,
    number: Option<u32>,
}

impl Unit {
    pub fn new(
        name: &str,
        id: u32,
        parent_unit: Option<Box<Unit>>,
        supplementary_name: Option<String>,
        unit_type: Option<UnitType>,
        number: Option<u32>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            id,
            parent_unit,
            child_units: Vec::new(),
            supplementary_name,
            unit_type,
            number,
        }
    }

    pub fn parent_unit(&self) -> &Option<Box<Unit>> {
        &self.parent_unit
    }

    pub fn child_units(&self) -> &Vec<Unit> {
        &self.child_units
    }

    pub fn child_units_mut(&mut self) -> &mut Vec<Unit> {
        &mut self.child_units
    }

    pub fn into_child_units(self) -> Vec<Unit> {
        self.child_units
    }

    pub fn supplementary_name(&self) -> &Option<String> {
        &self.supplementary_name
    }

    pub fn unit_type(&self) -> &Option<UnitType> {
        &self.unit_type
    }

    pub fn number(&self) -> &Option<u32> {
        &self.number
    }

    pub fn add_child_unit(&mut self, unit: Unit) {
        self.child_units.push(unit);
    }
}

impl Object<UnitBuilder> for Unit {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.id
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<8}{}", format!("({})", self.id()), self.name())
    }
}
