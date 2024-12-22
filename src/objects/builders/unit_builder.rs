use crate::objects::builders::ObjectBuilder;
use crate::objects::unit::Type;
use crate::objects::Unit;
use anyhow::anyhow;

/// A builder for creating [`Unit`] objects.
#[derive(Default)]
pub struct UnitBuilder {
    name: Option<String>,
    id: Option<u32>,
    parent_unit: Option<Box<Unit>>,
    supplementary_name: Option<String>,
    unit_type: Option<Type>,
    number: Option<u32>,
}

impl UnitBuilder {
    /// Sets the parent unit for the unit being built.
    ///
    /// # Arguments
    ///
    /// * `parent_unit` - The parent `Unit` object.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn parent_unit(&mut self, parent_unit: Unit) -> &mut Self {
        self.parent_unit = Some(Box::new(parent_unit));
        self
    }

    /// Sets the supplementary name for the unit being built.
    ///
    /// # Arguments
    ///
    /// * `supplementary_name` - A string slice that holds the supplementary name.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn supplementary_name(&mut self, supplementary_name: &str) -> &mut Self {
        self.supplementary_name = Some(supplementary_name.into());
        self
    }

    /// Sets the type for the unit being built.
    ///
    /// # Arguments
    ///
    /// * `unit_type` - The `Type` of the unit.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn unit_type(&mut self, unit_type: Type) -> &mut Self {
        self.unit_type = Some(unit_type);
        self
    }

    /// Sets the number for the unit being built.
    ///
    /// # Arguments
    ///
    /// * `number` - A 32-bit unsigned integer that holds the number.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn number(&mut self, number: u32) -> &mut Self {
        self.number = Some(number);
        self
    }
}

impl ObjectBuilder for UnitBuilder {
    type Object = Unit;

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
