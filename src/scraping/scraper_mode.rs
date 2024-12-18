use crate::object::builder::ObjectBuilder;
use crate::object::Object;

pub trait ScraperMode<T: Object<B>, B: ObjectBuilder<T>> {}
