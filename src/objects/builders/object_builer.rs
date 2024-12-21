use crate::Object;

pub trait ObjectBuilder: Default {
    type Object: Object<Builder = Self>;

    fn name(&mut self, name: &str) -> &mut Self;
    fn id(&mut self, id: u32) -> &mut Self;
    fn build(self) -> anyhow::Result<Self::Object>;
}
