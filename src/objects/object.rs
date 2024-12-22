use crate::objects::builders::ObjectBuilder;

/// The `Object` trait defines the common interface for all objects.
///
/// # Associated Types
/// - `Builder`: A type that implements the `ObjectBuilder` trait, used to construct instances of the object.
///
/// # Required Methods
/// - `builder() -> Self::Builder`: Returns a default builder for the object.
/// - `name(&self) -> &str`: Returns the name of the object.
/// - `id(&self) -> u32`: Returns the unique identifier of the object.
pub trait Object {
    type Builder: ObjectBuilder<Object = Self>;

    /// Returns a default builder for the object.
    ///
    /// # Returns
    /// A builder of type `Self::Builder` that can be used to construct instances of the object.
    #[must_use]
    fn builder() -> Self::Builder {
        Self::Builder::default()
    }

    /// Returns the name of the object.
    ///
    /// # Returns
    /// A string slice that holds the name of the object.
    #[must_use]
    fn name(&self) -> &str;

    /// Returns the unique identifier of the object.
    ///
    /// # Returns
    /// A 32-bit unsigned integer representing the unique identifier of the object.
    #[must_use]
    fn id(&self) -> u32;
}
