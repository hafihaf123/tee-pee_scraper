use crate::objects::builders::ObjectBuilder;

pub trait Object {
    type Builder: ObjectBuilder<Object = Self>;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
    fn name(&self) -> &str;
    fn id(&self) -> u32;
}
