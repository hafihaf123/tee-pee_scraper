use crate::Object;

/// A trait for building objects that implement the `Object` trait.
///
/// Types implementing this trait should also implement the `Default` trait.
pub trait ObjectBuilder: Default {
    /// The type of object being built, which must implement the `Object` trait.
    type Object: Object<Builder = Self>;

    /// Sets the name of the object being built.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the object.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    fn name(&mut self, name: &str) -> &mut Self;

    /// Sets the ID of the object being built.
    ///
    /// # Arguments
    ///
    /// * `id` - A 32-bit unsigned integer that holds the ID of the object.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    fn id(&mut self, id: u32) -> &mut Self;

    /// Builds the object.
    ///
    /// # Returns
    ///
    /// A result containing the built object or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the object could not be built.
    fn build(self) -> anyhow::Result<Self::Object>;
}
