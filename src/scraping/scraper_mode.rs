use crate::objects::builders::ObjectBuilder;
use crate::objects::Object;

pub trait ScraperMode<T: Object<B>, B: ObjectBuilder<T>> {}
