use crate::object::builder::ObjectBuilder;
use crate::object::unit::UnitType;
use crate::object::Unit;
use anyhow::anyhow;

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
}

impl ObjectBuilder<Unit> for UnitBuilder {
    fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    fn id(&mut self, id: u32) -> &mut Self {
        self.id = Some(id);
        self
    }

    fn build(self) -> anyhow::Result<Unit> {
        let name = self.name.ok_or_else(|| anyhow!("name is required"))?;
        let id = self.id.ok_or_else(|| anyhow!("id is required"))?;

        Ok(Unit::new(
            &name,
            id,
            self.parent_unit,
            self.supplementary_name,
            self.unit_type,
            self.number,
        ))
    }
}
