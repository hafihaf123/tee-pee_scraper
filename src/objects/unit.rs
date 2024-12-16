use crate::objects::teepee_object::TeePeeObject;
use anyhow::{anyhow, Result};
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
    pub fn new(name: &str, id: u32) -> Self {
        Self {
            name: name.into(),
            id,
            ..Default::default()
        }
    }

    pub fn builder() -> UnitBuilder {
        UnitBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent_unit(&self) -> &Option<Box<Unit>> {
        &self.parent_unit
    }

    pub fn child_units(&self) -> &Vec<Unit> {
        &self.child_units
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

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn add_child_unit(&mut self, unit: Unit) {
        self.child_units.push(unit);
    }
}

impl TeePeeObject for Unit {}

#[derive(Default)]
pub struct UnitBuilder {
    name: Option<String>,
    id: Option<u32>,
    parent_unit: Option<Box<Unit>>,
    supplementary_name: Option<String>,
    unit_type: Option<UnitType>,
    number: Option<u32>,
}

impl UnitBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn parent_unit(&mut self, parent_unit: Unit) -> &mut Self {
        self.parent_unit = Some(Box::new(parent_unit));
        self
    }

    pub fn supplementary_name(&mut self, supplementary_name: &str) -> &mut Self {
        self.supplementary_name = Some(supplementary_name.into());
        self
    }

    pub fn unit_type(&mut self, unit_type: UnitType) -> &mut Self {
        self.unit_type = Some(unit_type);
        self
    }

    pub fn number(&mut self, number: u32) -> &mut Self {
        self.number = Some(number);
        self
    }

    pub fn id(&mut self, id: u32) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn build(self) -> Result<Unit> {
        let name = self.name.ok_or_else(|| anyhow!("name is required"))?;
        let id = self.id.ok_or_else(|| anyhow!("id is required"))?;

        Ok(Unit {
            name,
            parent_unit: self.parent_unit,
            supplementary_name: self.supplementary_name,
            unit_type: self.unit_type,
            number: self.number,
            id,
            ..Default::default()
        })
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<8}{}", format!("({})", self.id()), self.name())
    }
}
