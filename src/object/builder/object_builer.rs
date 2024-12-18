use crate::object::Object;

pub trait ObjectBuilder<T: Object<Self>>: Sized + Default {
    fn name(&mut self, name: &str) -> &mut Self;
    fn id(&mut self, id: u32) -> &mut Self;
    fn build(self) -> anyhow::Result<T>;
}
