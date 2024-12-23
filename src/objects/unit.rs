use crate::objects::builders::UnitBuilder;
use crate::objects::{Object, Person};
use std::fmt::{Display, Formatter};

/// Represents the type of a unit in an organizational hierarchy.
#[derive(Clone, Debug)]
pub enum Type {
    Druzina,
    Oddiel,
    Zbor,
    Oblast,
    Rada,
}

/// Represents a unit in an organizational hierarchy.
///
/// A `Unit` can have a parent unit and multiple child units, forming a tree structure.
/// Each unit has a name, an ID, and optional supplementary information such as a supplementary name,
/// a type, and a number.
#[derive(Clone, Debug, Default)]
pub struct Unit {
    /// The name of the unit.
    name: String,
    /// The unique identifier of the unit.
    id: u32,
    /// The parent unit of this unit, if any.
    parent_unit: Option<Box<Unit>>,
    /// The child units of this unit.
    child_units: Vec<Unit>,
    /// The people belonging to this unit
    persons: Vec<Person>,
    /// An optional supplementary name for the unit.
    supplementary_name: Option<String>,
    /// The type of the unit, if specified.
    unit_type: Option<Type>,
    /// An optional number associated with the unit.
    number: Option<u32>,
}

impl Unit {
    /// Creates a new `Unit`.
    ///
    /// # Parameters
    /// - `name`: The name of the unit.
    /// - `id`: The unique identifier of the unit.
    /// - `parent_unit`: The parent unit of this unit, if any.
    /// - `supplementary_name`: An optional supplementary name for the unit.
    /// - `unit_type`: The type of the unit, if specified.
    /// - `number`: An optional number associated with the unit.
    ///
    /// # Returns
    /// A new `Unit` instance.
    #[must_use]
    pub fn new(
        name: &str,
        id: u32,
        parent_unit: Option<Box<Unit>>,
        supplementary_name: Option<String>,
        unit_type: Option<Type>,
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
            persons: Vec::new(),
        }
    }

    /// Returns a reference to the parent unit, if any.
    #[must_use]
    pub fn parent_unit(&self) -> &Option<Box<Unit>> {
        &self.parent_unit
    }

    /// Returns a reference to the child units
    #[must_use]
    pub fn child_units(&self) -> &Vec<Unit> {
        &self.child_units
    }

    /// Returns a mutable reference to the child units.
    #[must_use]
    pub fn child_units_mut(&mut self) -> &mut Vec<Unit> {
        &mut self.child_units
    }

    /// Consumes the `Unit` and returns its child units.
    #[must_use]
    pub fn into_child_units(self) -> Vec<Unit> {
        self.child_units
    }

    /// Returns a reference to the supplementary name, if any.
    #[must_use]
    pub fn supplementary_name(&self) -> &Option<String> {
        &self.supplementary_name
    }

    /// Returns a reference to the unit type, if specified.
    #[must_use]
    pub fn unit_type(&self) -> &Option<Type> {
        &self.unit_type
    }

    /// Returns the number, if any.
    #[must_use]
    pub fn number(&self) -> Option<u32> {
        self.number
    }

    /// Adds a child unit to this unit.
    ///
    /// # Parameters
    /// - `unit`: The child unit to add.
    pub fn add_child_unit(&mut self, unit: Unit) {
        self.child_units.push(unit);
    }

    #[must_use]
    pub fn persons(&self) -> &Vec<Person> {
        &self.persons
    }

    #[must_use]
    pub fn persons_mut(&mut self) -> &mut Vec<Person> {
        &mut self.persons
    }

    #[must_use]
    pub fn into_persons(self) -> Vec<Person> {
        self.persons
    }

    pub fn add_person(&mut self, person: Person) {
        self.persons.push(person);
    }
}

impl Object for Unit {
    type Builder = UnitBuilder;

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
