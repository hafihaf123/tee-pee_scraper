use crate::objects::builders::ObjectBuilder;
use crate::Object;

pub trait ScraperMode<T: Object<B>, B: ObjectBuilder<T>> {}
