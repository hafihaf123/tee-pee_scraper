use crate::object::builder::ObjectBuilder;

pub trait Object<T: ObjectBuilder<Self> + Default + ?Sized>: Sized {
    fn builder() -> T {
        T::default()
    }
    fn name(&self) -> &str;
    fn id(&self) -> u32;
}
