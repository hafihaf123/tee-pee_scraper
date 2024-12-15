pub enum UnitType {
    Druzina,
    Oddiel,
    Zbor,
    Oblast,
    Rada,
}

#[derive(Default)]
pub struct Unit {
    name: String,
    parent_unit: Option<Box<Unit>>,
    child_units: Vec<Box<Unit>>,
    supplementary_name: Option<String>,
    unit_type: Option<UnitType>,
    number: Option<u32>,
    id: Option<u32>,
}

impl Unit {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
    
    pub fn builder(name: String) -> UnitBuilder {
        UnitBuilder::new(name)
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn parent_unit(&self) -> &Option<Box<Unit>> {
        &self.parent_unit
    }
    
    pub fn child_units(&self) -> &Vec<Box<Unit>> {
        &self.child_units
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
    
    pub fn id(&self) -> &Option<u32> {
        &self.id
    }
    
    pub fn add_child_unit(&mut self, unit: Box<Unit>) {
        let _ = &self.child_units.push(unit);
    }
}

#[derive(Default)]
pub struct UnitBuilder {
    name: String,
    parent_unit: Option<Box<Unit>>,
    supplementary_name: Option<String>,
    unit_type: Option<UnitType>,
    number: Option<u32>,
    id: Option<u32>,
}

impl UnitBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    pub fn parent_unit(mut self, parent_unit: Option<Box<Unit>>) -> Self {
        self.parent_unit = parent_unit;
        self
    }
    
    pub fn supplementary_name(mut self, supplementary_name: Option<String>) -> Self {
        self.supplementary_name = supplementary_name;
        self
    }
    
    pub fn unit_type(mut self, unit_type: Option<UnitType>) -> Self {
        self.unit_type = unit_type;
        self
    }
    
    pub fn number(mut self, number: Option<u32>) -> Self {
        self.number = number;
        self
    }
    
    pub fn id(mut self, id: Option<u32>) -> Self {
        self.id = id;
        self
    }
    
    pub fn build(self) -> Unit {
        Unit {
            name: self.name,
            parent_unit: self.parent_unit,
            supplementary_name: self.supplementary_name,
            unit_type: self.unit_type,
            number: self.number,
            id: self.id,
            ..Default::default()
        }
    }
}
